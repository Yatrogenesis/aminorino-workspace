//! External stimulation patterns for neural populations.

use crate::error::{NeuralDynamicsError, Result};
use rand_distr::{Distribution, Exp, Normal};
use serde::{Deserialize, Serialize};

/// External stimulation protocol.
pub trait Stimulation: Send + Sync {
    /// Get stimulation current for a neuron at a given time.
    fn current(&mut self, neuron_idx: usize, time: f64, dt: f64) -> f64;

    /// Reset stimulation state.
    fn reset(&mut self);
}

/// Constant current injection.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CurrentInjection {
    /// Current amplitude (µA/cm²)
    pub amplitude: f64,
    /// Start time (ms)
    pub start_time: f64,
    /// End time (ms)
    pub end_time: f64,
    /// Target neuron indices (None = all neurons)
    pub target_neurons: Option<Vec<usize>>,
}

impl CurrentInjection {
    pub fn new(amplitude: f64, start_time: f64, end_time: f64) -> Self {
        Self {
            amplitude,
            start_time,
            end_time,
            target_neurons: None,
        }
    }

    pub fn with_targets(mut self, targets: Vec<usize>) -> Self {
        self.target_neurons = Some(targets);
        self
    }
}

impl Stimulation for CurrentInjection {
    fn current(&mut self, neuron_idx: usize, time: f64, _dt: f64) -> f64 {
        if time < self.start_time || time > self.end_time {
            return 0.0;
        }

        if let Some(ref targets) = self.target_neurons {
            if targets.contains(&neuron_idx) {
                self.amplitude
            } else {
                0.0
            }
        } else {
            self.amplitude
        }
    }

    fn reset(&mut self) {}
}

/// Poisson spike train stimulation.
pub struct PoissonSpikeTrains {
    /// Mean firing rate (Hz)
    pub rate: f64,
    /// Synaptic weight
    pub weight: f64,
    /// Start time (ms)
    pub start_time: f64,
    /// End time (ms)
    pub end_time: f64,
    /// Last spike times for each neuron
    last_spike_times: Vec<f64>,
    /// Inter-spike interval distribution
    isi_dist: Exp<f64>,
}

impl PoissonSpikeTrains {
    pub fn new(rate: f64, weight: f64, start_time: f64, end_time: f64, n_neurons: usize) -> Result<Self> {
        if rate <= 0.0 {
            return Err(NeuralDynamicsError::InvalidParameter {
                parameter: "rate".to_string(),
                value: rate,
                reason: "must be positive".to_string(),
            });
        }

        // Convert rate from Hz to ms^-1
        let lambda = rate / 1000.0;

        Ok(Self {
            rate,
            weight,
            start_time,
            end_time,
            last_spike_times: vec![start_time; n_neurons],
            isi_dist: Exp::new(lambda).map_err(|e| NeuralDynamicsError::InvalidParameter {
                parameter: "rate".to_string(),
                value: rate,
                reason: e.to_string(),
            })?,
        })
    }
}

impl Stimulation for PoissonSpikeTrains {
    fn current(&mut self, neuron_idx: usize, time: f64, dt: f64) -> f64 {
        if time < self.start_time || time > self.end_time {
            return 0.0;
        }

        let mut rng = rand::thread_rng();
        let mut current = 0.0;

        // Check if a spike should occur in this time step
        while self.last_spike_times[neuron_idx] < time {
            let isi = self.isi_dist.sample(&mut rng);
            self.last_spike_times[neuron_idx] += isi;

            if self.last_spike_times[neuron_idx] <= time && self.last_spike_times[neuron_idx] > time - dt {
                // Spike occurred in this time step
                current += self.weight / dt; // Impulse
            }
        }

        current
    }

    fn reset(&mut self) {
        self.last_spike_times.fill(self.start_time);
    }
}

/// Periodic sinusoidal stimulation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PeriodicStimulation {
    /// Base amplitude (µA/cm²)
    pub amplitude: f64,
    /// Frequency (Hz)
    pub frequency: f64,
    /// Phase offset (radians)
    pub phase: f64,
    /// DC offset (µA/cm²)
    pub offset: f64,
    /// Start time (ms)
    pub start_time: f64,
    /// End time (ms)
    pub end_time: f64,
}

impl PeriodicStimulation {
    pub fn new(amplitude: f64, frequency: f64) -> Self {
        Self {
            amplitude,
            frequency,
            phase: 0.0,
            offset: 0.0,
            start_time: 0.0,
            end_time: f64::INFINITY,
        }
    }
}

impl Stimulation for PeriodicStimulation {
    fn current(&mut self, _neuron_idx: usize, time: f64, _dt: f64) -> f64 {
        if time < self.start_time || time > self.end_time {
            return 0.0;
        }

        let omega = 2.0 * std::f64::consts::PI * self.frequency / 1000.0; // Convert Hz to ms^-1
        self.offset + self.amplitude * (omega * time + self.phase).sin()
    }

    fn reset(&mut self) {}
}

/// Noisy stimulation (Gaussian white noise).
pub struct NoisyStimulation {
    /// Mean current (µA/cm²)
    pub mean: f64,
    /// Standard deviation (µA/cm²)
    pub std: f64,
    /// Normal distribution
    dist: Normal<f64>,
}

impl NoisyStimulation {
    pub fn new(mean: f64, std: f64) -> Result<Self> {
        let dist = Normal::new(mean, std).map_err(|e| NeuralDynamicsError::InvalidParameter {
            parameter: "std".to_string(),
            value: std,
            reason: e.to_string(),
        })?;

        Ok(Self {
            mean,
            std,
            dist,
        })
    }
}

impl Stimulation for NoisyStimulation {
    fn current(&mut self, _neuron_idx: usize, _time: f64, _dt: f64) -> f64 {
        let mut rng = rand::thread_rng();
        self.dist.sample(&mut rng)
    }

    fn reset(&mut self) {
        // Nothing to reset
    }
}

/// Spatial pattern stimulation.
#[derive(Clone)]
pub struct SpatialPattern {
    /// Amplitude at each neuron
    pub pattern: Vec<f64>,
    /// Start time (ms)
    pub start_time: f64,
    /// End time (ms)
    pub end_time: f64,
}

impl SpatialPattern {
    pub fn new(pattern: Vec<f64>, start_time: f64, end_time: f64) -> Self {
        Self {
            pattern,
            start_time,
            end_time,
        }
    }

    /// Create Gaussian bump pattern.
    pub fn gaussian(n_neurons: usize, center: f64, sigma: f64, amplitude: f64) -> Self {
        let pattern: Vec<f64> = (0..n_neurons)
            .map(|i| {
                let x = i as f64 / n_neurons as f64;
                amplitude * (-(x - center).powi(2) / (2.0 * sigma * sigma)).exp()
            })
            .collect();

        Self {
            pattern,
            start_time: 0.0,
            end_time: f64::INFINITY,
        }
    }
}

impl Stimulation for SpatialPattern {
    fn current(&mut self, neuron_idx: usize, time: f64, _dt: f64) -> f64 {
        if time < self.start_time || time > self.end_time {
            return 0.0;
        }

        if neuron_idx < self.pattern.len() {
            self.pattern[neuron_idx]
        } else {
            0.0
        }
    }

    fn reset(&mut self) {}
}

/// Time-varying stimulation with arbitrary function.
pub struct TimeVaryingStimulation {
    /// Function defining current as f(time, neuron_idx)
    function: Box<dyn Fn(f64, usize) -> f64 + Send + Sync>,
}

impl TimeVaryingStimulation {
    pub fn new<F>(function: F) -> Self
    where
        F: Fn(f64, usize) -> f64 + Send + Sync + 'static,
    {
        Self {
            function: Box::new(function),
        }
    }
}

impl Stimulation for TimeVaryingStimulation {
    fn current(&mut self, neuron_idx: usize, time: f64, _dt: f64) -> f64 {
        (self.function)(time, neuron_idx)
    }

    fn reset(&mut self) {}
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;

    #[test]
    fn test_current_injection() {
        let mut stim = CurrentInjection::new(10.0, 100.0, 200.0);

        assert_eq!(stim.current(0, 50.0, 0.1), 0.0); // Before start
        assert_eq!(stim.current(0, 150.0, 0.1), 10.0); // During
        assert_eq!(stim.current(0, 250.0, 0.1), 0.0); // After end
    }

    #[test]
    fn test_current_injection_with_targets() {
        let mut stim = CurrentInjection::new(10.0, 0.0, 100.0).with_targets(vec![0, 2]);

        assert_eq!(stim.current(0, 50.0, 0.1), 10.0); // Target
        assert_eq!(stim.current(1, 50.0, 0.1), 0.0); // Non-target
        assert_eq!(stim.current(2, 50.0, 0.1), 10.0); // Target
    }

    #[test]
    fn test_periodic_stimulation() {
        let mut stim = PeriodicStimulation::new(5.0, 10.0); // 10 Hz

        let t1 = 0.0;
        let i1 = stim.current(0, t1, 0.1);
        assert_relative_eq!(i1, 0.0, epsilon = 0.1); // sin(0) = 0

        let t2 = 25.0; // Quarter period at 10 Hz (100 ms period)
        let i2 = stim.current(0, t2, 0.1);
        assert_relative_eq!(i2, 5.0, epsilon = 0.5); // sin(π/2) = 1
    }

    #[test]
    fn test_noisy_stimulation() {
        let mut stim = NoisyStimulation::new(5.0, 1.0).unwrap();

        // Generate many samples to test distribution
        let samples: Vec<f64> = (0..1000)
            .map(|_| stim.current(0, 0.0, 0.1))
            .collect();

        let mean = samples.iter().sum::<f64>() / samples.len() as f64;
        assert!(mean > 4.5 && mean < 5.5); // Close to expected mean
    }

    #[test]
    fn test_spatial_pattern() {
        let pattern = vec![1.0, 2.0, 3.0];
        let mut stim = SpatialPattern::new(pattern, 0.0, 100.0);

        assert_eq!(stim.current(0, 50.0, 0.1), 1.0);
        assert_eq!(stim.current(1, 50.0, 0.1), 2.0);
        assert_eq!(stim.current(2, 50.0, 0.1), 3.0);
        assert_eq!(stim.current(3, 50.0, 0.1), 0.0); // Out of range
    }

    #[test]
    fn test_gaussian_spatial_pattern() {
        let stim = SpatialPattern::gaussian(10, 0.5, 0.1, 10.0);

        // Check that center has maximum amplitude
        let mut max_idx = 0;
        let mut max_val = 0.0;
        for i in 0..10 {
            if stim.pattern[i] > max_val {
                max_val = stim.pattern[i];
                max_idx = i;
            }
        }

        // Center should be around index 5
        assert!(max_idx >= 4 && max_idx <= 6);
    }

    #[test]
    fn test_time_varying_stimulation() {
        let mut stim = TimeVaryingStimulation::new(|t, i| t * i as f64);

        assert_eq!(stim.current(0, 5.0, 0.1), 0.0);
        assert_eq!(stim.current(2, 5.0, 0.1), 10.0);
        assert_eq!(stim.current(3, 10.0, 0.1), 30.0);
    }

    #[test]
    fn test_poisson_spike_trains() {
        let stim = PoissonSpikeTrains::new(100.0, 1.0, 0.0, 1000.0, 10);
        assert!(stim.is_ok());

        // Invalid rate should fail
        let stim = PoissonSpikeTrains::new(-1.0, 1.0, 0.0, 1000.0, 10);
        assert!(stim.is_err());
    }
}
