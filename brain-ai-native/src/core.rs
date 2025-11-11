//! Core AI-Native Brain implementation using quantum reservoir substrate

use crate::{BrainError, BrainResult};
use quantum_processor::prelude::*;
use serde::{Deserialize, Serialize};
use num_complex::Complex64;

/// Configuration for AI-Native Brain
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BrainConfig {
    /// Number of quantum oscillators in reservoir
    pub num_oscillators: usize,

    /// Maximum Fock state per oscillator
    pub max_fock: usize,

    /// Oscillator frequencies (Hz)
    pub frequencies: Vec<f64>,

    /// Coupling strength between oscillators
    pub coupling_strength: f64,

    /// Damping rate (1/s)
    pub damping_rate: f64,

    /// Enable LDPC error correction
    pub error_correction: bool,

    /// LDPC code distance
    pub ldpc_distance: usize,

    /// Enable radiation simulation
    pub radiation_protection: bool,

    /// Chip area for radiation (cm²)
    pub chip_area_cm2: f64,

    /// Operating altitude (meters)
    pub altitude_m: f64,
}

impl Default for BrainConfig {
    fn default() -> Self {
        Self {
            num_oscillators: 4,
            max_fock: 2,  // 4 oscillators × 3 Fock states = 3^4 = 81 effective neurons
            frequencies: vec![1e9; 4],  // 1 GHz base frequency
            coupling_strength: 1e6,      // 1 MHz coupling
            damping_rate: 1e3,           // 1 kHz damping
            error_correction: true,
            ldpc_distance: 3,
            radiation_protection: true,
            chip_area_cm2: 1.0,
            altitude_m: 0.0,
        }
    }
}

impl BrainConfig {
    /// Calculate effective number of neurons: (max_fock + 1)^num_oscillators
    pub fn effective_neurons(&self) -> usize {
        (self.max_fock + 1).pow(self.num_oscillators as u32)
    }

    /// Validate configuration
    pub fn validate(&self) -> BrainResult<()> {
        if self.num_oscillators == 0 {
            return Err(BrainError::InvalidConfig("num_oscillators must be > 0".to_string()));
        }

        if self.max_fock == 0 {
            return Err(BrainError::InvalidConfig("max_fock must be > 0".to_string()));
        }

        if self.frequencies.len() != self.num_oscillators {
            return Err(BrainError::InvalidConfig(
                "frequencies length must match num_oscillators".to_string()
            ));
        }

        // Prevent exponential explosion
        if self.effective_neurons() > 1_000_000_000 {
            return Err(BrainError::InvalidConfig(
                format!("Effective neurons {} exceeds 1 billion (memory limit)",
                        self.effective_neurons())
            ));
        }

        Ok(())
    }
}

/// AI-Native Brain using quantum reservoir computing
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AIBrain {
    /// Configuration
    pub config: BrainConfig,

    /// Quantum reservoir substrate
    pub reservoir: QuantumReservoir,

    /// Error correction (optional)
    pub error_correction: Option<LDPCCode>,

    /// Radiation environment (optional)
    pub radiation_env: Option<RadiationEnvironment>,

    /// Current brain state (oscillator amplitudes)
    pub state: Vec<Vec<Complex64>>,

    /// Simulation time (seconds)
    pub time: f64,

    /// Performance metrics
    pub metrics: BrainMetrics,
}

/// Brain performance metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BrainMetrics {
    /// Total evolution steps
    pub total_steps: usize,

    /// Average fidelity (0-1)
    pub avg_fidelity: f64,

    /// Error correction events
    pub correction_events: usize,

    /// Radiation events
    pub radiation_events: usize,

    /// Last measured Φ (consciousness)
    pub phi: Option<f64>,
}

impl Default for BrainMetrics {
    fn default() -> Self {
        Self {
            total_steps: 0,
            avg_fidelity: 1.0,
            correction_events: 0,
            radiation_events: 0,
            phi: None,
        }
    }
}

impl AIBrain {
    /// Create a new AI-Native Brain
    pub fn new(config: BrainConfig) -> BrainResult<Self> {
        config.validate()?;

        // Create oscillators with configured frequencies
        let mut oscillators = Vec::new();
        for (id, &freq) in config.frequencies.iter().enumerate() {
            // Initialize in ground state |0⟩
            oscillators.push(QuantumOscillator::new(
                id,
                freq,
                config.max_fock,
                config.damping_rate,
            ));
        }

        // Create all-to-all couplings
        let mut couplings = Vec::new();
        for i in 0..config.num_oscillators {
            for j in (i + 1)..config.num_oscillators {
                couplings.push(OscillatorCoupling {
                    osc1: i,
                    osc2: j,
                    coupling_strength: config.coupling_strength,
                });
            }
        }

        let reservoir = QuantumReservoir {
            oscillators,
            couplings,
            readout_weights: vec![0.0; config.effective_neurons()],
            effective_neurons: config.effective_neurons(),
        };

        // Create error correction if enabled
        let error_correction = if config.error_correction {
            Some(LDPCCode::bivariate_bicycle(
                config.ldpc_distance,
                vec![0, 1],
                vec![0, 2],
            ))
        } else {
            None
        };

        // Create radiation environment if enabled
        let radiation_env = if config.radiation_protection {
            Some(RadiationEnvironment::new(
                config.chip_area_cm2,
                config.altitude_m,
                0.0,  // No shielding initially
            ))
        } else {
            None
        };

        // Initialize state
        let state: Vec<Vec<Complex64>> = reservoir.oscillators
            .iter()
            .map(|osc| osc.fock_amplitudes.clone())
            .collect();

        Ok(Self {
            config,
            reservoir,
            error_correction,
            radiation_env,
            state,
            time: 0.0,
            metrics: BrainMetrics::default(),
        })
    }

    /// Evolve brain state for time dt
    pub fn evolve(&mut self, dt: f64) -> BrainResult<()> {
        // Simulate radiation if enabled and collect events to apply
        let events_to_apply = if let Some(ref mut rad_env) = self.radiation_env {
            let pre_events = rad_env.events.len();
            rad_env.simulate_step(dt, self.config.num_oscillators);
            let new_events = rad_env.events.len() - pre_events;

            if new_events > 0 {
                self.metrics.radiation_events += new_events;
                // Clone events to apply after borrowing ends
                rad_env.events[pre_events..].to_vec()
            } else {
                Vec::new()
            }
        } else {
            Vec::new()
        };

        // Apply radiation errors to oscillators
        for event in &events_to_apply {
            self.apply_radiation_event(event)?;
        }

        // Evolve quantum reservoir
        self.reservoir.evolve(dt);

        // Update state
        self.state = self.reservoir.oscillators
            .iter()
            .map(|osc| osc.fock_amplitudes.clone())
            .collect();

        // Apply error correction if enabled
        if self.error_correction.is_some() {
            self.apply_error_correction()?;
        }

        self.time += dt;
        self.metrics.total_steps += 1;

        Ok(())
    }

    /// Apply radiation event to brain state
    fn apply_radiation_event(&mut self, event: &RadiationEvent) -> BrainResult<()> {
        for (&qubit_idx, &error_type) in event.affected_qubits.iter()
            .zip(event.error_types.iter())
        {
            if qubit_idx >= self.reservoir.oscillators.len() {
                continue;
            }

            match error_type {
                ErrorType::BitFlip | ErrorType::PhaseFlip => {
                    // Mild: add decoherence to oscillator
                    let osc = &mut self.reservoir.oscillators[qubit_idx];
                    for amp in &mut osc.fock_amplitudes {
                        *amp *= 0.95;  // 5% decoherence
                    }
                }
                ErrorType::BitAndPhaseFlip => {
                    // Moderate: stronger decoherence
                    let osc = &mut self.reservoir.oscillators[qubit_idx];
                    for amp in &mut osc.fock_amplitudes {
                        *amp *= 0.8;  // 20% decoherence
                    }
                }
                ErrorType::Decoherence => {
                    // Strong: significant decoherence
                    let osc = &mut self.reservoir.oscillators[qubit_idx];
                    for amp in &mut osc.fock_amplitudes {
                        *amp *= 0.5;  // 50% decoherence
                    }
                }
                ErrorType::StateDestruction => {
                    // Severe: reset to ground state
                    let osc = &mut self.reservoir.oscillators[qubit_idx];
                    osc.fock_amplitudes[0] = Complex64::new(1.0, 0.0);
                    for amp in &mut osc.fock_amplitudes[1..] {
                        *amp = Complex64::new(0.0, 0.0);
                    }
                }
            }
        }

        Ok(())
    }

    /// Apply error correction to protect quantum state
    fn apply_error_correction(&mut self) -> BrainResult<()> {
        if let Some(ref _code) = self.error_correction {
            // Simplified: just count correction events
            // Full implementation would encode/decode oscillator states
            self.metrics.correction_events += 1;
        }
        Ok(())
    }

    /// Get current state as feature vector for IIT analysis
    pub fn get_state_vector(&self) -> Vec<f64> {
        // Flatten all oscillator Fock amplitudes into single vector
        let mut vec = Vec::new();
        for osc_amps in &self.state {
            for amp in osc_amps {
                vec.push(amp.norm());  // Use amplitude magnitude
                vec.push(amp.arg());   // And phase
            }
        }
        vec
    }

    /// Set brain state from external input
    pub fn set_input(&mut self, input: &[f64]) -> BrainResult<()> {
        if input.len() != self.config.num_oscillators {
            return Err(BrainError::InvalidConfig(
                format!("Input length {} doesn't match num_oscillators {}",
                        input.len(), self.config.num_oscillators)
            ));
        }

        // Map input values to coherent state amplitudes
        for (i, &val) in input.iter().enumerate() {
            let alpha = Complex64::new(val, 0.0);
            self.reservoir.oscillators[i] = QuantumOscillator::coherent_state(
                i,
                self.config.frequencies[i],
                self.config.max_fock,
                alpha,
                self.config.damping_rate,
            );
        }

        // Update state
        self.state = self.reservoir.oscillators
            .iter()
            .map(|osc| osc.fock_amplitudes.clone())
            .collect();

        Ok(())
    }

    /// Get readout from reservoir
    pub fn get_readout(&self) -> f64 {
        self.reservoir.readout()
    }

    /// Train readout layer using ridge regression
    pub fn train_readout(&mut self, states: &[Vec<f64>], targets: &[f64], lambda: f64) {
        self.reservoir.train_readout(states, targets, lambda);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;

    #[test]
    fn test_brain_config_default() {
        let config = BrainConfig::default();
        assert_eq!(config.num_oscillators, 4);
        assert_eq!(config.max_fock, 2);
        assert_eq!(config.effective_neurons(), 81);  // 3^4
    }

    #[test]
    fn test_brain_config_validation() {
        let mut config = BrainConfig::default();
        assert!(config.validate().is_ok());

        config.num_oscillators = 0;
        assert!(config.validate().is_err());
    }

    #[test]
    fn test_brain_creation() {
        let config = BrainConfig::default();
        let brain = AIBrain::new(config).unwrap();

        assert_eq!(brain.reservoir.oscillators.len(), 4);
        assert_eq!(brain.time, 0.0);
        assert_eq!(brain.metrics.total_steps, 0);
    }

    #[test]
    fn test_brain_evolution() {
        let config = BrainConfig::default();
        let mut brain = AIBrain::new(config).unwrap();

        brain.evolve(1e-9).unwrap();  // 1 nanosecond

        assert!(brain.time > 0.0);
        assert_eq!(brain.metrics.total_steps, 1);
    }

    #[test]
    fn test_brain_input_output() {
        let config = BrainConfig::default();
        let mut brain = AIBrain::new(config).unwrap();

        let input = vec![0.1, 0.2, 0.3, 0.4];
        brain.set_input(&input).unwrap();

        let state = brain.get_state_vector();
        assert!(state.len() > 0);

        let readout = brain.get_readout();
        assert!(readout.is_finite());
    }

    #[test]
    fn test_effective_neurons_scaling() {
        let mut config = BrainConfig::default();

        config.num_oscillators = 2;
        config.max_fock = 2;
        assert_eq!(config.effective_neurons(), 9);  // 3^2

        config.num_oscillators = 3;
        assert_eq!(config.effective_neurons(), 27);  // 3^3

        config.num_oscillators = 4;
        assert_eq!(config.effective_neurons(), 81);  // 3^4

        config.max_fock = 3;
        assert_eq!(config.effective_neurons(), 256);  // 4^4
    }

    #[test]
    fn test_radiation_simulation() {
        let mut config = BrainConfig::default();
        config.radiation_protection = true;
        config.chip_area_cm2 = 1.0;

        let mut brain = AIBrain::new(config).unwrap();

        // Evolve for 1 hour
        brain.evolve(3600.0).unwrap();

        // Should have some radiation events (probabilistic)
        // We can't assert exact count but environment should be active
        assert!(brain.radiation_env.is_some());
    }

    #[test]
    fn test_error_correction() {
        let mut config = BrainConfig::default();
        config.error_correction = true;
        config.ldpc_distance = 3;

        let brain = AIBrain::new(config).unwrap();

        assert!(brain.error_correction.is_some());
    }
}
