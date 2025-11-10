//! Complete synapse model integrating all components.
//!
//! This module provides a comprehensive synapse model that combines:
//! - Presynaptic mechanisms (vesicle release)
//! - Neurotransmitter dynamics
//! - Postsynaptic receptors
//! - Short-term plasticity
//! - Long-term plasticity
//! - Calcium dynamics

use crate::calcium::CalciumDynamics;
use crate::error::{Result, SynapseError};
use crate::neurotransmitter::Neurotransmitter;
use crate::plasticity::STDP;
use crate::receptor::{AMPAReceptor, GABAAReceptor, NMDAReceptor, ReceptorDynamics};
use crate::vesicle::VesiclePool;

/// Type of synapse.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SynapseType {
    /// Excitatory synapse (e.g., glutamatergic).
    Excitatory,
    /// Inhibitory synapse (e.g., GABAergic).
    Inhibitory,
    /// Modulatory synapse (e.g., dopaminergic).
    Modulatory,
}

/// Complete synapse model.
///
/// Integrates all components of synaptic transmission:
/// - Presynaptic vesicle release
/// - Neurotransmitter diffusion and clearance
/// - Postsynaptic receptor activation
/// - Short-term plasticity (depression/facilitation)
/// - Long-term plasticity (STDP)
/// - Calcium dynamics
#[derive(Debug, Clone)]
pub struct Synapse {
    /// Synapse type.
    pub synapse_type: SynapseType,

    /// Synaptic weight (0 to w_max).
    pub weight: f64,

    /// Maximum synaptic weight.
    pub w_max: f64,

    /// Synaptic delay (ms).
    pub delay: f64,

    /// Presynaptic vesicle pool.
    pub vesicle_pool: VesiclePool,

    /// Neurotransmitter in synaptic cleft.
    pub neurotransmitter: Neurotransmitter,

    /// AMPA receptor (for excitatory synapses).
    pub ampa: Option<AMPAReceptor>,

    /// NMDA receptor (for excitatory synapses).
    pub nmda: Option<NMDAReceptor>,

    /// GABA-A receptor (for inhibitory synapses).
    pub gabaa: Option<GABAAReceptor>,

    /// STDP plasticity rule.
    pub stdp: Option<STDP>,

    /// Presynaptic calcium dynamics.
    pub presynaptic_calcium: CalciumDynamics,

    /// Postsynaptic calcium dynamics.
    pub postsynaptic_calcium: CalciumDynamics,

    /// Last presynaptic spike time (ms).
    last_pre_spike_time: Option<f64>,

    /// Last postsynaptic spike time (ms).
    last_post_spike_time: Option<f64>,

    /// Current postsynaptic voltage (mV).
    postsynaptic_voltage: f64,

    /// Pending spike (arrives after delay).
    pending_spike: Option<f64>,
}

impl Synapse {
    /// Create a new excitatory (glutamatergic) synapse.
    pub fn excitatory(weight: f64, delay: f64) -> Result<Self> {
        if weight < 0.0 {
            return Err(SynapseError::InvalidWeight(weight, 0.0, f64::INFINITY));
        }
        if delay < 0.0 {
            return Err(SynapseError::InvalidDelay(delay));
        }

        Ok(Self {
            synapse_type: SynapseType::Excitatory,
            weight,
            w_max: 2.0,
            delay,
            vesicle_pool: VesiclePool::new(),
            neurotransmitter: Neurotransmitter::glutamate(),
            ampa: Some(AMPAReceptor::new()),
            nmda: Some(NMDAReceptor::new()),
            gabaa: None,
            stdp: Some(STDP::new()),
            presynaptic_calcium: CalciumDynamics::presynaptic(),
            postsynaptic_calcium: CalciumDynamics::postsynaptic(),
            last_pre_spike_time: None,
            last_post_spike_time: None,
            postsynaptic_voltage: -65.0,
            pending_spike: None,
        })
    }

    /// Create a new inhibitory (GABAergic) synapse.
    pub fn inhibitory(weight: f64, delay: f64) -> Result<Self> {
        if weight < 0.0 {
            return Err(SynapseError::InvalidWeight(weight, 0.0, f64::INFINITY));
        }
        if delay < 0.0 {
            return Err(SynapseError::InvalidDelay(delay));
        }

        Ok(Self {
            synapse_type: SynapseType::Inhibitory,
            weight,
            w_max: 2.0,
            delay,
            vesicle_pool: VesiclePool::new(),
            neurotransmitter: Neurotransmitter::gaba(),
            ampa: None,
            nmda: None,
            gabaa: Some(GABAAReceptor::new()),
            stdp: Some(STDP::new()),
            presynaptic_calcium: CalciumDynamics::presynaptic(),
            postsynaptic_calcium: CalciumDynamics::postsynaptic(),
            last_pre_spike_time: None,
            last_post_spike_time: None,
            postsynaptic_voltage: -65.0,
            pending_spike: None,
        })
    }

    /// Create depressing excitatory synapse.
    pub fn depressing_excitatory(weight: f64, delay: f64) -> Result<Self> {
        let mut syn = Self::excitatory(weight, delay)?;
        syn.vesicle_pool = VesiclePool::depressing();
        Ok(syn)
    }

    /// Create facilitating excitatory synapse.
    pub fn facilitating_excitatory(weight: f64, delay: f64) -> Result<Self> {
        let mut syn = Self::excitatory(weight, delay)?;
        syn.vesicle_pool = VesiclePool::facilitating();
        Ok(syn)
    }

    /// Process presynaptic spike.
    ///
    /// # Arguments
    /// * `time` - Current time (ms)
    pub fn presynaptic_spike(&mut self, time: f64) -> Result<()> {
        self.last_pre_spike_time = Some(time);

        // Schedule spike to arrive after delay
        self.pending_spike = Some(time + self.delay);

        // Note: Presynaptic calcium spike happens when spike arrives (in update),
        // not here, to properly model the delay in calcium influx

        // Apply STDP if enabled
        if let Some(stdp) = &mut self.stdp {
            let _dw = stdp.pre_spike(time, self.weight);
            self.weight = stdp.apply_update(self.weight);
        }

        Ok(())
    }

    /// Process postsynaptic spike.
    ///
    /// # Arguments
    /// * `time` - Current time (ms)
    pub fn postsynaptic_spike(&mut self, time: f64) -> Result<()> {
        self.last_post_spike_time = Some(time);

        // Update postsynaptic calcium
        self.postsynaptic_calcium.spike();

        // Apply STDP if enabled
        if let Some(stdp) = &mut self.stdp {
            let _dw = stdp.post_spike(time, self.weight);
            self.weight = stdp.apply_update(self.weight);
        }

        Ok(())
    }

    /// Update synapse dynamics.
    ///
    /// # Arguments
    /// * `time` - Current time (ms)
    /// * `postsynaptic_voltage` - Current postsynaptic membrane voltage (mV)
    /// * `dt` - Time step (ms)
    pub fn update(&mut self, time: f64, postsynaptic_voltage: f64, dt: f64) -> Result<()> {
        self.postsynaptic_voltage = postsynaptic_voltage;

        // Check if delayed spike arrives
        if let Some(spike_time) = self.pending_spike {
            if time >= spike_time {
                // Spike arrives - trigger calcium influx at presynaptic terminal
                self.presynaptic_calcium.spike();

                // Release vesicles based on calcium
                let ca_conc = self.presynaptic_calcium.get_concentration();
                let vesicles_released = self.vesicle_pool.release(ca_conc)?;

                // Release neurotransmitter if vesicles were released
                if vesicles_released > 0 {
                    self.neurotransmitter.release();
                }

                self.pending_spike = None;
            }
        }

        // Update vesicle pool dynamics
        self.vesicle_pool.update(dt)?;

        // Update neurotransmitter concentration
        self.neurotransmitter.update(dt)?;

        // Get neurotransmitter concentration
        let nt_conc = self.neurotransmitter.get_concentration();

        // Update receptors
        if let Some(ampa) = &mut self.ampa {
            ampa.update(nt_conc, postsynaptic_voltage, dt)?;
        }
        if let Some(nmda) = &mut self.nmda {
            nmda.update(nt_conc, postsynaptic_voltage, dt)?;

            // NMDA receptors allow calcium influx
            let nmda_current = nmda.current(postsynaptic_voltage);
            let ca_influx = nmda_current.abs() * 0.001; // Convert to calcium (simplified)
            self.postsynaptic_calcium.add_influx(ca_influx)?;
        }
        if let Some(gabaa) = &mut self.gabaa {
            gabaa.update(nt_conc, postsynaptic_voltage, dt)?;
        }

        // Update calcium dynamics
        self.presynaptic_calcium.update(0.0, dt)?;
        self.postsynaptic_calcium.update(0.0, dt)?;

        Ok(())
    }

    /// Get total postsynaptic current.
    ///
    /// # Arguments
    /// * `voltage` - Postsynaptic membrane voltage (mV)
    ///
    /// # Returns
    /// Synaptic current (pA)
    pub fn current(&self, voltage: f64) -> f64 {
        let mut total_current = 0.0;

        if let Some(ampa) = &self.ampa {
            total_current += ampa.current(voltage);
        }
        if let Some(nmda) = &self.nmda {
            total_current += nmda.current(voltage);
        }
        if let Some(gabaa) = &self.gabaa {
            total_current += gabaa.current(voltage);
        }

        total_current * self.weight
    }

    /// Get total postsynaptic conductance.
    ///
    /// # Returns
    /// Synaptic conductance (nS)
    pub fn conductance(&self) -> f64 {
        let mut total_conductance = 0.0;

        if let Some(ampa) = &self.ampa {
            total_conductance += ampa.get_conductance();
        }
        if let Some(nmda) = &self.nmda {
            total_conductance += nmda.get_conductance();
        }
        if let Some(gabaa) = &self.gabaa {
            total_conductance += gabaa.get_conductance();
        }

        total_conductance * self.weight
    }

    /// Get effective synaptic strength (includes short-term plasticity).
    pub fn effective_weight(&self) -> f64 {
        self.weight * self.vesicle_pool.release_probability()
    }

    /// Set synaptic weight.
    pub fn set_weight(&mut self, weight: f64) -> Result<()> {
        if weight < 0.0 || weight > self.w_max {
            return Err(SynapseError::InvalidWeight(weight, 0.0, self.w_max));
        }
        self.weight = weight;
        Ok(())
    }

    /// Enable STDP plasticity.
    pub fn enable_stdp(&mut self, a_plus: f64, a_minus: f64, tau_plus: f64, tau_minus: f64) -> Result<()> {
        self.stdp = Some(STDP::with_params(a_plus, a_minus, tau_plus, tau_minus)?);
        Ok(())
    }

    /// Disable STDP plasticity.
    pub fn disable_stdp(&mut self) {
        self.stdp = None;
    }

    /// Reset synapse to initial state.
    pub fn reset(&mut self) {
        self.vesicle_pool.reset();
        self.neurotransmitter.reset();

        if let Some(ampa) = &mut self.ampa {
            ampa.reset();
        }
        if let Some(nmda) = &mut self.nmda {
            nmda.reset();
        }
        if let Some(gabaa) = &mut self.gabaa {
            gabaa.reset();
        }
        if let Some(stdp) = &mut self.stdp {
            stdp.reset();
        }

        self.presynaptic_calcium.reset();
        self.postsynaptic_calcium.reset();
        self.last_pre_spike_time = None;
        self.last_post_spike_time = None;
        self.pending_spike = None;
    }
}

/// Builder for creating custom synapses.
pub struct SynapseBuilder {
    synapse_type: SynapseType,
    weight: f64,
    w_max: f64,
    delay: f64,
    vesicle_pool: VesiclePool,
    neurotransmitter: Neurotransmitter,
    enable_stdp: bool,
}

impl SynapseBuilder {
    /// Create a new synapse builder.
    pub fn new(synapse_type: SynapseType) -> Self {
        Self {
            synapse_type,
            weight: 1.0,
            w_max: 2.0,
            delay: 1.0,
            vesicle_pool: VesiclePool::new(),
            neurotransmitter: Neurotransmitter::glutamate(),
            enable_stdp: true,
        }
    }

    /// Set synaptic weight.
    pub fn weight(mut self, weight: f64) -> Self {
        self.weight = weight;
        self
    }

    /// Set maximum weight.
    pub fn max_weight(mut self, w_max: f64) -> Self {
        self.w_max = w_max;
        self
    }

    /// Set synaptic delay.
    pub fn delay(mut self, delay: f64) -> Self {
        self.delay = delay;
        self
    }

    /// Set vesicle pool dynamics.
    pub fn vesicle_pool(mut self, pool: VesiclePool) -> Self {
        self.vesicle_pool = pool;
        self
    }

    /// Set neurotransmitter.
    pub fn neurotransmitter(mut self, nt: Neurotransmitter) -> Self {
        self.neurotransmitter = nt;
        self
    }

    /// Enable or disable STDP.
    pub fn stdp(mut self, enable: bool) -> Self {
        self.enable_stdp = enable;
        self
    }

    /// Build the synapse.
    pub fn build(self) -> Result<Synapse> {
        if self.weight < 0.0 {
            return Err(SynapseError::InvalidWeight(self.weight, 0.0, self.w_max));
        }
        if self.delay < 0.0 {
            return Err(SynapseError::InvalidDelay(self.delay));
        }

        let (ampa, nmda, gabaa) = match self.synapse_type {
            SynapseType::Excitatory => (Some(AMPAReceptor::new()), Some(NMDAReceptor::new()), None),
            SynapseType::Inhibitory => (None, None, Some(GABAAReceptor::new())),
            SynapseType::Modulatory => (None, None, None),
        };

        Ok(Synapse {
            synapse_type: self.synapse_type,
            weight: self.weight,
            w_max: self.w_max,
            delay: self.delay,
            vesicle_pool: self.vesicle_pool,
            neurotransmitter: self.neurotransmitter,
            ampa,
            nmda,
            gabaa,
            stdp: if self.enable_stdp { Some(STDP::new()) } else { None },
            presynaptic_calcium: CalciumDynamics::presynaptic(),
            postsynaptic_calcium: CalciumDynamics::postsynaptic(),
            last_pre_spike_time: None,
            last_post_spike_time: None,
            postsynaptic_voltage: -65.0,
            pending_spike: None,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_excitatory_synapse_creation() {
        let syn = Synapse::excitatory(1.0, 1.0).unwrap();
        assert_eq!(syn.synapse_type, SynapseType::Excitatory);
        assert!(syn.ampa.is_some());
        assert!(syn.nmda.is_some());
        assert!(syn.gabaa.is_none());
    }

    #[test]
    fn test_inhibitory_synapse_creation() {
        let syn = Synapse::inhibitory(1.0, 1.0).unwrap();
        assert_eq!(syn.synapse_type, SynapseType::Inhibitory);
        assert!(syn.gabaa.is_some());
        assert!(syn.ampa.is_none());
        assert!(syn.nmda.is_none());
    }

    #[test]
    fn test_synaptic_transmission() {
        let mut syn = Synapse::excitatory(1.0, 1.0).unwrap();

        // Presynaptic spike
        syn.presynaptic_spike(0.0).unwrap();

        // Update to allow spike to propagate and activate receptors
        for t in 0..50 {
            syn.update(t as f64 * 0.1, -65.0, 0.1).unwrap();
        }

        // Should have some conductance or neurotransmitter
        let conductance = syn.conductance();
        let nt_conc = syn.neurotransmitter.get_concentration();
        assert!(conductance > 0.0 || nt_conc > 0.0, "conductance: {}, nt_conc: {}", conductance, nt_conc);
    }

    #[test]
    fn test_synaptic_delay() {
        let mut syn = Synapse::excitatory(1.0, 5.0).unwrap(); // 5 ms delay

        syn.presynaptic_spike(0.0).unwrap();

        // At t=1ms, no conductance yet
        syn.update(1.0, -65.0, 1.0).unwrap();
        assert_eq!(syn.conductance(), 0.0);

        // At t=6ms and beyond, should have conductance after receptors activate
        for t in 6..20 {
            syn.update(t as f64, -65.0, 0.5).unwrap();
        }
        assert!(syn.conductance() > 0.0 || syn.neurotransmitter.get_concentration() > 0.0);
    }

    #[test]
    fn test_short_term_depression() {
        let mut syn = Synapse::depressing_excitatory(1.0, 1.0).unwrap();

        let initial_prob = syn.vesicle_pool.release_probability();

        // Multiple rapid spikes
        for i in 0..5 {
            syn.presynaptic_spike(i as f64 * 10.0).unwrap();
            syn.update((i as f64 + 1.0) * 10.0, -65.0, 10.0).unwrap();
        }

        // Release probability should decrease
        assert!(syn.vesicle_pool.release_probability() < initial_prob);
    }

    #[test]
    fn test_stdp_potentiation() {
        let mut syn = Synapse::excitatory(0.5, 1.0).unwrap();

        let initial_weight = syn.weight;

        // Pre before post -> potentiation
        syn.presynaptic_spike(0.0).unwrap();
        syn.postsynaptic_spike(10.0).unwrap();

        // Weight should increase
        assert!(syn.weight > initial_weight);
    }

    #[test]
    fn test_stdp_depression() {
        let mut syn = Synapse::excitatory(0.5, 1.0).unwrap();

        let initial_weight = syn.weight;

        // Post before pre -> depression
        syn.postsynaptic_spike(0.0).unwrap();
        syn.presynaptic_spike(10.0).unwrap();

        // Weight should decrease
        assert!(syn.weight < initial_weight);
    }

    #[test]
    fn test_synapse_builder() {
        let syn = SynapseBuilder::new(SynapseType::Excitatory)
            .weight(1.5)
            .delay(2.0)
            .stdp(false)
            .build()
            .unwrap();

        assert_eq!(syn.weight, 1.5);
        assert_eq!(syn.delay, 2.0);
        assert!(syn.stdp.is_none());
    }

    #[test]
    fn test_synapse_reset() {
        let mut syn = Synapse::excitatory(1.0, 1.0).unwrap();

        syn.presynaptic_spike(0.0).unwrap();
        syn.update(2.0, -65.0, 2.0).unwrap();

        syn.reset();

        assert_eq!(syn.conductance(), 0.0);
        assert_eq!(syn.last_pre_spike_time, None);
    }
}
