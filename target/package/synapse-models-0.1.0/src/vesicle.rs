//! Vesicle pool dynamics and neurotransmitter release mechanisms.
//!
//! This module implements the dynamics of synaptic vesicle pools including:
//! - Ready Releasable Pool (RRP)
//! - Reserve pool
//! - Recycling pool
//! - Calcium-dependent release
//! - Short-term depression and facilitation

use crate::error::{Result, SynapseError};

/// Vesicle pool dynamics implementing the Tsodyks-Markram model.
///
/// The model tracks three pools of vesicles:
/// 1. Ready Releasable Pool (RRP) - immediately available for release
/// 2. Reserve pool - can be recruited to RRP
/// 3. Released/recycling pool - recovering from release
///
/// Key equations:
/// - dx/dt = (1 - x)/τ_rec - U*x*δ(t_spike)  (recovery from depression)
/// - du/dt = (U₀ - u)/τ_facil + U₀(1-u)δ(t_spike)  (facilitation)
#[derive(Debug, Clone)]
pub struct VesiclePool {
    /// Fraction of available vesicles (0 to 1).
    pub available_fraction: f64,

    /// Utilization parameter (effective release probability, 0 to 1).
    pub utilization: f64,

    /// Baseline utilization (U₀).
    pub baseline_utilization: f64,

    /// Recovery time constant from depression (ms).
    pub tau_recovery: f64,

    /// Facilitation time constant (ms).
    pub tau_facilitation: f64,

    /// Total number of vesicles in readily releasable pool.
    pub total_vesicles: usize,

    /// Number of docked vesicles ready for release.
    pub docked_vesicles: usize,

    /// Reserve pool size.
    pub reserve_pool: usize,

    /// Recycling pool size.
    pub recycling_pool: usize,

    /// Calcium sensitivity exponent.
    pub calcium_cooperativity: f64,

    /// Half-maximal calcium concentration for release (μM).
    pub calcium_half_max: f64,
}

impl Default for VesiclePool {
    fn default() -> Self {
        Self {
            available_fraction: 1.0,
            utilization: 0.5,
            baseline_utilization: 0.5,
            tau_recovery: 800.0,      // 800 ms recovery
            tau_facilitation: 1000.0, // 1000 ms facilitation decay
            total_vesicles: 100,
            docked_vesicles: 10,
            reserve_pool: 80,
            recycling_pool: 0,
            calcium_cooperativity: 4.0, // Hill coefficient
            calcium_half_max: 1.0,      // 1 μM
        }
    }
}

impl VesiclePool {
    /// Create a new vesicle pool with default parameters.
    pub fn new() -> Self {
        Self::default()
    }

    /// Create a vesicle pool with custom parameters.
    ///
    /// # Arguments
    /// * `baseline_u` - Baseline utilization (0 to 1)
    /// * `tau_rec` - Recovery time constant (ms)
    /// * `tau_facil` - Facilitation time constant (ms)
    pub fn with_params(baseline_u: f64, tau_rec: f64, tau_facil: f64) -> Result<Self> {
        if !(0.0..=1.0).contains(&baseline_u) {
            return Err(SynapseError::InvalidProbability(baseline_u));
        }
        if tau_rec <= 0.0 {
            return Err(SynapseError::InvalidTimeConstant(tau_rec));
        }
        if tau_facil <= 0.0 {
            return Err(SynapseError::InvalidTimeConstant(tau_facil));
        }

        Ok(Self {
            baseline_utilization: baseline_u,
            utilization: baseline_u,
            tau_recovery: tau_rec,
            tau_facilitation: tau_facil,
            ..Self::default()
        })
    }

    /// Create a depressing synapse (high U, fast recovery).
    ///
    /// Typical for cortical excitatory synapses.
    pub fn depressing() -> Self {
        Self {
            baseline_utilization: 0.6,
            utilization: 0.6,
            tau_recovery: 130.0,
            tau_facilitation: 530.0,
            ..Self::default()
        }
    }

    /// Create a facilitating synapse (low U, slow recovery).
    ///
    /// Typical for some cortical and hippocampal synapses.
    pub fn facilitating() -> Self {
        Self {
            baseline_utilization: 0.15,
            utilization: 0.15,
            tau_recovery: 670.0,
            tau_facilitation: 17.0,
            ..Self::default()
        }
    }

    /// Update vesicle pool dynamics over time.
    ///
    /// Implements continuous recovery and facilitation decay.
    ///
    /// # Arguments
    /// * `dt` - Time step (ms)
    pub fn update(&mut self, dt: f64) -> Result<()> {
        // Recovery from depression: dx/dt = (1 - x)/τ_rec
        let dx = (1.0 - self.available_fraction) / self.tau_recovery;
        self.available_fraction += dx * dt;
        self.available_fraction = self.available_fraction.clamp(0.0, 1.0);

        // Facilitation decay: du/dt = (U₀ - u)/τ_facil
        let du = (self.baseline_utilization - self.utilization) / self.tau_facilitation;
        self.utilization += du * dt;
        self.utilization = self.utilization.clamp(0.0, 1.0);

        // Update vesicle pool sizes
        let total = self.total_vesicles as f64;
        self.docked_vesicles = (self.available_fraction * total * 0.1) as usize;
        self.reserve_pool = ((1.0 - self.available_fraction) * total * 0.8) as usize;
        self.recycling_pool = self.total_vesicles - self.docked_vesicles - self.reserve_pool;

        Ok(())
    }

    /// Calculate release probability given calcium concentration.
    ///
    /// Uses Hill equation: P = [Ca]^n / ([Ca]^n + K^n)
    ///
    /// # Arguments
    /// * `calcium_concentration` - Presynaptic calcium concentration (μM)
    pub fn calcium_release_probability(&self, calcium_concentration: f64) -> f64 {
        let ca_n = calcium_concentration.powf(self.calcium_cooperativity);
        let k_n = self.calcium_half_max.powf(self.calcium_cooperativity);
        ca_n / (ca_n + k_n)
    }

    /// Trigger vesicle release (spike arrives).
    ///
    /// Updates both depression and facilitation according to Tsodyks-Markram model.
    ///
    /// # Arguments
    /// * `calcium_concentration` - Presynaptic calcium concentration (μM)
    ///
    /// # Returns
    /// Number of vesicles released
    pub fn release(&mut self, calcium_concentration: f64) -> Result<usize> {
        if calcium_concentration < 0.0 {
            return Err(SynapseError::InvalidConcentration(calcium_concentration));
        }

        // Calculate calcium-dependent release probability
        let ca_prob = self.calcium_release_probability(calcium_concentration);

        // Effective release probability combines utilization and calcium
        let release_prob = self.utilization * ca_prob;

        // Number of vesicles released
        let vesicles_released = (self.available_fraction * release_prob * self.total_vesicles as f64) as usize;

        // Update depression: x → x - U*x
        self.available_fraction *= 1.0 - self.utilization;
        self.available_fraction = self.available_fraction.clamp(0.0, 1.0);

        // Update facilitation: u → u + U₀(1-u)
        self.utilization += self.baseline_utilization * (1.0 - self.utilization);
        self.utilization = self.utilization.clamp(0.0, 1.0);

        // Update pool counts
        if vesicles_released <= self.docked_vesicles {
            self.docked_vesicles -= vesicles_released;
            self.recycling_pool += vesicles_released;
        } else {
            self.recycling_pool += self.docked_vesicles;
            self.docked_vesicles = 0;
        }

        Ok(vesicles_released)
    }

    /// Get current release probability (without calcium dependence).
    pub fn release_probability(&self) -> f64 {
        self.available_fraction * self.utilization
    }

    /// Reset vesicle pool to initial state.
    pub fn reset(&mut self) {
        self.available_fraction = 1.0;
        self.utilization = self.baseline_utilization;
        self.docked_vesicles = (self.total_vesicles as f64 * 0.1) as usize;
        self.reserve_pool = (self.total_vesicles as f64 * 0.8) as usize;
        self.recycling_pool = 0;
    }
}

/// Quantal release model for vesicle fusion.
///
/// Models individual vesicle release events with binomial statistics.
#[derive(Debug, Clone)]
pub struct QuantalRelease {
    /// Number of release sites (N).
    pub n_sites: usize,

    /// Release probability per site (p).
    pub release_probability: f64,

    /// Quantal size (postsynaptic response per vesicle, pA or mV).
    pub quantal_size: f64,

    /// Variance in quantal size.
    pub quantal_variance: f64,
}

impl QuantalRelease {
    /// Create a new quantal release model.
    pub fn new(n_sites: usize, release_probability: f64, quantal_size: f64) -> Result<Self> {
        if !(0.0..=1.0).contains(&release_probability) {
            return Err(SynapseError::InvalidProbability(release_probability));
        }

        Ok(Self {
            n_sites,
            release_probability,
            quantal_size,
            quantal_variance: quantal_size * 0.3, // 30% CV
        })
    }

    /// Calculate expected number of vesicles released.
    pub fn expected_release(&self) -> f64 {
        self.n_sites as f64 * self.release_probability
    }

    /// Calculate variance in number of vesicles released (binomial).
    pub fn release_variance(&self) -> f64 {
        self.n_sites as f64 * self.release_probability * (1.0 - self.release_probability)
    }

    /// Calculate expected postsynaptic response amplitude.
    pub fn expected_amplitude(&self) -> f64 {
        self.expected_release() * self.quantal_size
    }

    /// Calculate coefficient of variation (CV) of response.
    pub fn coefficient_of_variation(&self) -> f64 {
        if self.expected_release() == 0.0 {
            return f64::INFINITY;
        }

        let release_cv = (self.release_variance() / self.expected_release().powi(2)).sqrt();
        let quantal_cv = self.quantal_variance / self.quantal_size;

        (release_cv.powi(2) + quantal_cv.powi(2)).sqrt()
    }
}

/// Multi-vesicular release (MVR) model.
///
/// Models the probability of releasing multiple vesicles from a single release site.
#[derive(Debug, Clone)]
pub struct MultiVesicularRelease {
    /// Probability of releasing 0, 1, 2, ... vesicles per site.
    pub release_probabilities: Vec<f64>,

    /// Maximum number of vesicles per site.
    pub max_vesicles_per_site: usize,
}

impl MultiVesicularRelease {
    /// Create MVR model with Poisson distribution.
    ///
    /// # Arguments
    /// * `mean_vesicles` - Mean number of vesicles released per site
    /// * `max_vesicles` - Maximum vesicles to consider per site
    pub fn poisson(mean_vesicles: f64, max_vesicles: usize) -> Self {
        let mut probs = Vec::with_capacity(max_vesicles + 1);

        // Calculate Poisson probabilities
        for k in 0..=max_vesicles {
            let p = (mean_vesicles.powi(k as i32) * (-mean_vesicles).exp())
                    / Self::factorial(k) as f64;
            probs.push(p);
        }

        // Normalize
        let sum: f64 = probs.iter().sum();
        probs.iter_mut().for_each(|p| *p /= sum);

        Self {
            release_probabilities: probs,
            max_vesicles_per_site: max_vesicles,
        }
    }

    /// Calculate factorial.
    fn factorial(n: usize) -> usize {
        (1..=n).product()
    }

    /// Get mean number of vesicles released per site.
    pub fn mean_release(&self) -> f64 {
        self.release_probabilities
            .iter()
            .enumerate()
            .map(|(k, &p)| k as f64 * p)
            .sum()
    }

    /// Get probability of releasing exactly k vesicles.
    pub fn probability(&self, k: usize) -> f64 {
        if k <= self.max_vesicles_per_site {
            self.release_probabilities[k]
        } else {
            0.0
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vesicle_pool_creation() {
        let pool = VesiclePool::new();
        assert_eq!(pool.available_fraction, 1.0);
        assert_eq!(pool.utilization, 0.5);
    }

    #[test]
    fn test_depressing_synapse() {
        let mut pool = VesiclePool::depressing();
        let initial_prob = pool.release_probability();

        // First release
        pool.release(10.0).unwrap(); // High calcium
        let prob_after_first = pool.release_probability();

        // Depression: probability should decrease
        assert!(prob_after_first < initial_prob);

        // Second release immediately after
        pool.release(10.0).unwrap();
        let prob_after_second = pool.release_probability();

        // Further depression
        assert!(prob_after_second < prob_after_first);
    }

    #[test]
    fn test_facilitating_synapse() {
        let mut pool = VesiclePool::facilitating();
        let initial_u = pool.utilization;

        // Release causes facilitation
        pool.release(10.0).unwrap();

        // Utilization should increase
        assert!(pool.utilization > initial_u);
    }

    #[test]
    fn test_vesicle_pool_recovery() {
        let mut pool = VesiclePool::new();

        // Deplete pool
        pool.release(10.0).unwrap();
        let depleted_fraction = pool.available_fraction;

        // Recovery over time
        for _ in 0..100 {
            pool.update(10.0).unwrap(); // 10 ms steps
        }

        assert!(pool.available_fraction > depleted_fraction);
    }

    #[test]
    fn test_calcium_release_probability() {
        let pool = VesiclePool::new();

        let low_ca = pool.calcium_release_probability(0.1);
        let medium_ca = pool.calcium_release_probability(1.0);
        let high_ca = pool.calcium_release_probability(10.0);

        assert!(low_ca < medium_ca);
        assert!(medium_ca < high_ca);
        assert!(high_ca > 0.9); // Should be near saturation
    }

    #[test]
    fn test_quantal_release() {
        let qr = QuantalRelease::new(10, 0.5, 20.0).unwrap();

        assert_eq!(qr.expected_release(), 5.0); // 10 * 0.5
        assert_eq!(qr.expected_amplitude(), 100.0); // 5 * 20
        assert!(qr.coefficient_of_variation() > 0.0);
    }

    #[test]
    fn test_multi_vesicular_release() {
        let mvr = MultiVesicularRelease::poisson(2.0, 10);

        // Probabilities should sum to ~1
        let sum: f64 = mvr.release_probabilities.iter().sum();
        assert!((sum - 1.0).abs() < 1e-6);

        // Mean should be close to 2.0
        assert!((mvr.mean_release() - 2.0).abs() < 0.1);
    }

    #[test]
    fn test_vesicle_pool_reset() {
        let mut pool = VesiclePool::new();
        pool.release(10.0).unwrap();

        pool.reset();
        assert_eq!(pool.available_fraction, 1.0);
        assert_eq!(pool.utilization, pool.baseline_utilization);
    }
}
