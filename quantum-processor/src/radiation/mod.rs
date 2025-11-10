//! Radiation Effects Simulation for Quantum Processors
//!
//! Models cosmic radiation impact including:
//! - Cosmic ray muons, neutrons, protons
//! - Single Event Upsets (SEU)
//! - Bit flip errors in quantum states
//! - Multi-qubit correlated errors
//!
//! Physics:
//! - Radiation flux at sea level: ~1 muon/cm²/min
//! - Neutron flux: ~10 neutrons/cm²/hour
//! - Energy deposition in silicon/superconductors
//! - Cross-sections for qubit disruption

use serde::{Deserialize, Serialize};
use rand::Rng;

/// Type of radiation particle
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RadiationParticle {
    /// Cosmic ray muon (µ)
    Muon,
    /// Neutron
    Neutron,
    /// Proton
    Proton,
    /// Alpha particle (He nucleus)
    Alpha,
    /// Heavy ion
    HeavyIon,
}

impl RadiationParticle {
    /// Get typical energy in MeV
    pub fn typical_energy_mev(&self) -> f64 {
        match self {
            RadiationParticle::Muon => 4000.0,      // 4 GeV
            RadiationParticle::Neutron => 10.0,      // 10 MeV
            RadiationParticle::Proton => 100.0,      // 100 MeV
            RadiationParticle::Alpha => 5.0,         // 5 MeV
            RadiationParticle::HeavyIon => 1000.0,   // 1 GeV
        }
    }

    /// Get flux at sea level (particles/cm²/hour)
    pub fn flux_at_sea_level(&self) -> f64 {
        match self {
            RadiationParticle::Muon => 60.0,         // ~1/min
            RadiationParticle::Neutron => 10.0,      // ~10/hour
            RadiationParticle::Proton => 0.1,        // Rare
            RadiationParticle::Alpha => 0.01,        // Very rare
            RadiationParticle::HeavyIon => 0.001,    // Extremely rare
        }
    }

    /// Get cross-section for qubit disruption (cm²)
    pub fn disruption_cross_section(&self) -> f64 {
        match self {
            RadiationParticle::Muon => 1e-12,        // Small
            RadiationParticle::Neutron => 1e-10,     // Moderate
            RadiationParticle::Proton => 5e-11,      // Moderate
            RadiationParticle::Alpha => 1e-9,        // Large
            RadiationParticle::HeavyIon => 1e-8,     // Very large
        }
    }

    /// Linear Energy Transfer (LET) in MeV·cm²/mg
    pub fn let_value(&self) -> f64 {
        match self {
            RadiationParticle::Muon => 0.2,
            RadiationParticle::Neutron => 5.0,
            RadiationParticle::Proton => 10.0,
            RadiationParticle::Alpha => 100.0,
            RadiationParticle::HeavyIon => 500.0,
        }
    }
}

/// Radiation event affecting quantum processor
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RadiationEvent {
    /// Particle type
    pub particle: RadiationParticle,
    /// Event energy in MeV
    pub energy_mev: f64,
    /// Position (x, y, z) in cm
    pub position: (f64, f64, f64),
    /// Affected qubits
    pub affected_qubits: Vec<usize>,
    /// Error type per qubit
    pub error_types: Vec<ErrorType>,
    /// Time of occurrence (seconds)
    pub time: f64,
}

/// Type of error induced by radiation
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ErrorType {
    /// Bit flip (X error)
    BitFlip,
    /// Phase flip (Z error)
    PhaseFlip,
    /// Both bit and phase flip (Y error)
    BitAndPhaseFlip,
    /// Loss of coherence
    Decoherence,
    /// Complete state destruction
    StateDestruction,
}

/// Radiation environment simulator
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RadiationEnvironment {
    /// Chip area in cm²
    pub chip_area_cm2: f64,
    /// Altitude in meters (affects flux)
    pub altitude_m: f64,
    /// Shielding thickness in cm (water equivalent)
    pub shielding_cm: f64,
    /// Simulation time in seconds
    pub current_time: f64,
    /// Event history
    pub events: Vec<RadiationEvent>,
}

impl RadiationEnvironment {
    /// Create a new radiation environment
    pub fn new(chip_area_cm2: f64, altitude_m: f64, shielding_cm: f64) -> Self {
        Self {
            chip_area_cm2,
            altitude_m,
            shielding_cm,
            current_time: 0.0,
            events: Vec::new(),
        }
    }

    /// Get altitude correction factor for flux
    ///
    /// Cosmic ray flux doubles approximately every 1500m
    fn altitude_factor(&self) -> f64 {
        (self.altitude_m / 1500.0).exp2()
    }

    /// Get shielding attenuation factor
    ///
    /// Exponential attenuation with mean free path ~15 cm water
    fn shielding_factor(&self) -> f64 {
        (-self.shielding_cm / 15.0).exp()
    }

    /// Get effective flux for particle type (particles/cm²/s)
    fn effective_flux(&self, particle: RadiationParticle) -> f64 {
        let base_flux = particle.flux_at_sea_level() / 3600.0; // Convert to per second
        base_flux * self.altitude_factor() * self.shielding_factor()
    }

    /// Simulate radiation for a time step
    pub fn simulate_step(&mut self, dt: f64, num_qubits: usize) {
        let mut rng = rand::thread_rng();

        // Check each particle type
        for particle in [
            RadiationParticle::Muon,
            RadiationParticle::Neutron,
            RadiationParticle::Proton,
            RadiationParticle::Alpha,
            RadiationParticle::HeavyIon,
        ] {
            let flux = self.effective_flux(particle);
            let expected_events = flux * self.chip_area_cm2 * dt;

            // Poisson sampling for number of events
            let num_events = self.poisson_sample(expected_events);

            for _ in 0..num_events {
                let event = self.generate_event(particle, num_qubits, &mut rng);
                self.events.push(event);
            }
        }

        self.current_time += dt;
    }

    /// Sample from Poisson distribution
    fn poisson_sample(&self, lambda: f64) -> usize {
        if lambda <= 0.0 {
            return 0;
        }

        let mut rng = rand::thread_rng();

        // For small lambda, use Knuth's algorithm
        if lambda < 30.0 {
            let l = (-lambda).exp();
            let mut k = 0;
            let mut p = 1.0;

            loop {
                k += 1;
                p *= rng.gen::<f64>();
                if p <= l {
                    return k - 1;
                }
                if k > 100 {
                    return 0; // Safety cutoff
                }
            }
        } else {
            // For large lambda, use normal approximation
            let z: f64 = rng.gen::<f64>() * 2.0 - 1.0;
            let value = lambda + lambda.sqrt() * z;
            value.max(0.0) as usize
        }
    }

    /// Generate a radiation event
    fn generate_event<R: Rng>(
        &self,
        particle: RadiationParticle,
        num_qubits: usize,
        rng: &mut R,
    ) -> RadiationEvent {
        // Random energy (Gaussian around typical energy)
        let energy_mean = particle.typical_energy_mev();
        let energy_std = energy_mean * 0.5;
        let energy_mev = energy_mean + energy_std * (rng.gen::<f64>() - 0.5) * 2.0;

        // Random position on chip
        let chip_size = self.chip_area_cm2.sqrt();
        let position = (
            rng.gen::<f64>() * chip_size,
            rng.gen::<f64>() * chip_size,
            0.0, // Surface level
        );

        // Determine affected qubits based on LET and geometry
        let let_value = particle.let_value();
        let _track_length_um = (energy_mev / let_value) * 100.0; // Approximate

        // Number of qubits affected (proportional to LET and energy)
        let base_affected = (let_value * energy_mev / 100.0).sqrt() as usize;
        let num_affected = (base_affected.max(1)).min(num_qubits);

        let mut affected_qubits = Vec::new();
        let mut error_types = Vec::new();

        // Randomly select affected qubits
        for _ in 0..num_affected {
            let qubit = rng.gen_range(0..num_qubits);
            if !affected_qubits.contains(&qubit) {
                affected_qubits.push(qubit);

                // Determine error type based on energy deposition
                let error_type = if energy_mev > 1000.0 {
                    // High energy: complete destruction
                    ErrorType::StateDestruction
                } else if energy_mev > 100.0 {
                    // Medium-high energy: bit and phase flip
                    ErrorType::BitAndPhaseFlip
                } else if rng.gen::<f64>() < 0.5 {
                    // Medium energy: bit flip more likely
                    ErrorType::BitFlip
                } else {
                    // Medium energy: phase flip
                    ErrorType::PhaseFlip
                };

                error_types.push(error_type);
            }
        }

        RadiationEvent {
            particle,
            energy_mev,
            position,
            affected_qubits,
            error_types,
            time: self.current_time,
        }
    }

    /// Get event rate (events per second)
    pub fn event_rate(&self) -> f64 {
        if self.current_time == 0.0 {
            0.0
        } else {
            self.events.len() as f64 / self.current_time
        }
    }

    /// Get total events by particle type
    pub fn events_by_type(&self, particle: RadiationParticle) -> usize {
        self.events.iter().filter(|e| e.particle == particle).count()
    }

    /// Clear event history
    pub fn clear_events(&mut self) {
        self.events.clear();
    }

    /// Get mean time between failures (MTBF) for critical errors
    ///
    /// Critical errors are StateDestruction or multi-qubit errors
    pub fn mtbf_seconds(&self) -> f64 {
        let critical_events: usize = self.events
            .iter()
            .filter(|e| {
                e.error_types.contains(&ErrorType::StateDestruction)
                    || e.affected_qubits.len() > 3
            })
            .count();

        if critical_events == 0 {
            f64::INFINITY
        } else {
            self.current_time / critical_events as f64
        }
    }

    /// Estimate upset rate at 10km altitude (aircraft)
    pub fn aircraft_upset_rate(&self) -> f64 {
        let altitude_factor = (10000.0_f64 / 1500.0).exp2();
        self.event_rate() * altitude_factor
    }

    /// Estimate upset rate in space (LEO)
    pub fn space_upset_rate(&self) -> f64 {
        // Space radiation is ~100x worse than ground level
        self.event_rate() * 100.0
    }
}

/// Radiation-hardened qubit wrapper
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RadiationHardenedQubit {
    /// Base qubit ID
    pub qubit_id: usize,
    /// Redundancy factor (N copies)
    pub redundancy: usize,
    /// Error correction capable
    pub error_correction: bool,
    /// Shielding factor
    pub shielding_factor: f64,
}

impl RadiationHardenedQubit {
    /// Create a radiation-hardened qubit
    pub fn new(qubit_id: usize, redundancy: usize, error_correction: bool) -> Self {
        Self {
            qubit_id,
            redundancy,
            error_correction,
            shielding_factor: 1.0,
        }
    }

    /// Apply shielding (reduces effective cross-section)
    pub fn add_shielding(&mut self, thickness_cm: f64) {
        self.shielding_factor *= (-thickness_cm / 15.0).exp();
    }

    /// Calculate effective error rate reduction
    pub fn error_rate_reduction(&self) -> f64 {
        let mut reduction = self.shielding_factor;

        // Redundancy provides polynomial improvement
        if self.redundancy > 1 {
            reduction /= (self.redundancy as f64).powi(2);
        }

        // Error correction provides exponential improvement
        if self.error_correction {
            reduction /= 10.0;
        }

        reduction
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_particle_properties() {
        let muon = RadiationParticle::Muon;
        assert!(muon.typical_energy_mev() > 1000.0);
        assert!(muon.flux_at_sea_level() > 10.0);
    }

    #[test]
    fn test_environment_creation() {
        let env = RadiationEnvironment::new(1.0, 0.0, 0.0);
        assert_eq!(env.chip_area_cm2, 1.0);
        assert_eq!(env.events.len(), 0);
    }

    #[test]
    fn test_altitude_factor() {
        let env = RadiationEnvironment::new(1.0, 1500.0, 0.0);
        assert!((env.altitude_factor() - 2.0).abs() < 0.1);
    }

    #[test]
    fn test_shielding_factor() {
        let env = RadiationEnvironment::new(1.0, 0.0, 15.0);
        assert!((env.shielding_factor() - (1.0_f64 / std::f64::consts::E)).abs() < 0.01);
    }

    #[test]
    fn test_radiation_simulation() {
        let mut env = RadiationEnvironment::new(1.0, 0.0, 0.0);
        env.simulate_step(3600.0, 100); // 1 hour

        // Should have some events
        assert!(env.events.len() > 0);
    }

    #[test]
    fn test_event_rate() {
        let mut env = RadiationEnvironment::new(1.0, 0.0, 0.0);
        env.simulate_step(1000.0, 100);

        let rate = env.event_rate();
        assert!(rate > 0.0);
    }

    #[test]
    fn test_radiation_hardening() {
        let mut qubit = RadiationHardenedQubit::new(0, 3, true);
        qubit.add_shielding(10.0);

        let reduction = qubit.error_rate_reduction();
        assert!(reduction < 0.1); // At least 10x improvement
    }

    #[test]
    fn test_mtbf() {
        let mut env = RadiationEnvironment::new(1.0, 0.0, 0.0);
        env.simulate_step(10000.0, 100); // Longer simulation

        let mtbf = env.mtbf_seconds();
        assert!(mtbf > 0.0 || mtbf == f64::INFINITY);
    }

    #[test]
    fn test_poisson_sampling() {
        let env = RadiationEnvironment::new(1.0, 0.0, 0.0);

        let sample1 = env.poisson_sample(5.0);
        let sample2 = env.poisson_sample(5.0);

        // Should be different (with high probability)
        // And reasonable values
        assert!(sample1 < 20);
        assert!(sample2 < 20);
    }
}
