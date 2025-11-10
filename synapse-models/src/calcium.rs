//! Calcium dynamics in pre- and postsynaptic compartments.
//!
//! This module implements calcium dynamics including:
//! - Influx through voltage-gated channels
//! - Buffering by calcium-binding proteins
//! - Extrusion by pumps and exchangers
//! - Calcium stores (endoplasmic reticulum)
//! - Calcium-induced calcium release (CICR)

use crate::error::{Result, SynapseError};

/// Calcium dynamics in a cellular compartment.
///
/// Models calcium concentration changes due to:
/// - Influx (voltage-gated channels, NMDA receptors)
/// - Buffering (calmodulin, calbindin, etc.)
/// - Extrusion (PMCA, NCX)
/// - Diffusion
/// - ER uptake and release
#[derive(Debug, Clone)]
pub struct CalciumDynamics {
    /// Free calcium concentration (μM).
    pub concentration: f64,

    /// Buffered calcium concentration (μM).
    pub buffered: f64,

    /// Resting calcium concentration (μM).
    pub resting_concentration: f64,

    /// Time constant for calcium removal (ms).
    pub tau_removal: f64,

    /// Calcium influx per spike (μM).
    pub spike_influx: f64,

    /// Buffer capacity (ratio of buffered to free calcium).
    pub buffer_capacity: f64,

    /// Buffer binding rate (1/(μM·ms)).
    pub buffer_kon: f64,

    /// Buffer unbinding rate (1/ms).
    pub buffer_koff: f64,

    /// Total buffer concentration (μM).
    pub buffer_total: f64,

    /// Compartment volume (μm³).
    pub volume: f64,
}

impl Default for CalciumDynamics {
    fn default() -> Self {
        Self {
            concentration: 0.05,      // 50 nM resting
            buffered: 0.0,
            resting_concentration: 0.05,
            tau_removal: 20.0,        // 20 ms removal
            spike_influx: 0.5,        // 0.5 μM per spike
            buffer_capacity: 100.0,   // High buffering
            buffer_kon: 0.1,          // Fast binding
            buffer_koff: 0.001,       // Slow unbinding
            buffer_total: 100.0,      // 100 μM total buffer
            volume: 1.0,              // Normalized volume
        }
    }
}

impl CalciumDynamics {
    /// Create new calcium dynamics with default parameters.
    pub fn new() -> Self {
        Self::default()
    }

    /// Create calcium dynamics for presynaptic terminal.
    ///
    /// Presynaptic terminals have rapid calcium dynamics with high buffering.
    pub fn presynaptic() -> Self {
        Self {
            resting_concentration: 0.05,
            tau_removal: 15.0,     // Fast removal
            spike_influx: 1.0,     // Large influx
            buffer_capacity: 200.0, // Very high buffering
            ..Self::default()
        }
    }

    /// Create calcium dynamics for postsynaptic spine.
    ///
    /// Spines have intermediate calcium dynamics.
    pub fn postsynaptic() -> Self {
        Self {
            resting_concentration: 0.05,
            tau_removal: 30.0,     // Moderate removal
            spike_influx: 0.3,     // Moderate influx
            buffer_capacity: 50.0, // Moderate buffering
            ..Self::default()
        }
    }

    /// Update calcium concentration over time.
    ///
    /// Implements first-order removal and buffering dynamics.
    ///
    /// # Arguments
    /// * `influx` - Calcium influx rate (μM/ms)
    /// * `dt` - Time step (ms)
    pub fn update(&mut self, influx: f64, dt: f64) -> Result<()> {
        if influx < 0.0 {
            return Err(SynapseError::InvalidConcentration(influx));
        }

        // Free buffer concentration
        let buffer_free = self.buffer_total - self.buffered;

        // Buffering dynamics
        // dB/dt = k_on * [Ca] * [B_free] - k_off * [B_bound]
        let d_buffered = self.buffer_kon * self.concentration * buffer_free
                         - self.buffer_koff * self.buffered;
        self.buffered += d_buffered * dt;
        self.buffered = self.buffered.clamp(0.0, self.buffer_total);

        // Calcium dynamics with buffering
        // d[Ca]/dt = influx - (([Ca] - [Ca]_rest) / τ) - dB/dt
        let removal = (self.concentration - self.resting_concentration) / self.tau_removal;
        let d_concentration = influx - removal - d_buffered;

        self.concentration += d_concentration * dt;
        self.concentration = self.concentration.max(0.0);

        Ok(())
    }

    /// Add calcium influx from spike.
    pub fn spike(&mut self) {
        self.concentration += self.spike_influx;
    }

    /// Add calcium influx (e.g., from NMDA receptors).
    ///
    /// # Arguments
    /// * `amount` - Amount of calcium to add (μM)
    pub fn add_influx(&mut self, amount: f64) -> Result<()> {
        if amount < 0.0 {
            return Err(SynapseError::InvalidConcentration(amount));
        }
        self.concentration += amount;
        Ok(())
    }

    /// Get current free calcium concentration.
    pub fn get_concentration(&self) -> f64 {
        self.concentration
    }

    /// Reset to resting state.
    pub fn reset(&mut self) {
        self.concentration = self.resting_concentration;
        self.buffered = 0.0;
    }
}

/// Calcium store (endoplasmic reticulum) dynamics.
///
/// Models calcium release from and uptake into the ER.
#[derive(Debug, Clone)]
pub struct CalciumStore {
    /// ER calcium concentration (μM).
    pub store_concentration: f64,

    /// Cytoplasmic calcium concentration (μM).
    pub cytoplasmic_concentration: f64,

    /// Maximum store concentration (μM).
    pub max_store_concentration: f64,

    /// SERCA pump rate (μM/ms).
    pub pump_rate: f64,

    /// SERCA pump affinity (μM).
    pub pump_km: f64,

    /// IP3 receptor open probability.
    pub ip3r_open_probability: f64,

    /// RyR (ryanodine receptor) open probability.
    pub ryr_open_probability: f64,

    /// Maximum release rate (μM/ms).
    pub max_release_rate: f64,

    /// IP3 concentration (μM).
    pub ip3_concentration: f64,

    /// Calcium threshold for CICR (μM).
    pub cicr_threshold: f64,
}

impl Default for CalciumStore {
    fn default() -> Self {
        Self {
            store_concentration: 400.0, // ~400 μM in ER
            cytoplasmic_concentration: 0.05,
            max_store_concentration: 500.0,
            pump_rate: 0.5,
            pump_km: 0.3,
            ip3r_open_probability: 0.0,
            ryr_open_probability: 0.0,
            max_release_rate: 10.0,
            ip3_concentration: 0.0,
            cicr_threshold: 0.3, // 0.3 μM threshold
        }
    }
}

impl CalciumStore {
    /// Create new calcium store with default parameters.
    pub fn new() -> Self {
        Self::default()
    }

    /// Update calcium store dynamics.
    ///
    /// # Arguments
    /// * `cytoplasmic_ca` - Current cytoplasmic calcium (μM)
    /// * `dt` - Time step (ms)
    ///
    /// # Returns
    /// Net calcium release from store (positive = release, negative = uptake)
    pub fn update(&mut self, cytoplasmic_ca: f64, dt: f64) -> Result<f64> {
        self.cytoplasmic_concentration = cytoplasmic_ca;

        // SERCA pump (uptake into ER)
        let pump_flux = self.pump_rate * cytoplasmic_ca / (cytoplasmic_ca + self.pump_km);

        // IP3 receptor release
        let ip3r_flux = self.ip3r_open_probability * self.max_release_rate
                        * (self.store_concentration / self.max_store_concentration);

        // Ryanodine receptor (CICR)
        self.update_ryr_probability();
        let ryr_flux = self.ryr_open_probability * self.max_release_rate
                       * (self.store_concentration / self.max_store_concentration);

        // Net flux (positive = release from ER)
        let net_flux = ip3r_flux + ryr_flux - pump_flux;

        // Update store concentration
        self.store_concentration -= net_flux * dt;
        self.store_concentration = self.store_concentration.clamp(0.0, self.max_store_concentration);

        Ok(net_flux)
    }

    /// Update ryanodine receptor open probability for CICR.
    ///
    /// RyR opens when cytoplasmic calcium exceeds threshold.
    fn update_ryr_probability(&mut self) {
        // Simplified Hill equation for RyR activation
        let ca = self.cytoplasmic_concentration;
        if ca > self.cicr_threshold {
            let hill_coeff = 4.0;
            let k_half = 0.5_f64; // μM
            self.ryr_open_probability = ca.powf(hill_coeff) / (ca.powf(hill_coeff) + k_half.powf(hill_coeff));
        } else {
            self.ryr_open_probability = 0.0;
        }
    }

    /// Trigger IP3-mediated calcium release.
    ///
    /// # Arguments
    /// * `ip3_level` - IP3 concentration (μM)
    pub fn trigger_ip3_release(&mut self, ip3_level: f64) {
        self.ip3_concentration = ip3_level;

        // IP3R activation depends on both IP3 and calcium
        let ip3_term = ip3_level / (ip3_level + 0.5); // IP3 binding
        let ca_term = self.cytoplasmic_concentration / (self.cytoplasmic_concentration + 0.3); // Ca binding

        self.ip3r_open_probability = ip3_term * ca_term * 0.8; // Max 80% open
    }

    /// Check if CICR is active.
    pub fn is_cicr_active(&self) -> bool {
        self.cytoplasmic_concentration > self.cicr_threshold && self.ryr_open_probability > 0.01
    }

    /// Reset store to resting state.
    pub fn reset(&mut self) {
        self.store_concentration = 400.0;
        self.cytoplasmic_concentration = 0.05;
        self.ip3r_open_probability = 0.0;
        self.ryr_open_probability = 0.0;
        self.ip3_concentration = 0.0;
    }
}

/// Calcium-dependent processes.
///
/// Helper functions for calcium-dependent synaptic processes.
pub struct CalciumDependent;

impl CalciumDependent {
    /// Calculate calcium-dependent plasticity signal.
    ///
    /// Used for CaMKII activation, calcineurin activation, etc.
    ///
    /// # Arguments
    /// * `calcium` - Calcium concentration (μM)
    /// * `threshold_low` - Low threshold for depression (μM)
    /// * `threshold_high` - High threshold for potentiation (μM)
    pub fn plasticity_signal(calcium: f64, threshold_low: f64, threshold_high: f64) -> f64 {
        if calcium < threshold_low {
            0.0
        } else if calcium < threshold_high {
            // Depression range
            -(calcium - threshold_low) / (threshold_high - threshold_low)
        } else {
            // Potentiation range
            (calcium - threshold_high) / threshold_high
        }
    }

    /// Calculate CaMKII activation.
    ///
    /// CaMKII is activated by calcium-calmodulin and drives LTP.
    ///
    /// # Arguments
    /// * `calcium` - Calcium concentration (μM)
    pub fn camkii_activation(calcium: f64) -> f64 {
        let k_half = 0.7_f64; // μM
        let hill = 3.0;
        calcium.powf(hill) / (calcium.powf(hill) + k_half.powf(hill))
    }

    /// Calculate calcineurin activation.
    ///
    /// Calcineurin is activated by moderate calcium and drives LTD.
    ///
    /// # Arguments
    /// * `calcium` - Calcium concentration (μM)
    pub fn calcineurin_activation(calcium: f64) -> f64 {
        let k_half = 0.3_f64; // μM (lower threshold than CaMKII)
        let hill = 2.0;
        calcium.powf(hill) / (calcium.powf(hill) + k_half.powf(hill))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calcium_dynamics_creation() {
        let ca = CalciumDynamics::new();
        assert_eq!(ca.concentration, 0.05);
        assert_eq!(ca.resting_concentration, 0.05);
    }

    #[test]
    fn test_calcium_spike() {
        let mut ca = CalciumDynamics::new();
        let initial = ca.concentration;

        ca.spike();
        assert!(ca.concentration > initial);
    }

    #[test]
    fn test_calcium_decay() {
        let mut ca = CalciumDynamics::new();
        ca.concentration = 5.0; // Elevated calcium

        // Let it decay
        for _ in 0..100 {
            ca.update(0.0, 1.0).unwrap();
        }

        // Should return toward resting
        assert!(ca.concentration < 5.0);
        assert!(ca.concentration > ca.resting_concentration);
    }

    #[test]
    fn test_calcium_buffering() {
        let mut ca = CalciumDynamics::new();
        ca.concentration = 1.0;

        // Update to allow buffering
        for _ in 0..50 {
            ca.update(0.0, 0.1).unwrap();
        }

        // Some calcium should be buffered
        assert!(ca.buffered > 0.0);
    }

    #[test]
    fn test_presynaptic_calcium() {
        let ca_pre = CalciumDynamics::presynaptic();
        assert!(ca_pre.spike_influx > 0.5);
        assert!(ca_pre.buffer_capacity > 100.0);
    }

    #[test]
    fn test_postsynaptic_calcium() {
        let ca_post = CalciumDynamics::postsynaptic();
        assert!(ca_post.spike_influx < 0.5);
    }

    #[test]
    fn test_calcium_store() {
        let store = CalciumStore::new();
        assert!(store.store_concentration > 0.0);
        assert_eq!(store.cytoplasmic_concentration, 0.05);
    }

    #[test]
    fn test_cicr_activation() {
        let mut store = CalciumStore::new();

        // Low calcium - no CICR
        store.update(0.1, 1.0).unwrap();
        assert!(!store.is_cicr_active());

        // High calcium - triggers CICR
        store.update(1.0, 1.0).unwrap();
        assert!(store.ryr_open_probability > 0.0);
    }

    #[test]
    fn test_ip3_release() {
        let mut store = CalciumStore::new();
        store.trigger_ip3_release(1.0);

        assert!(store.ip3_concentration > 0.0);
        assert!(store.ip3r_open_probability > 0.0);
    }

    #[test]
    fn test_camkii_activation() {
        let low_ca = CalciumDependent::camkii_activation(0.1);
        let medium_ca = CalciumDependent::camkii_activation(0.7);
        let high_ca = CalciumDependent::camkii_activation(2.0);

        assert!(low_ca < medium_ca);
        assert!(medium_ca < high_ca);
    }

    #[test]
    fn test_plasticity_signal() {
        let signal_low = CalciumDependent::plasticity_signal(0.2, 0.3, 0.8);
        let signal_medium = CalciumDependent::plasticity_signal(0.5, 0.3, 0.8);
        let signal_high = CalciumDependent::plasticity_signal(1.5, 0.3, 0.8);

        assert_eq!(signal_low, 0.0); // Below threshold
        assert!(signal_medium < 0.0); // Depression range
        assert!(signal_high > 0.0);   // Potentiation range
    }
}
