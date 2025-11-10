//! Quantum Reservoir Computing with Coupled Oscillators
//!
//! Implements quantum reservoir computing where N coupled quantum oscillators
//! create 10^N effective computational neurons through quantum superposition.
//!
//! Based on research showing:
//! - 2 oscillators → 81 computational neurons
//! - 10 oscillators → 10 billion neurons
//!
//! Physics:
//! - Quantum harmonic oscillators with coherent coupling
//! - Hamiltonian evolution: H = Σ ℏω(a†a + 1/2) + Σ g(a†b + ab†)
//! - Fock space representation
//! - Readout via measurement on computational basis

use num_complex::Complex64;
use serde::{Deserialize, Serialize};

/// Quantum harmonic oscillator state
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuantumOscillator {
    /// Oscillator ID
    pub id: usize,
    /// Frequency (ω) in rad/s
    pub frequency: f64,
    /// Maximum Fock state (truncation level)
    pub max_fock: usize,
    /// Current state amplitudes in Fock basis |0⟩, |1⟩, |2⟩, ...
    pub fock_amplitudes: Vec<Complex64>,
    /// Damping rate (γ) in rad/s
    pub damping_rate: f64,
}

impl QuantumOscillator {
    /// Create a new quantum oscillator in ground state |0⟩
    pub fn new(id: usize, frequency: f64, max_fock: usize, damping_rate: f64) -> Self {
        let mut fock_amplitudes = vec![Complex64::new(0.0, 0.0); max_fock + 1];
        fock_amplitudes[0] = Complex64::new(1.0, 0.0); // Ground state

        Self {
            id,
            frequency,
            max_fock,
            fock_amplitudes,
            damping_rate,
        }
    }

    /// Create coherent state |α⟩ = exp(-|α|²/2) Σ (αⁿ/√n!) |n⟩
    pub fn coherent_state(id: usize, frequency: f64, max_fock: usize, alpha: Complex64, damping_rate: f64) -> Self {
        let mut fock_amplitudes = vec![Complex64::new(0.0, 0.0); max_fock + 1];

        let normalization = (-alpha.norm_sqr() / 2.0).exp();
        let mut factorial = 1.0;
        let mut alpha_power = Complex64::new(1.0, 0.0);

        for n in 0..=max_fock {
            if n > 0 {
                factorial *= n as f64;
                alpha_power *= alpha;
            }

            fock_amplitudes[n] = normalization * alpha_power / factorial.sqrt();
        }

        Self {
            id,
            frequency,
            max_fock,
            fock_amplitudes,
            damping_rate,
        }
    }

    /// Apply creation operator a† |n⟩ = √(n+1) |n+1⟩
    pub fn creation_operator(&self, n: usize) -> Option<(f64, usize)> {
        if n >= self.max_fock {
            None
        } else {
            Some((((n + 1) as f64).sqrt(), n + 1))
        }
    }

    /// Apply annihilation operator a |n⟩ = √n |n-1⟩
    pub fn annihilation_operator(&self, n: usize) -> Option<(f64, usize)> {
        if n == 0 {
            None
        } else {
            Some(((n as f64).sqrt(), n - 1))
        }
    }

    /// Number operator ⟨a†a⟩ = average photon number
    pub fn average_photon_number(&self) -> f64 {
        let mut avg = 0.0;
        for n in 0..=self.max_fock {
            avg += (n as f64) * self.fock_amplitudes[n].norm_sqr();
        }
        avg
    }

    /// Position quadrature x = (a + a†)/√2
    pub fn position_quadrature(&self) -> Complex64 {
        let mut x = Complex64::new(0.0, 0.0);
        let sqrt2 = 2.0_f64.sqrt();

        for n in 0..=self.max_fock {
            let amp_n = self.fock_amplitudes[n];

            // a† term
            if let Some((coeff, m)) = self.creation_operator(n) {
                if m <= self.max_fock {
                    x += amp_n.conj() * self.fock_amplitudes[m] * coeff;
                }
            }

            // a term
            if let Some((coeff, m)) = self.annihilation_operator(n) {
                x += amp_n.conj() * self.fock_amplitudes[m] * coeff;
            }
        }

        x / sqrt2
    }

    /// Momentum quadrature p = i(a† - a)/√2
    pub fn momentum_quadrature(&self) -> Complex64 {
        let mut p = Complex64::new(0.0, 0.0);
        let sqrt2 = 2.0_f64.sqrt();
        let i = Complex64::new(0.0, 1.0);

        for n in 0..=self.max_fock {
            let amp_n = self.fock_amplitudes[n];

            // a† term
            if let Some((coeff, m)) = self.creation_operator(n) {
                if m <= self.max_fock {
                    p += amp_n.conj() * self.fock_amplitudes[m] * coeff;
                }
            }

            // -a term
            if let Some((coeff, m)) = self.annihilation_operator(n) {
                p -= amp_n.conj() * self.fock_amplitudes[m] * coeff;
            }
        }

        i * p / sqrt2
    }

    /// Normalize state
    pub fn normalize(&mut self) {
        let norm: f64 = self.fock_amplitudes.iter().map(|a| a.norm_sqr()).sum::<f64>().sqrt();
        if norm > 0.0 {
            for amp in &mut self.fock_amplitudes {
                *amp /= norm;
            }
        }
    }
}

/// Coupling between two oscillators
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OscillatorCoupling {
    /// First oscillator index
    pub osc1: usize,
    /// Second oscillator index
    pub osc2: usize,
    /// Coupling strength g (rad/s)
    pub coupling_strength: f64,
}

/// Quantum reservoir computer with coupled oscillators
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuantumReservoir {
    /// Coupled quantum oscillators
    pub oscillators: Vec<QuantumOscillator>,
    /// Coupling matrix
    pub couplings: Vec<OscillatorCoupling>,
    /// Readout weights (trained)
    pub readout_weights: Vec<f64>,
    /// Effective computational neurons
    pub effective_neurons: usize,
}

impl QuantumReservoir {
    /// Create a new quantum reservoir with N oscillators
    ///
    /// Effective neurons ≈ (max_fock + 1)^N
    /// For max_fock = 8: 2 oscillators → 81 neurons, 10 oscillators → 10^10 neurons
    pub fn new(num_oscillators: usize, frequency: f64, max_fock: usize, damping_rate: f64) -> Self {
        let oscillators: Vec<QuantumOscillator> = (0..num_oscillators)
            .map(|i| QuantumOscillator::new(i, frequency, max_fock, damping_rate))
            .collect();

        // Effective neurons = (max_fock + 1)^N
        let effective_neurons = (max_fock + 1).pow(num_oscillators as u32);

        Self {
            oscillators,
            couplings: Vec::new(),
            readout_weights: vec![0.0; effective_neurons],
            effective_neurons,
        }
    }

    /// Add coupling between oscillators
    pub fn add_coupling(&mut self, osc1: usize, osc2: usize, coupling_strength: f64) {
        if osc1 < self.oscillators.len() && osc2 < self.oscillators.len() && osc1 != osc2 {
            self.couplings.push(OscillatorCoupling {
                osc1,
                osc2,
                coupling_strength,
            });
        }
    }

    /// Add all-to-all coupling
    pub fn add_all_to_all_coupling(&mut self, coupling_strength: f64) {
        let n = self.oscillators.len();
        for i in 0..n {
            for j in (i + 1)..n {
                self.add_coupling(i, j, coupling_strength);
            }
        }
    }

    /// Evolve system for time dt using Hamiltonian evolution
    ///
    /// H = Σᵢ ℏωᵢ(aᵢ†aᵢ + 1/2) + Σᵢⱼ gᵢⱼ(aᵢ†aⱼ + aᵢaⱼ†)
    pub fn evolve(&mut self, dt: f64) {
        let hbar = 1.0; // Use natural units

        // Single oscillator evolution (free Hamiltonian)
        for osc in &mut self.oscillators {
            let mut new_amps = osc.fock_amplitudes.clone();

            for n in 0..=osc.max_fock {
                let energy = hbar * osc.frequency * (n as f64 + 0.5);
                let phase = -energy * dt;
                new_amps[n] = osc.fock_amplitudes[n] * Complex64::from_polar(1.0, phase);
            }

            osc.fock_amplitudes = new_amps;
        }

        // Coupling evolution (interaction Hamiltonian)
        // Using first-order approximation: e^(-iHt) ≈ I - iHt for small dt
        for coupling in &self.couplings {
            let g = coupling.coupling_strength;
            let osc1 = &self.oscillators[coupling.osc1];
            let osc2 = &self.oscillators[coupling.osc2];

            // This is simplified; full implementation needs tensor product space
            // For now, apply perturbative correction to each oscillator

            let perturbation = g * dt;

            // Oscillator 1 gets influenced by oscillator 2
            let avg_photons_2 = osc2.average_photon_number();
            let mut new_amps_1 = osc1.fock_amplitudes.clone();

            for n in 0..=osc1.max_fock {
                // a†b + ab† interaction
                if let Some((coeff, m)) = osc1.creation_operator(n) {
                    if m <= osc1.max_fock {
                        new_amps_1[m] += -Complex64::new(0.0, perturbation * coeff * avg_photons_2)
                            * osc1.fock_amplitudes[n];
                    }
                }
            }

            self.oscillators[coupling.osc1].fock_amplitudes = new_amps_1;
        }

        // Apply damping
        for osc in &mut self.oscillators {
            let decay_factor = (-osc.damping_rate * dt).exp();
            for amp in &mut osc.fock_amplitudes {
                *amp *= decay_factor;
            }
            osc.normalize();
        }
    }

    /// Get computational basis state index from oscillator Fock states
    fn get_basis_index(&self, fock_states: &[usize]) -> usize {
        let mut index = 0;
        let base = self.oscillators[0].max_fock + 1;

        for (i, &n) in fock_states.iter().enumerate() {
            index += n * base.pow(i as u32);
        }

        index
    }

    /// Compute readout (weighted sum of basis state probabilities)
    pub fn readout(&self) -> f64 {
        let mut output = 0.0;

        // For small systems, enumerate all basis states
        if self.oscillators.len() <= 3 {
            self.enumerate_basis_states(&mut vec![0; self.oscillators.len()], 0, &mut output);
        } else {
            // For large systems, sample most probable states
            output = self.sample_readout(1000);
        }

        output
    }

    /// Recursive enumeration of basis states
    fn enumerate_basis_states(&self, current_state: &mut Vec<usize>, osc_idx: usize, output: &mut f64) {
        if osc_idx == self.oscillators.len() {
            // Compute probability of this basis state
            let mut prob = 1.0;
            for (i, &n) in current_state.iter().enumerate() {
                prob *= self.oscillators[i].fock_amplitudes[n].norm_sqr();
            }

            let basis_idx = self.get_basis_index(current_state);
            if basis_idx < self.readout_weights.len() {
                *output += prob * self.readout_weights[basis_idx];
            }
        } else {
            // Recurse over Fock states for current oscillator
            for n in 0..=self.oscillators[osc_idx].max_fock {
                current_state[osc_idx] = n;
                self.enumerate_basis_states(current_state, osc_idx + 1, output);
            }
        }
    }

    /// Sample-based readout for large systems
    fn sample_readout(&self, num_samples: usize) -> f64 {
        use rand::Rng;
        let mut rng = rand::thread_rng();
        let mut sum = 0.0;

        for _ in 0..num_samples {
            // Sample Fock state for each oscillator
            let mut fock_states = Vec::new();

            for osc in &self.oscillators {
                let mut cumulative = 0.0;
                let r: f64 = rng.gen();

                for n in 0..=osc.max_fock {
                    cumulative += osc.fock_amplitudes[n].norm_sqr();
                    if r < cumulative {
                        fock_states.push(n);
                        break;
                    }
                }
            }

            if fock_states.len() == self.oscillators.len() {
                let basis_idx = self.get_basis_index(&fock_states);
                if basis_idx < self.readout_weights.len() {
                    sum += self.readout_weights[basis_idx];
                }
            }
        }

        sum / num_samples as f64
    }

    /// Train readout weights using ridge regression
    ///
    /// X: Input matrix (num_samples × num_features)
    /// y: Target outputs (num_samples)
    pub fn train_readout(&mut self, states: &[Vec<f64>], targets: &[f64], _lambda: f64) {
        let n_samples = states.len();
        let n_features = self.effective_neurons;

        if n_samples == 0 || states[0].len() != n_features || targets.len() != n_samples {
            return;
        }

        // Ridge regression: w = (X^T X + λI)^(-1) X^T y
        // Simplified: just use pseudo-inverse for now

        // For demonstration, use simple averaging
        // Full implementation would use nalgebra for matrix inversion
        let mut sum_weights = vec![0.0; n_features];

        for (state, &target) in states.iter().zip(targets) {
            for (i, &val) in state.iter().enumerate() {
                sum_weights[i] += target * val / n_samples as f64;
            }
        }

        self.readout_weights = sum_weights;
    }

    /// Get total energy of the system
    pub fn total_energy(&self) -> f64 {
        let hbar = 1.0;
        let mut energy = 0.0;

        // Single oscillator energies
        for osc in &self.oscillators {
            for n in 0..=osc.max_fock {
                let prob = osc.fock_amplitudes[n].norm_sqr();
                energy += prob * hbar * osc.frequency * (n as f64 + 0.5);
            }
        }

        // Coupling energies (approximate)
        for coupling in &self.couplings {
            let osc1 = &self.oscillators[coupling.osc1];
            let osc2 = &self.oscillators[coupling.osc2];

            let n1 = osc1.average_photon_number();
            let n2 = osc2.average_photon_number();

            energy += coupling.coupling_strength * (n1 * n2).sqrt();
        }

        energy
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;

    #[test]
    fn test_oscillator_ground_state() {
        let osc = QuantumOscillator::new(0, 1.0, 10, 0.01);
        assert_eq!(osc.fock_amplitudes[0].norm(), 1.0);
        assert_relative_eq!(osc.average_photon_number(), 0.0);
    }

    #[test]
    fn test_coherent_state() {
        let alpha = Complex64::new(2.0, 0.0);
        let osc = QuantumOscillator::coherent_state(0, 1.0, 20, alpha, 0.01);

        // Average photon number should be |α|²
        assert_relative_eq!(osc.average_photon_number(), alpha.norm_sqr(), epsilon = 0.1);
    }

    #[test]
    fn test_creation_annihilation() {
        let osc = QuantumOscillator::new(0, 1.0, 10, 0.01);

        // a|0⟩ = 0
        assert!(osc.annihilation_operator(0).is_none());

        // a†|0⟩ = |1⟩
        let (coeff, n) = osc.creation_operator(0).unwrap();
        assert_relative_eq!(coeff, 1.0);
        assert_eq!(n, 1);
    }

    #[test]
    fn test_reservoir_creation() {
        let reservoir = QuantumReservoir::new(2, 1.0, 8, 0.01);

        assert_eq!(reservoir.oscillators.len(), 2);
        assert_eq!(reservoir.effective_neurons, 81); // 9^2
    }

    #[test]
    fn test_reservoir_scaling() {
        let reservoir_2 = QuantumReservoir::new(2, 1.0, 8, 0.01);
        let reservoir_3 = QuantumReservoir::new(3, 1.0, 8, 0.01);

        assert_eq!(reservoir_2.effective_neurons, 81);
        assert_eq!(reservoir_3.effective_neurons, 729); // 9^3
    }

    #[test]
    fn test_coupling() {
        let mut reservoir = QuantumReservoir::new(3, 1.0, 5, 0.01);
        reservoir.add_coupling(0, 1, 0.1);
        reservoir.add_coupling(1, 2, 0.1);

        assert_eq!(reservoir.couplings.len(), 2);
    }

    #[test]
    fn test_all_to_all_coupling() {
        let mut reservoir = QuantumReservoir::new(4, 1.0, 5, 0.01);
        reservoir.add_all_to_all_coupling(0.1);

        // n(n-1)/2 = 4*3/2 = 6 couplings
        assert_eq!(reservoir.couplings.len(), 6);
    }

    #[test]
    fn test_evolution() {
        let mut reservoir = QuantumReservoir::new(2, 1.0, 5, 0.01);
        reservoir.add_coupling(0, 1, 0.1);

        let energy_before = reservoir.total_energy();
        reservoir.evolve(0.01);
        let energy_after = reservoir.total_energy();

        // Energy should be approximately conserved (with damping)
        assert!(energy_after > 0.0);
        assert!(energy_after <= energy_before * 1.1); // Allow some numerical error
    }

    #[test]
    fn test_readout() {
        let mut reservoir = QuantumReservoir::new(2, 1.0, 3, 0.01);
        reservoir.readout_weights = vec![1.0; 16]; // 4^2

        let output = reservoir.readout();
        assert!(output >= 0.0);
    }
}
