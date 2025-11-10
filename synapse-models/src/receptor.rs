//! Receptor dynamics and kinetic models.
//!
//! This module implements various postsynaptic receptor types with detailed
//! kinetic models based on experimental data.

use crate::error::{Result, SynapseError};

/// Trait for receptor dynamics.
pub trait ReceptorDynamics {
    /// Update receptor state given neurotransmitter concentration and membrane voltage.
    ///
    /// # Arguments
    /// * `nt_concentration` - Neurotransmitter concentration (mM)
    /// * `voltage` - Membrane voltage (mV)
    /// * `dt` - Time step (ms)
    fn update(&mut self, nt_concentration: f64, voltage: f64, dt: f64) -> Result<()>;

    /// Get the current open probability or conductance state.
    fn get_conductance(&self) -> f64;

    /// Get the reversal potential for this receptor (mV).
    fn reversal_potential(&self) -> f64;

    /// Reset receptor to resting state.
    fn reset(&mut self);
}

/// AMPA receptor - fast excitatory glutamate receptor.
///
/// AMPA receptors mediate the majority of fast excitatory synaptic transmission.
/// They have rapid kinetics with rise time ~0.2 ms and decay time ~2 ms.
///
/// Model: Two-state kinetic scheme
/// C <-> O (Closed <-> Open)
/// dr/dt = α[NT](1-r) - βr  (rise)
/// do/dt = r/τ_rise - o/τ_decay  (opening and decay)
#[derive(Debug, Clone)]
pub struct AMPAReceptor {
    /// Open probability (0 to 1).
    pub open_probability: f64,

    /// Rising phase variable (0 to 1).
    rise_state: f64,

    /// Rise time constant (ms).
    pub tau_rise: f64,

    /// Decay time constant (ms).
    pub tau_decay: f64,

    /// Forward binding rate (1/(mM·ms)).
    pub alpha: f64,

    /// Unbinding rate (1/ms).
    pub beta: f64,

    /// Reversal potential (mV).
    pub e_rev: f64,

    /// Maximum conductance (nS).
    pub g_max: f64,
}

impl Default for AMPAReceptor {
    fn default() -> Self {
        Self {
            open_probability: 0.0,
            rise_state: 0.0,
            tau_rise: 0.2,   // 0.2 ms rise time
            tau_decay: 2.0,  // 2 ms decay time
            alpha: 1.1,      // Fast binding
            beta: 0.19,      // Relatively slow unbinding
            e_rev: 0.0,      // Non-selective cation channel
            g_max: 1.0,      // Normalized conductance
        }
    }
}

impl AMPAReceptor {
    /// Create a new AMPA receptor with default parameters.
    pub fn new() -> Self {
        Self::default()
    }

    /// Create AMPA receptor with custom parameters.
    pub fn with_params(tau_rise: f64, tau_decay: f64, g_max: f64) -> Result<Self> {
        if tau_rise <= 0.0 {
            return Err(SynapseError::InvalidTimeConstant(tau_rise));
        }
        if tau_decay <= 0.0 {
            return Err(SynapseError::InvalidTimeConstant(tau_decay));
        }

        Ok(Self {
            tau_rise,
            tau_decay,
            g_max,
            ..Self::default()
        })
    }

    /// Get the synaptic current (pA).
    pub fn current(&self, voltage: f64) -> f64 {
        self.g_max * self.open_probability * (voltage - self.e_rev)
    }
}

impl ReceptorDynamics for AMPAReceptor {
    fn update(&mut self, nt_concentration: f64, _voltage: f64, dt: f64) -> Result<()> {
        // Update rise state: dr/dt = α[NT](1-r) - βr
        let dr = self.alpha * nt_concentration * (1.0 - self.rise_state)
                 - self.beta * self.rise_state;
        self.rise_state += dr * dt;
        self.rise_state = self.rise_state.clamp(0.0, 1.0);

        // Update open probability using exponential Euler for stability
        // do/dt = r/τ_rise - o/τ_decay
        let target = self.rise_state * self.tau_decay / (self.tau_rise + self.tau_decay);
        let tau_eff = self.tau_decay;
        self.open_probability += (target - self.open_probability) * (1.0 - (-dt / tau_eff).exp());
        self.open_probability = self.open_probability.clamp(0.0, 1.0);

        Ok(())
    }

    fn get_conductance(&self) -> f64 {
        self.g_max * self.open_probability
    }

    fn reversal_potential(&self) -> f64 {
        self.e_rev
    }

    fn reset(&mut self) {
        self.open_probability = 0.0;
        self.rise_state = 0.0;
    }
}

/// NMDA receptor - slow excitatory glutamate receptor with voltage dependence.
///
/// NMDA receptors have slower kinetics and are blocked by Mg2+ at resting potentials.
/// They are critical for synaptic plasticity and learning.
///
/// Key features:
/// - Voltage-dependent Mg2+ block
/// - Slow kinetics (rise ~2 ms, decay ~100 ms)
/// - High Ca2+ permeability
#[derive(Debug, Clone)]
pub struct NMDAReceptor {
    /// Open probability (0 to 1).
    pub open_probability: f64,

    /// Rising phase variable (0 to 1).
    rise_state: f64,

    /// Rise time constant (ms).
    pub tau_rise: f64,

    /// Decay time constant (ms).
    pub tau_decay: f64,

    /// Forward binding rate (1/(mM·ms)).
    pub alpha: f64,

    /// Unbinding rate (1/ms).
    pub beta: f64,

    /// Reversal potential (mV).
    pub e_rev: f64,

    /// Maximum conductance (nS).
    pub g_max: f64,

    /// Mg2+ concentration (mM).
    pub mg_concentration: f64,
}

impl Default for NMDAReceptor {
    fn default() -> Self {
        Self {
            open_probability: 0.0,
            rise_state: 0.0,
            tau_rise: 2.0,    // 2 ms rise time
            tau_decay: 100.0, // 100 ms decay time
            alpha: 0.72,      // Slower binding than AMPA
            beta: 0.0066,     // Very slow unbinding
            e_rev: 0.0,       // Non-selective cation channel
            g_max: 1.0,       // Normalized conductance
            mg_concentration: 1.0, // 1 mM external Mg2+
        }
    }
}

impl NMDAReceptor {
    /// Create a new NMDA receptor with default parameters.
    pub fn new() -> Self {
        Self::default()
    }

    /// Create NMDA receptor with custom parameters.
    pub fn with_params(tau_rise: f64, tau_decay: f64, g_max: f64) -> Result<Self> {
        if tau_rise <= 0.0 {
            return Err(SynapseError::InvalidTimeConstant(tau_rise));
        }
        if tau_decay <= 0.0 {
            return Err(SynapseError::InvalidTimeConstant(tau_decay));
        }

        Ok(Self {
            tau_rise,
            tau_decay,
            g_max,
            ..Self::default()
        })
    }

    /// Calculate voltage-dependent Mg2+ block.
    ///
    /// Year and Stevens (1990) model:
    /// B(V) = 1 / (1 + [Mg2+]/3.57 * exp(-0.062 * V))
    pub fn mg_block(&self, voltage: f64) -> f64 {
        1.0 / (1.0 + (self.mg_concentration / 3.57) * (-0.062 * voltage).exp())
    }

    /// Get the synaptic current (pA).
    pub fn current(&self, voltage: f64) -> f64 {
        let mg_block = self.mg_block(voltage);
        self.g_max * self.open_probability * mg_block * (voltage - self.e_rev)
    }
}

impl ReceptorDynamics for NMDAReceptor {
    fn update(&mut self, nt_concentration: f64, _voltage: f64, dt: f64) -> Result<()> {
        // Update rise state
        let dr = self.alpha * nt_concentration * (1.0 - self.rise_state)
                 - self.beta * self.rise_state;
        self.rise_state += dr * dt;
        self.rise_state = self.rise_state.clamp(0.0, 1.0);

        // Update open probability with exponential Euler
        let target = self.rise_state * self.tau_decay / (self.tau_rise + self.tau_decay);
        let tau_eff = self.tau_decay;
        self.open_probability += (target - self.open_probability) * (1.0 - (-dt / tau_eff).exp());
        self.open_probability = self.open_probability.clamp(0.0, 1.0);

        Ok(())
    }

    fn get_conductance(&self) -> f64 {
        self.g_max * self.open_probability
    }

    fn reversal_potential(&self) -> f64 {
        self.e_rev
    }

    fn reset(&mut self) {
        self.open_probability = 0.0;
        self.rise_state = 0.0;
    }
}

/// GABA-A receptor - fast inhibitory receptor.
///
/// GABA-A receptors are ionotropic chloride channels that mediate fast
/// inhibitory transmission with time constants similar to AMPA receptors.
#[derive(Debug, Clone)]
pub struct GABAAReceptor {
    /// Open probability (0 to 1).
    pub open_probability: f64,

    /// Rising phase variable (0 to 1).
    rise_state: f64,

    /// Rise time constant (ms).
    pub tau_rise: f64,

    /// Decay time constant (ms).
    pub tau_decay: f64,

    /// Forward binding rate (1/(mM·ms)).
    pub alpha: f64,

    /// Unbinding rate (1/ms).
    pub beta: f64,

    /// Reversal potential (mV) - depends on Cl- gradient.
    pub e_rev: f64,

    /// Maximum conductance (nS).
    pub g_max: f64,
}

impl Default for GABAAReceptor {
    fn default() -> Self {
        Self {
            open_probability: 0.0,
            rise_state: 0.0,
            tau_rise: 0.5,   // 0.5 ms rise time
            tau_decay: 5.0,  // 5 ms decay time
            alpha: 5.0,      // Fast binding
            beta: 0.18,      // Moderate unbinding
            e_rev: -70.0,    // Chloride reversal (can vary)
            g_max: 1.0,      // Normalized conductance
        }
    }
}

impl GABAAReceptor {
    /// Create a new GABA-A receptor with default parameters.
    pub fn new() -> Self {
        Self::default()
    }

    /// Get the synaptic current (pA).
    pub fn current(&self, voltage: f64) -> f64 {
        self.g_max * self.open_probability * (voltage - self.e_rev)
    }
}

impl ReceptorDynamics for GABAAReceptor {
    fn update(&mut self, nt_concentration: f64, _voltage: f64, dt: f64) -> Result<()> {
        // Update rise state
        let dr = self.alpha * nt_concentration * (1.0 - self.rise_state)
                 - self.beta * self.rise_state;
        self.rise_state += dr * dt;
        self.rise_state = self.rise_state.clamp(0.0, 1.0);

        // Update open probability
        let target = self.rise_state * self.tau_decay / (self.tau_rise + self.tau_decay);
        let tau_eff = self.tau_decay;
        self.open_probability += (target - self.open_probability) * (1.0 - (-dt / tau_eff).exp());
        self.open_probability = self.open_probability.clamp(0.0, 1.0);

        Ok(())
    }

    fn get_conductance(&self) -> f64 {
        self.g_max * self.open_probability
    }

    fn reversal_potential(&self) -> f64 {
        self.e_rev
    }

    fn reset(&mut self) {
        self.open_probability = 0.0;
        self.rise_state = 0.0;
    }
}

/// GABA-B receptor - slow inhibitory metabotropic receptor.
///
/// GABA-B receptors are G-protein coupled receptors that activate K+ channels,
/// producing slow, long-lasting inhibition.
#[derive(Debug, Clone)]
pub struct GABABReceptor {
    /// Receptor activation state (0 to 1).
    pub activation: f64,

    /// G-protein activation state (0 to 1).
    pub g_protein: f64,

    /// Rise time constant (ms).
    pub tau_rise: f64,

    /// Decay time constant (ms).
    pub tau_decay: f64,

    /// G-protein activation time constant (ms).
    pub tau_gprotein: f64,

    /// Forward binding rate (1/(mM·ms)).
    pub alpha: f64,

    /// Unbinding rate (1/ms).
    pub beta: f64,

    /// Reversal potential (mV) - K+ reversal.
    pub e_rev: f64,

    /// Maximum conductance (nS).
    pub g_max: f64,
}

impl Default for GABABReceptor {
    fn default() -> Self {
        Self {
            activation: 0.0,
            g_protein: 0.0,
            tau_rise: 50.0,   // 50 ms rise time
            tau_decay: 200.0, // 200 ms decay time
            tau_gprotein: 100.0, // G-protein time constant
            alpha: 0.09,      // Slow binding
            beta: 0.0012,     // Very slow unbinding
            e_rev: -90.0,     // K+ reversal potential
            g_max: 1.0,       // Normalized conductance
        }
    }
}

impl GABABReceptor {
    /// Create a new GABA-B receptor with default parameters.
    pub fn new() -> Self {
        Self::default()
    }

    /// Get the synaptic current (pA).
    pub fn current(&self, voltage: f64) -> f64 {
        // Current depends on G-protein activation
        self.g_max * self.g_protein * (voltage - self.e_rev)
    }
}

impl ReceptorDynamics for GABABReceptor {
    fn update(&mut self, nt_concentration: f64, _voltage: f64, dt: f64) -> Result<()> {
        // Update receptor activation
        let dr = self.alpha * nt_concentration * (1.0 - self.activation)
                 - self.beta * self.activation;
        self.activation += dr * dt;
        self.activation = self.activation.clamp(0.0, 1.0);

        // G-protein activation follows receptor activation with delay
        let dg = (self.activation - self.g_protein) / self.tau_gprotein;
        self.g_protein += dg * dt;
        self.g_protein = self.g_protein.clamp(0.0, 1.0);

        Ok(())
    }

    fn get_conductance(&self) -> f64 {
        self.g_max * self.g_protein
    }

    fn reversal_potential(&self) -> f64 {
        self.e_rev
    }

    fn reset(&mut self) {
        self.activation = 0.0;
        self.g_protein = 0.0;
    }
}

/// Metabotropic glutamate receptor (mGluR).
///
/// mGluRs are G-protein coupled receptors that modulate neuronal excitability
/// through various second messenger pathways.
#[derive(Debug, Clone)]
pub struct MetabotropicGlutamateReceptor {
    /// Receptor activation state (0 to 1).
    pub activation: f64,

    /// G-protein activation state (0 to 1).
    pub g_protein: f64,

    /// Activation time constant (ms).
    pub tau_activation: f64,

    /// Deactivation time constant (ms).
    pub tau_deactivation: f64,

    /// Forward binding rate (1/(mM·ms)).
    pub alpha: f64,

    /// Unbinding rate (1/ms).
    pub beta: f64,
}

impl Default for MetabotropicGlutamateReceptor {
    fn default() -> Self {
        Self {
            activation: 0.0,
            g_protein: 0.0,
            tau_activation: 100.0,   // 100 ms activation
            tau_deactivation: 500.0, // 500 ms deactivation
            alpha: 0.05,             // Slow binding
            beta: 0.002,             // Very slow unbinding
        }
    }
}

impl MetabotropicGlutamateReceptor {
    /// Create a new mGluR with default parameters.
    pub fn new() -> Self {
        Self::default()
    }

    /// Get G-protein activation level.
    pub fn get_gprotein_activation(&self) -> f64 {
        self.g_protein
    }
}

impl ReceptorDynamics for MetabotropicGlutamateReceptor {
    fn update(&mut self, nt_concentration: f64, _voltage: f64, dt: f64) -> Result<()> {
        // Update receptor activation
        let dr = self.alpha * nt_concentration * (1.0 - self.activation)
                 - self.beta * self.activation;
        self.activation += dr * dt;
        self.activation = self.activation.clamp(0.0, 1.0);

        // G-protein activation/deactivation
        let tau = if self.activation > self.g_protein {
            self.tau_activation
        } else {
            self.tau_deactivation
        };
        let dg = (self.activation - self.g_protein) / tau;
        self.g_protein += dg * dt;
        self.g_protein = self.g_protein.clamp(0.0, 1.0);

        Ok(())
    }

    fn get_conductance(&self) -> f64 {
        // Metabotropic receptors don't directly contribute to conductance
        0.0
    }

    fn reversal_potential(&self) -> f64 {
        0.0
    }

    fn reset(&mut self) {
        self.activation = 0.0;
        self.g_protein = 0.0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ampa_receptor_creation() {
        let ampa = AMPAReceptor::new();
        assert_eq!(ampa.open_probability, 0.0);
        assert_eq!(ampa.e_rev, 0.0);
    }

    #[test]
    fn test_ampa_receptor_activation() {
        let mut ampa = AMPAReceptor::new();
        let nt_conc = 1.0; // 1 mM
        let voltage = -65.0;
        let dt = 0.1;

        // Simulate activation
        for _ in 0..100 {
            ampa.update(nt_conc, voltage, dt).unwrap();
        }

        assert!(ampa.open_probability > 0.0);
        assert!(ampa.open_probability <= 1.0);
    }

    #[test]
    fn test_nmda_mg_block() {
        let nmda = NMDAReceptor::new();

        // At -70 mV, strong block
        let block_hyperpol = nmda.mg_block(-70.0);
        assert!(block_hyperpol < 0.1);

        // At 0 mV, partial relief
        let block_depol = nmda.mg_block(0.0);
        assert!(block_depol > block_hyperpol);

        // At +40 mV, nearly complete relief
        let block_strong_depol = nmda.mg_block(40.0);
        assert!(block_strong_depol > 0.8);
    }

    #[test]
    fn test_nmda_receptor_kinetics() {
        let mut nmda = NMDAReceptor::new();
        let nt_conc = 0.5;
        let voltage = 0.0;
        let dt = 0.1;

        // Simulate activation
        for _ in 0..1000 {
            nmda.update(nt_conc, voltage, dt).unwrap();
        }

        assert!(nmda.open_probability > 0.0);
    }

    #[test]
    fn test_gabaa_receptor() {
        let mut gabaa = GABAAReceptor::new();
        assert_eq!(gabaa.e_rev, -70.0);

        gabaa.update(1.0, -65.0, 0.1).unwrap();
        assert!(gabaa.open_probability >= 0.0);
    }

    #[test]
    fn test_gabab_receptor_gprotein() {
        let mut gabab = GABABReceptor::new();

        // Activate with GABA
        for _ in 0..1000 {
            gabab.update(1.0, -65.0, 0.1).unwrap();
        }

        // G-protein should be activated
        assert!(gabab.g_protein > 0.0);
        assert!(gabab.activation > 0.0);
    }

    #[test]
    fn test_receptor_reset() {
        let mut ampa = AMPAReceptor::new();
        ampa.update(1.0, -65.0, 0.1).unwrap();

        ampa.reset();
        assert_eq!(ampa.open_probability, 0.0);
        assert_eq!(ampa.rise_state, 0.0);
    }
}
