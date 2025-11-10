//! Synaptic plasticity rules for learning and memory.
//!
//! This module implements various plasticity mechanisms including:
//! - Spike-Timing Dependent Plasticity (STDP)
//! - BCM (Bienenstock-Cooper-Munro) rule
//! - Oja's rule
//! - Hebbian and Anti-Hebbian learning
//! - Homeostatic plasticity
//! - Meta-plasticity

use crate::error::{Result, SynapseError};

/// Spike-Timing Dependent Plasticity (STDP) implementation.
///
/// STDP is a biological learning rule where synaptic strength changes depend
/// on the relative timing of pre- and postsynaptic spikes.
///
/// Δw = A+ * exp(-Δt/τ+) for Δt > 0 (pre before post, potentiation)
/// Δw = -A- * exp(Δt/τ-) for Δt < 0 (post before pre, depression)
#[derive(Debug, Clone)]
pub struct STDP {
    /// Amplitude of potentiation.
    pub a_plus: f64,

    /// Amplitude of depression.
    pub a_minus: f64,

    /// Time constant for potentiation (ms).
    pub tau_plus: f64,

    /// Time constant for depression (ms).
    pub tau_minus: f64,

    /// Minimum synaptic weight.
    pub w_min: f64,

    /// Maximum synaptic weight.
    pub w_max: f64,

    /// Whether to use multiplicative updates (vs additive).
    pub multiplicative: bool,

    /// Last presynaptic spike time (ms).
    last_pre_spike: Option<f64>,

    /// Last postsynaptic spike time (ms).
    last_post_spike: Option<f64>,

    /// Accumulated weight change.
    pub accumulated_dw: f64,
}

impl Default for STDP {
    fn default() -> Self {
        Self {
            a_plus: 0.01,
            a_minus: 0.01,
            tau_plus: 20.0,
            tau_minus: 20.0,
            w_min: 0.0,
            w_max: 1.0,
            multiplicative: false,
            last_pre_spike: None,
            last_post_spike: None,
            accumulated_dw: 0.0,
        }
    }
}

impl STDP {
    /// Create new STDP with default parameters.
    pub fn new() -> Self {
        Self::default()
    }

    /// Create STDP with custom parameters.
    pub fn with_params(a_plus: f64, a_minus: f64, tau_plus: f64, tau_minus: f64) -> Result<Self> {
        if tau_plus <= 0.0 || tau_minus <= 0.0 {
            return Err(SynapseError::InvalidTimeConstant(tau_plus.min(tau_minus)));
        }

        Ok(Self {
            a_plus,
            a_minus,
            tau_plus,
            tau_minus,
            ..Self::default()
        })
    }

    /// Create STDP with multiplicative updates (weight-dependent).
    pub fn multiplicative(mut self) -> Self {
        self.multiplicative = true;
        self
    }

    /// Register presynaptic spike and calculate weight change.
    ///
    /// # Arguments
    /// * `time` - Current time (ms)
    /// * `current_weight` - Current synaptic weight
    ///
    /// # Returns
    /// Weight change (Δw)
    pub fn pre_spike(&mut self, time: f64, current_weight: f64) -> f64 {
        let mut dw = 0.0;

        // If there was a recent postsynaptic spike, apply depression
        if let Some(post_time) = self.last_post_spike {
            let dt = time - post_time;
            if dt > 0.0 && dt < 5.0 * self.tau_minus {
                dw = -self.a_minus * (-dt / self.tau_minus).exp();

                // Multiplicative depression: Δw ∝ w
                if self.multiplicative {
                    dw *= current_weight;
                }
            }
        }

        self.last_pre_spike = Some(time);
        self.accumulated_dw += dw;
        dw
    }

    /// Register postsynaptic spike and calculate weight change.
    ///
    /// # Arguments
    /// * `time` - Current time (ms)
    /// * `current_weight` - Current synaptic weight
    ///
    /// # Returns
    /// Weight change (Δw)
    pub fn post_spike(&mut self, time: f64, current_weight: f64) -> f64 {
        let mut dw = 0.0;

        // If there was a recent presynaptic spike, apply potentiation
        if let Some(pre_time) = self.last_pre_spike {
            let dt = time - pre_time;
            if dt > 0.0 && dt < 5.0 * self.tau_plus {
                dw = self.a_plus * (-dt / self.tau_plus).exp();

                // Multiplicative potentiation: Δw ∝ (w_max - w)
                if self.multiplicative {
                    dw *= self.w_max - current_weight;
                }
            }
        }

        self.last_post_spike = Some(time);
        self.accumulated_dw += dw;
        dw
    }

    /// Apply accumulated weight change to synaptic weight.
    ///
    /// # Arguments
    /// * `weight` - Current synaptic weight
    ///
    /// # Returns
    /// New synaptic weight
    pub fn apply_update(&mut self, weight: f64) -> f64 {
        let new_weight = (weight + self.accumulated_dw).clamp(self.w_min, self.w_max);
        self.accumulated_dw = 0.0;
        new_weight
    }

    /// Calculate STDP window function for a given time difference.
    ///
    /// # Arguments
    /// * `dt` - Time difference (post - pre) in ms
    pub fn window(&self, dt: f64) -> f64 {
        if dt > 0.0 {
            self.a_plus * (-dt / self.tau_plus).exp()
        } else {
            -self.a_minus * (dt / self.tau_minus).exp()
        }
    }

    /// Reset STDP state.
    pub fn reset(&mut self) {
        self.last_pre_spike = None;
        self.last_post_spike = None;
        self.accumulated_dw = 0.0;
    }
}

/// BCM (Bienenstock-Cooper-Munro) plasticity rule.
///
/// BCM theory proposes that synaptic modification depends on postsynaptic
/// activity relative to a sliding threshold.
///
/// Δw = η * x * (y - θ) * y
/// where x = presynaptic activity, y = postsynaptic activity, θ = threshold
#[derive(Debug, Clone)]
pub struct BCM {
    /// Learning rate.
    pub learning_rate: f64,

    /// Modification threshold.
    pub threshold: f64,

    /// Time constant for threshold adaptation (ms).
    pub tau_threshold: f64,

    /// Average postsynaptic activity (for threshold update).
    avg_post_activity: f64,

    /// Minimum weight.
    pub w_min: f64,

    /// Maximum weight.
    pub w_max: f64,
}

impl Default for BCM {
    fn default() -> Self {
        Self {
            learning_rate: 0.001,
            threshold: 0.5,
            tau_threshold: 10000.0, // Slow adaptation
            avg_post_activity: 0.0,
            w_min: 0.0,
            w_max: 1.0,
        }
    }
}

impl BCM {
    /// Create new BCM with default parameters.
    pub fn new() -> Self {
        Self::default()
    }

    /// Update synaptic weight using BCM rule.
    ///
    /// # Arguments
    /// * `pre_activity` - Presynaptic activity (firing rate or activation)
    /// * `post_activity` - Postsynaptic activity
    /// * `current_weight` - Current synaptic weight
    /// * `dt` - Time step (ms)
    pub fn update(&mut self, pre_activity: f64, post_activity: f64, current_weight: f64, dt: f64) -> f64 {
        // BCM rule: Δw = η * x * (y - θ) * y
        let dw = self.learning_rate * pre_activity * (post_activity - self.threshold) * post_activity * dt;

        // Update threshold based on average postsynaptic activity
        self.avg_post_activity += (post_activity - self.avg_post_activity) / self.tau_threshold * dt;
        self.threshold = self.avg_post_activity * self.avg_post_activity;

        (current_weight + dw).clamp(self.w_min, self.w_max)
    }

    /// Reset BCM state.
    pub fn reset(&mut self) {
        self.threshold = 0.5;
        self.avg_post_activity = 0.0;
    }
}

/// Oja's rule for normalized Hebbian learning.
///
/// Oja's rule prevents unbounded weight growth through normalization.
///
/// Δw = η * (y * x - y² * w)
#[derive(Debug, Clone)]
pub struct OjasRule {
    /// Learning rate.
    pub learning_rate: f64,

    /// Minimum weight.
    pub w_min: f64,

    /// Maximum weight.
    pub w_max: f64,
}

impl Default for OjasRule {
    fn default() -> Self {
        Self {
            learning_rate: 0.001,
            w_min: 0.0,
            w_max: 1.0,
        }
    }
}

impl OjasRule {
    /// Create new Oja's rule with default parameters.
    pub fn new() -> Self {
        Self::default()
    }

    /// Update synaptic weight using Oja's rule.
    ///
    /// # Arguments
    /// * `pre_activity` - Presynaptic activity
    /// * `post_activity` - Postsynaptic activity
    /// * `current_weight` - Current synaptic weight
    /// * `dt` - Time step (ms)
    pub fn update(&mut self, pre_activity: f64, post_activity: f64, current_weight: f64, dt: f64) -> f64 {
        // Oja's rule: Δw = η * (y * x - y² * w)
        let dw = self.learning_rate * (post_activity * pre_activity - post_activity * post_activity * current_weight) * dt;

        (current_weight + dw).clamp(self.w_min, self.w_max)
    }
}

/// Hebbian learning rule.
///
/// Classic Hebbian learning: "Cells that fire together, wire together."
///
/// Δw = η * x * y
#[derive(Debug, Clone)]
pub struct HebbianRule {
    /// Learning rate.
    pub learning_rate: f64,

    /// Whether to normalize weights.
    pub normalize: bool,

    /// Minimum weight.
    pub w_min: f64,

    /// Maximum weight.
    pub w_max: f64,
}

impl Default for HebbianRule {
    fn default() -> Self {
        Self {
            learning_rate: 0.001,
            normalize: false,
            w_min: 0.0,
            w_max: 1.0,
        }
    }
}

impl HebbianRule {
    /// Create new Hebbian rule with default parameters.
    pub fn new() -> Self {
        Self::default()
    }

    /// Create normalized Hebbian rule.
    pub fn normalized(mut self) -> Self {
        self.normalize = true;
        self
    }

    /// Update synaptic weight using Hebbian rule.
    ///
    /// # Arguments
    /// * `pre_activity` - Presynaptic activity
    /// * `post_activity` - Postsynaptic activity
    /// * `current_weight` - Current synaptic weight
    /// * `dt` - Time step (ms)
    pub fn update(&mut self, pre_activity: f64, post_activity: f64, current_weight: f64, dt: f64) -> f64 {
        let dw = if self.normalize {
            // Normalized: Δw = η * (x * y - w * y²)
            self.learning_rate * (pre_activity * post_activity - current_weight * post_activity.powi(2)) * dt
        } else {
            // Standard: Δw = η * x * y
            self.learning_rate * pre_activity * post_activity * dt
        };

        (current_weight + dw).clamp(self.w_min, self.w_max)
    }
}

/// Anti-Hebbian learning rule.
///
/// Opposite of Hebbian learning, weakens connections between co-active neurons.
///
/// Δw = -η * x * y
#[derive(Debug, Clone)]
pub struct AntiHebbianRule {
    /// Learning rate.
    pub learning_rate: f64,

    /// Minimum weight.
    pub w_min: f64,

    /// Maximum weight.
    pub w_max: f64,
}

impl Default for AntiHebbianRule {
    fn default() -> Self {
        Self {
            learning_rate: 0.001,
            w_min: 0.0,
            w_max: 1.0,
        }
    }
}

impl AntiHebbianRule {
    /// Create new Anti-Hebbian rule with default parameters.
    pub fn new() -> Self {
        Self::default()
    }

    /// Update synaptic weight using Anti-Hebbian rule.
    pub fn update(&mut self, pre_activity: f64, post_activity: f64, current_weight: f64, dt: f64) -> f64 {
        let dw = -self.learning_rate * pre_activity * post_activity * dt;
        (current_weight + dw).clamp(self.w_min, self.w_max)
    }
}

/// Homeostatic plasticity for maintaining stable activity levels.
///
/// Scales synaptic weights to maintain target firing rate.
#[derive(Debug, Clone)]
pub struct HomeostaticPlasticity {
    /// Target firing rate (Hz).
    pub target_rate: f64,

    /// Time constant for homeostatic adjustment (ms).
    pub tau_homeostatic: f64,

    /// Current average firing rate (Hz).
    avg_rate: f64,

    /// Scaling factor.
    pub scaling_factor: f64,
}

impl Default for HomeostaticPlasticity {
    fn default() -> Self {
        Self {
            target_rate: 5.0,           // 5 Hz target
            tau_homeostatic: 1000000.0, // Very slow (hours)
            avg_rate: 5.0,
            scaling_factor: 1.0,
        }
    }
}

impl HomeostaticPlasticity {
    /// Create new homeostatic plasticity with default parameters.
    pub fn new() -> Self {
        Self::default()
    }

    /// Update homeostatic scaling based on current activity.
    ///
    /// # Arguments
    /// * `current_rate` - Current firing rate (Hz)
    /// * `dt` - Time step (ms)
    pub fn update(&mut self, current_rate: f64, dt: f64) {
        // Update average rate
        self.avg_rate += (current_rate - self.avg_rate) / self.tau_homeostatic * dt;

        // Update scaling factor
        // If rate too high, decrease weights; if too low, increase weights
        let rate_error = self.target_rate - self.avg_rate;
        self.scaling_factor += rate_error / self.target_rate / self.tau_homeostatic * dt;
        self.scaling_factor = self.scaling_factor.max(0.1).min(10.0);
    }

    /// Apply homeostatic scaling to synaptic weight.
    pub fn apply_scaling(&self, weight: f64) -> f64 {
        weight * self.scaling_factor
    }

    /// Reset homeostatic state.
    pub fn reset(&mut self) {
        self.avg_rate = self.target_rate;
        self.scaling_factor = 1.0;
    }
}

/// Meta-plasticity: plasticity of plasticity.
///
/// Learning rate adapts based on recent synaptic activity.
#[derive(Debug, Clone)]
pub struct MetaPlasticity {
    /// Base learning rate.
    pub base_learning_rate: f64,

    /// Current learning rate (modulated).
    pub learning_rate: f64,

    /// Time constant for meta-plasticity (ms).
    pub tau_meta: f64,

    /// Average synaptic activity.
    avg_activity: f64,

    /// Threshold for meta-plasticity.
    pub activity_threshold: f64,
}

impl Default for MetaPlasticity {
    fn default() -> Self {
        Self {
            base_learning_rate: 0.01,
            learning_rate: 0.01,
            tau_meta: 100000.0, // Slow time scale
            avg_activity: 0.0,
            activity_threshold: 0.5,
        }
    }
}

impl MetaPlasticity {
    /// Create new meta-plasticity with default parameters.
    pub fn new() -> Self {
        Self::default()
    }

    /// Update meta-plasticity based on synaptic activity.
    ///
    /// # Arguments
    /// * `activity` - Current synaptic activity level
    /// * `dt` - Time step (ms)
    pub fn update(&mut self, activity: f64, dt: f64) {
        // Update average activity
        self.avg_activity += (activity - self.avg_activity) / self.tau_meta * dt;

        // Modulate learning rate based on activity
        // High activity -> lower learning rate (homeostatic)
        // Low activity -> higher learning rate
        let modulation = if self.avg_activity > self.activity_threshold {
            0.5 // Reduce learning rate
        } else {
            2.0 // Increase learning rate
        };

        self.learning_rate = self.base_learning_rate * modulation;
    }

    /// Get current learning rate.
    pub fn get_learning_rate(&self) -> f64 {
        self.learning_rate
    }

    /// Reset meta-plasticity state.
    pub fn reset(&mut self) {
        self.learning_rate = self.base_learning_rate;
        self.avg_activity = 0.0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stdp_creation() {
        let stdp = STDP::new();
        assert_eq!(stdp.a_plus, 0.01);
        assert_eq!(stdp.a_minus, 0.01);
    }

    #[test]
    fn test_stdp_potentiation() {
        let mut stdp = STDP::new();
        let weight = 0.5;

        // Pre spike at t=0
        stdp.pre_spike(0.0, weight);

        // Post spike at t=10 (pre before post -> potentiation)
        let dw = stdp.post_spike(10.0, weight);

        assert!(dw > 0.0); // Should potentiate
    }

    #[test]
    fn test_stdp_depression() {
        let mut stdp = STDP::new();
        let weight = 0.5;

        // Post spike at t=0
        stdp.post_spike(0.0, weight);

        // Pre spike at t=10 (post before pre -> depression)
        let dw = stdp.pre_spike(10.0, weight);

        assert!(dw < 0.0); // Should depress
    }

    #[test]
    fn test_stdp_window() {
        let stdp = STDP::new();

        let pot = stdp.window(10.0);  // Potentiation
        let dep = stdp.window(-10.0); // Depression

        assert!(pot > 0.0);
        assert!(dep < 0.0);
    }

    #[test]
    fn test_bcm_rule() {
        let mut bcm = BCM::new();
        let weight = 0.5;

        // Low postsynaptic activity -> depression
        let w1 = bcm.update(1.0, 0.1, weight, 1.0);
        assert!(w1 < weight);

        // High postsynaptic activity -> potentiation
        let w2 = bcm.update(1.0, 0.9, weight, 1.0);
        assert!(w2 > weight);
    }

    #[test]
    fn test_ojas_rule() {
        let mut oja = OjasRule::new();
        let weight = 0.5;

        let new_weight = oja.update(1.0, 1.0, weight, 1.0);
        assert!(new_weight >= 0.0 && new_weight <= 1.0);
    }

    #[test]
    fn test_hebbian_rule() {
        let mut hebb = HebbianRule::new();
        let weight = 0.5;

        // Both active -> strengthen
        let new_weight = hebb.update(1.0, 1.0, weight, 1.0);
        assert!(new_weight > weight);
    }

    #[test]
    fn test_anti_hebbian_rule() {
        let mut anti = AntiHebbianRule::new();
        let weight = 0.5;

        // Both active -> weaken
        let new_weight = anti.update(1.0, 1.0, weight, 1.0);
        assert!(new_weight < weight);
    }

    #[test]
    fn test_homeostatic_plasticity() {
        let mut homeo = HomeostaticPlasticity::new();

        // High activity should reduce scaling
        for _ in 0..100 {
            homeo.update(10.0, 100.0); // 10 Hz, higher than target
        }
        assert!(homeo.scaling_factor < 1.0);

        homeo.reset();

        // Low activity should increase scaling
        for _ in 0..100 {
            homeo.update(1.0, 100.0); // 1 Hz, lower than target
        }
        assert!(homeo.scaling_factor > 1.0);
    }

    #[test]
    fn test_meta_plasticity() {
        let mut meta = MetaPlasticity::new();

        // High activity should reduce learning rate (needs longer to accumulate)
        for _ in 0..1000 {
            meta.update(0.8, 100.0);
        }
        // After high activity, learning rate should be modulated down
        assert!(meta.avg_activity > meta.activity_threshold);
        assert!(meta.learning_rate < meta.base_learning_rate);

        meta.reset();

        // Low activity should increase learning rate
        for _ in 0..1000 {
            meta.update(0.2, 100.0);
        }
        assert!(meta.avg_activity < meta.activity_threshold);
        assert!(meta.learning_rate > meta.base_learning_rate);
    }
}
