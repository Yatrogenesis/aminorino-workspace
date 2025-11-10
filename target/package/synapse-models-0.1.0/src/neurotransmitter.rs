//! Neurotransmitter types and their biophysical properties.
//!
//! This module defines various neurotransmitters with their characteristic properties
//! including reversal potentials, time constants, and diffusion rates.

use crate::error::Result;

/// Types of neurotransmitters in the nervous system.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum NeurotransmitterType {
    /// Glutamate - primary excitatory neurotransmitter.
    Glutamate,
    /// GABA - primary inhibitory neurotransmitter.
    GABA,
    /// Dopamine - involved in reward and motor control.
    Dopamine,
    /// Serotonin - regulates mood and sleep.
    Serotonin,
    /// Acetylcholine - involved in memory and muscle activation.
    Acetylcholine,
    /// Norepinephrine - involved in arousal and stress response.
    Norepinephrine,
}

/// Properties of a neurotransmitter.
#[derive(Debug, Clone)]
pub struct NeurotransmitterProperties {
    /// Type of neurotransmitter.
    pub nt_type: NeurotransmitterType,

    /// Reversal potential (mV).
    pub reversal_potential: f64,

    /// Time constant for neurotransmitter clearance from synaptic cleft (ms).
    pub clearance_time: f64,

    /// Diffusion coefficient (μm²/ms).
    pub diffusion_rate: f64,

    /// Peak concentration in synaptic cleft after release (mM).
    pub peak_concentration: f64,

    /// Whether this is primarily excitatory or inhibitory.
    pub is_excitatory: bool,
}

impl NeurotransmitterProperties {
    /// Create properties for glutamate.
    ///
    /// Glutamate is the primary excitatory neurotransmitter in the CNS.
    /// It acts on ionotropic receptors (AMPA, NMDA, Kainate) and metabotropic receptors (mGluR).
    pub fn glutamate() -> Self {
        Self {
            nt_type: NeurotransmitterType::Glutamate,
            reversal_potential: 0.0,  // Non-selective cation channel
            clearance_time: 1.2,      // Fast reuptake by astrocytes and neurons
            diffusion_rate: 0.33,     // Relatively fast diffusion
            peak_concentration: 1.1,  // ~1 mM in synaptic cleft
            is_excitatory: true,
        }
    }

    /// Create properties for GABA.
    ///
    /// GABA is the primary inhibitory neurotransmitter in the CNS.
    /// Acts on ionotropic GABA-A and metabotropic GABA-B receptors.
    pub fn gaba() -> Self {
        Self {
            nt_type: NeurotransmitterType::GABA,
            reversal_potential: -70.0, // Chloride channel (can vary with development)
            clearance_time: 3.0,       // Moderate clearance rate
            diffusion_rate: 0.25,      // Moderate diffusion
            peak_concentration: 1.0,   // ~1 mM in synaptic cleft
            is_excitatory: false,
        }
    }

    /// Create properties for dopamine.
    ///
    /// Dopamine is involved in reward, motivation, and motor control.
    /// Acts primarily on metabotropic D1-D5 receptors.
    pub fn dopamine() -> Self {
        Self {
            nt_type: NeurotransmitterType::Dopamine,
            reversal_potential: 0.0,   // Modulates other channels
            clearance_time: 100.0,     // Slower clearance, volume transmission
            diffusion_rate: 0.2,       // Slower diffusion
            peak_concentration: 0.05,  // Lower peak concentration
            is_excitatory: true,       // Generally excitatory modulation
        }
    }

    /// Create properties for serotonin.
    ///
    /// Serotonin (5-HT) regulates mood, sleep, and appetite.
    /// Acts on numerous receptor subtypes (5-HT1-7).
    pub fn serotonin() -> Self {
        Self {
            nt_type: NeurotransmitterType::Serotonin,
            reversal_potential: 0.0,   // Modulates other channels
            clearance_time: 80.0,      // Slow clearance
            diffusion_rate: 0.18,      // Slow diffusion
            peak_concentration: 0.04,  // Low peak concentration
            is_excitatory: false,      // Mixed effects, often inhibitory
        }
    }

    /// Create properties for acetylcholine.
    ///
    /// Acetylcholine is involved in memory formation and muscle activation.
    /// Acts on nicotinic (ionotropic) and muscarinic (metabotropic) receptors.
    pub fn acetylcholine() -> Self {
        Self {
            nt_type: NeurotransmitterType::Acetylcholine,
            reversal_potential: -10.0, // Nicotinic receptors (mixed cation)
            clearance_time: 0.5,       // Very fast degradation by AChE
            diffusion_rate: 0.4,       // Fast diffusion
            peak_concentration: 0.5,   // Moderate concentration
            is_excitatory: true,       // Generally excitatory
        }
    }

    /// Create properties for norepinephrine.
    ///
    /// Norepinephrine is involved in arousal, attention, and stress response.
    /// Acts on alpha and beta adrenergic receptors.
    pub fn norepinephrine() -> Self {
        Self {
            nt_type: NeurotransmitterType::Norepinephrine,
            reversal_potential: 0.0,   // Modulates other channels
            clearance_time: 120.0,     // Slow clearance
            diffusion_rate: 0.15,      // Slow diffusion
            peak_concentration: 0.03,  // Low peak concentration
            is_excitatory: true,       // Generally excitatory modulation
        }
    }

    /// Get current neurotransmitter concentration based on time since release.
    ///
    /// Uses exponential decay: C(t) = C_peak * exp(-t / τ_clearance)
    ///
    /// # Arguments
    /// * `time_since_release` - Time since neurotransmitter release (ms)
    ///
    /// # Returns
    /// Current concentration (mM)
    pub fn concentration(&self, time_since_release: f64) -> f64 {
        if time_since_release < 0.0 {
            return 0.0;
        }
        self.peak_concentration * (-time_since_release / self.clearance_time).exp()
    }
}

/// Neurotransmitter state in the synaptic cleft.
#[derive(Debug, Clone)]
pub struct Neurotransmitter {
    /// Properties of this neurotransmitter.
    pub properties: NeurotransmitterProperties,

    /// Current concentration in synaptic cleft (mM).
    pub concentration: f64,

    /// Time since last release (ms).
    pub time_since_release: f64,
}

impl Neurotransmitter {
    /// Create a new neurotransmitter instance.
    pub fn new(properties: NeurotransmitterProperties) -> Self {
        Self {
            properties,
            concentration: 0.0,
            time_since_release: f64::INFINITY,
        }
    }

    /// Create a glutamate neurotransmitter.
    pub fn glutamate() -> Self {
        Self::new(NeurotransmitterProperties::glutamate())
    }

    /// Create a GABA neurotransmitter.
    pub fn gaba() -> Self {
        Self::new(NeurotransmitterProperties::gaba())
    }

    /// Create a dopamine neurotransmitter.
    pub fn dopamine() -> Self {
        Self::new(NeurotransmitterProperties::dopamine())
    }

    /// Create a serotonin neurotransmitter.
    pub fn serotonin() -> Self {
        Self::new(NeurotransmitterProperties::serotonin())
    }

    /// Create an acetylcholine neurotransmitter.
    pub fn acetylcholine() -> Self {
        Self::new(NeurotransmitterProperties::acetylcholine())
    }

    /// Create a norepinephrine neurotransmitter.
    pub fn norepinephrine() -> Self {
        Self::new(NeurotransmitterProperties::norepinephrine())
    }

    /// Release neurotransmitter into synaptic cleft.
    pub fn release(&mut self) {
        self.concentration = self.properties.peak_concentration;
        self.time_since_release = 0.0;
    }

    /// Update neurotransmitter concentration over time.
    ///
    /// # Arguments
    /// * `dt` - Time step (ms)
    pub fn update(&mut self, dt: f64) -> Result<()> {
        self.time_since_release += dt;
        self.concentration = self.properties.concentration(self.time_since_release);
        Ok(())
    }

    /// Get current concentration.
    pub fn get_concentration(&self) -> f64 {
        self.concentration
    }

    /// Reset neurotransmitter state.
    pub fn reset(&mut self) {
        self.concentration = 0.0;
        self.time_since_release = f64::INFINITY;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_glutamate_properties() {
        let glu = NeurotransmitterProperties::glutamate();
        assert_eq!(glu.nt_type, NeurotransmitterType::Glutamate);
        assert_eq!(glu.reversal_potential, 0.0);
        assert!(glu.is_excitatory);
    }

    #[test]
    fn test_gaba_properties() {
        let gaba = NeurotransmitterProperties::gaba();
        assert_eq!(gaba.nt_type, NeurotransmitterType::GABA);
        assert_eq!(gaba.reversal_potential, -70.0);
        assert!(!gaba.is_excitatory);
    }

    #[test]
    fn test_concentration_decay() {
        let glu = NeurotransmitterProperties::glutamate();
        let c0 = glu.concentration(0.0);
        let c1 = glu.concentration(1.2); // One time constant
        let c2 = glu.concentration(2.4); // Two time constants

        assert_eq!(c0, glu.peak_concentration);
        assert!((c1 - glu.peak_concentration / std::f64::consts::E).abs() < 1e-6);
        assert!(c2 < c1);
    }

    #[test]
    fn test_neurotransmitter_release() {
        let mut nt = Neurotransmitter::glutamate();
        assert_eq!(nt.concentration, 0.0);

        nt.release();
        assert!(nt.concentration > 0.0);
        assert_eq!(nt.time_since_release, 0.0);
    }

    #[test]
    fn test_neurotransmitter_update() {
        let mut nt = Neurotransmitter::glutamate();
        nt.release();

        let initial_conc = nt.concentration;
        nt.update(1.0).unwrap();

        assert!(nt.concentration < initial_conc);
        assert_eq!(nt.time_since_release, 1.0);
    }

    #[test]
    fn test_neurotransmitter_reset() {
        let mut nt = Neurotransmitter::glutamate();
        nt.release();
        nt.update(1.0).unwrap();

        nt.reset();
        assert_eq!(nt.concentration, 0.0);
        assert_eq!(nt.time_since_release, f64::INFINITY);
    }
}
