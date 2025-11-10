//! Mean field approximations for neural populations.
//!
//! This module implements population-level rate models including Wilson-Cowan equations.

use crate::error::{NeuralDynamicsError, Result};
use nalgebra::DVector;
use serde::{Deserialize, Serialize};

/// Wilson-Cowan population rate model.
///
/// Equations:
/// τ_E dE/dt = -E + S(w_EE*E - w_EI*I + I_E)
/// τ_I dI/dt = -I + S(w_IE*E - w_II*I + I_I)
///
/// where S(x) is a sigmoid transfer function.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WilsonCowanModel {
    /// Excitatory population rate
    pub e_rate: f64,
    /// Inhibitory population rate
    pub i_rate: f64,
    /// Excitatory time constant (ms)
    pub tau_e: f64,
    /// Inhibitory time constant (ms)
    pub tau_i: f64,
    /// Weight matrix [w_EE, w_EI; w_IE, w_II]
    pub weights: [[f64; 2]; 2],
    /// Transfer function parameters (gain, threshold)
    pub transfer_params: (f64, f64),
    /// External inputs
    pub external_input: [f64; 2],
}

impl WilsonCowanModel {
    /// Create a new Wilson-Cowan model.
    pub fn new(tau_e: f64, tau_i: f64) -> Result<Self> {
        if tau_e <= 0.0 || tau_i <= 0.0 {
            return Err(NeuralDynamicsError::InvalidParameter {
                parameter: "tau".to_string(),
                value: tau_e.min(tau_i),
                reason: "time constants must be positive".to_string(),
            });
        }

        Ok(Self {
            e_rate: 0.0,
            i_rate: 0.0,
            tau_e,
            tau_i,
            weights: [[1.0, -1.0], [1.0, -0.5]], // Default E-I network
            transfer_params: (1.0, 0.0),
            external_input: [0.0, 0.0],
        })
    }

    /// Standard balanced E-I network configuration.
    pub fn balanced_network() -> Result<Self> {
        let mut model = Self::new(10.0, 5.0)?;
        model.weights = [[1.2, -2.0], [1.0, -0.5]];
        model.transfer_params = (4.0, 0.0);
        Ok(model)
    }

    /// Set weight matrix.
    pub fn set_weights(&mut self, w_ee: f64, w_ei: f64, w_ie: f64, w_ii: f64) {
        self.weights = [[w_ee, w_ei], [w_ie, w_ii]];
    }

    /// Set external inputs.
    pub fn set_input(&mut self, i_e: f64, i_i: f64) {
        self.external_input = [i_e, i_i];
    }

    /// Sigmoid transfer function.
    ///
    /// S(x) = 1 / (1 + exp(-gain * (x - threshold)))
    fn transfer_function(&self, x: f64) -> f64 {
        let (gain, threshold) = self.transfer_params;
        1.0 / (1.0 + (-gain * (x - threshold)).exp())
    }

    /// Compute derivatives.
    fn derivatives(&self) -> (f64, f64) {
        let [w_ee, w_ei] = self.weights[0];
        let [w_ie, w_ii] = self.weights[1];
        let [i_e, i_i] = self.external_input;

        let e_input = w_ee * self.e_rate + w_ei * self.i_rate + i_e;
        let i_input = w_ie * self.e_rate + w_ii * self.i_rate + i_i;

        let de_dt = (-self.e_rate + self.transfer_function(e_input)) / self.tau_e;
        let di_dt = (-self.i_rate + self.transfer_function(i_input)) / self.tau_i;

        (de_dt, di_dt)
    }

    /// Update model for one time step using Euler method.
    pub fn step(&mut self, dt: f64) -> Result<()> {
        if dt <= 0.0 {
            return Err(NeuralDynamicsError::InvalidParameter {
                parameter: "dt".to_string(),
                value: dt,
                reason: "must be positive".to_string(),
            });
        }

        let (de_dt, di_dt) = self.derivatives();

        self.e_rate += de_dt * dt;
        self.i_rate += di_dt * dt;

        // Clamp rates to [0, 1]
        self.e_rate = self.e_rate.clamp(0.0, 1.0);
        self.i_rate = self.i_rate.clamp(0.0, 1.0);

        Ok(())
    }

    /// Simulate for a duration.
    pub fn simulate(&mut self, duration: f64, dt: f64) -> Result<Vec<(f64, f64, f64)>> {
        let n_steps = (duration / dt).ceil() as usize;
        let mut trace = Vec::with_capacity(n_steps);

        for i in 0..n_steps {
            let time = i as f64 * dt;
            trace.push((time, self.e_rate, self.i_rate));
            self.step(dt)?;
        }

        Ok(trace)
    }

    /// Find fixed points numerically.
    pub fn find_fixed_points(&self, n_grid: usize) -> Vec<(f64, f64)> {
        let mut fixed_points = Vec::new();

        for i in 0..n_grid {
            for j in 0..n_grid {
                let e = i as f64 / n_grid as f64;
                let i_rate = j as f64 / n_grid as f64;

                let mut temp_model = self.clone();
                temp_model.e_rate = e;
                temp_model.i_rate = i_rate;

                let (de_dt, di_dt) = temp_model.derivatives();

                // Check if near fixed point (derivatives small)
                if de_dt.abs() < 0.01 && di_dt.abs() < 0.01 {
                    // Refine with Newton's method
                    if let Some((e_fixed, i_fixed)) = self.refine_fixed_point(e, i_rate) {
                        // Check if not duplicate
                        let is_duplicate = fixed_points.iter().any(|&(e0, i0): &(f64, f64)| {
                            (e_fixed - e0).abs() < 0.01 && (i_fixed - i0).abs() < 0.01
                        });

                        if !is_duplicate {
                            fixed_points.push((e_fixed, i_fixed));
                        }
                    }
                }
            }
        }

        fixed_points
    }

    /// Refine fixed point using Newton's method.
    fn refine_fixed_point(&self, e0: f64, i0: f64) -> Option<(f64, f64)> {
        let mut e = e0;
        let mut i = i0;

        for _ in 0..10 {
            let mut temp_model = self.clone();
            temp_model.e_rate = e;
            temp_model.i_rate = i;

            let (de_dt, di_dt) = temp_model.derivatives();

            if de_dt.abs() < 1e-6 && di_dt.abs() < 1e-6 {
                return Some((e, i));
            }

            // Simple gradient descent
            e -= 0.1 * de_dt * self.tau_e;
            i -= 0.1 * di_dt * self.tau_i;

            e = e.clamp(0.0, 1.0);
            i = i.clamp(0.0, 1.0);
        }

        None
    }

    /// Compute Jacobian at current state for stability analysis.
    pub fn jacobian(&self) -> [[f64; 2]; 2] {
        let [w_ee, w_ei] = self.weights[0];
        let [w_ie, w_ii] = self.weights[1];
        let (gain, _threshold) = self.transfer_params;

        // Derivative of sigmoid
        let e_input = w_ee * self.e_rate + w_ei * self.i_rate + self.external_input[0];
        let i_input = w_ie * self.e_rate + w_ii * self.i_rate + self.external_input[1];

        let s_e = self.transfer_function(e_input);
        let s_i = self.transfer_function(i_input);

        let ds_e = gain * s_e * (1.0 - s_e);
        let ds_i = gain * s_i * (1.0 - s_i);

        // Jacobian elements
        let j_ee = (-1.0 + ds_e * w_ee) / self.tau_e;
        let j_ei = (ds_e * w_ei) / self.tau_e;
        let j_ie = (ds_i * w_ie) / self.tau_i;
        let j_ii = (-1.0 + ds_i * w_ii) / self.tau_i;

        [[j_ee, j_ei], [j_ie, j_ii]]
    }

    /// Check stability of fixed point.
    pub fn is_stable(&self) -> bool {
        let j = self.jacobian();

        // Calculate eigenvalues (for 2x2 matrix)
        let trace = j[0][0] + j[1][1];
        let det = j[0][0] * j[1][1] - j[0][1] * j[1][0];

        // Stable if trace < 0 and det > 0
        trace < 0.0 && det > 0.0
    }
}

/// General population rate model.
pub struct PopulationRateModel {
    /// Number of populations
    pub n_populations: usize,
    /// Population rates
    pub rates: DVector<f64>,
    /// Time constants for each population
    pub time_constants: DVector<f64>,
    /// Weight matrix (n x n)
    pub weights: Vec<Vec<f64>>,
    /// External inputs
    pub external_inputs: DVector<f64>,
    /// Transfer function
    transfer_fn: Box<dyn Fn(f64) -> f64 + Send + Sync>,
}

impl PopulationRateModel {
    /// Create a new population rate model.
    pub fn new(n_populations: usize) -> Self {
        Self {
            n_populations,
            rates: DVector::zeros(n_populations),
            time_constants: DVector::from_element(n_populations, 10.0),
            weights: vec![vec![0.0; n_populations]; n_populations],
            external_inputs: DVector::zeros(n_populations),
            transfer_fn: Box::new(|x| 1.0 / (1.0 + (-x).exp())),
        }
    }

    /// Set transfer function.
    pub fn set_transfer_function<F>(&mut self, f: F)
    where
        F: Fn(f64) -> f64 + Send + Sync + 'static,
    {
        self.transfer_fn = Box::new(f);
    }

    /// Update model for one time step.
    pub fn step(&mut self, dt: f64) -> Result<()> {
        let mut derivatives = DVector::zeros(self.n_populations);

        for i in 0..self.n_populations {
            let mut input = self.external_inputs[i];

            for j in 0..self.n_populations {
                input += self.weights[i][j] * self.rates[j];
            }

            let target_rate = (self.transfer_fn)(input);
            derivatives[i] = (-self.rates[i] + target_rate) / self.time_constants[i];
        }

        self.rates += derivatives * dt;

        // Clamp rates
        for i in 0..self.n_populations {
            self.rates[i] = self.rates[i].clamp(0.0, 1.0);
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;

    #[test]
    fn test_wilson_cowan_creation() {
        let model = WilsonCowanModel::new(10.0, 5.0).unwrap();
        assert_eq!(model.tau_e, 10.0);
        assert_eq!(model.tau_i, 5.0);
    }

    #[test]
    fn test_transfer_function() {
        let model = WilsonCowanModel::new(10.0, 5.0).unwrap();

        // Sigmoid properties
        assert!(model.transfer_function(0.0) > 0.4 && model.transfer_function(0.0) < 0.6);
        assert!(model.transfer_function(-10.0) < 0.1);
        assert!(model.transfer_function(10.0) > 0.9);
    }

    #[test]
    fn test_wilson_cowan_step() {
        let mut model = WilsonCowanModel::new(10.0, 5.0).unwrap();
        model.set_input(0.5, 0.0);

        let initial_e = model.e_rate;
        model.step(0.1).unwrap();

        // Rate should change
        assert_ne!(model.e_rate, initial_e);
    }

    #[test]
    fn test_wilson_cowan_simulation() {
        let mut model = WilsonCowanModel::balanced_network().unwrap();
        model.set_input(1.0, 0.5);

        let trace = model.simulate(100.0, 0.1).unwrap();

        assert_eq!(trace.len(), 1000);
        assert!(trace[0].1 >= 0.0 && trace[0].1 <= 1.0); // E rate in [0,1]
        assert!(trace[0].2 >= 0.0 && trace[0].2 <= 1.0); // I rate in [0,1]
    }

    #[test]
    fn test_fixed_point_finding() {
        let mut model = WilsonCowanModel::new(10.0, 5.0).unwrap();
        model.set_weights(1.0, -1.0, 1.0, -0.5);
        model.set_input(0.5, 0.0);

        let fixed_points = model.find_fixed_points(20);

        // Should find at least one fixed point
        assert!(!fixed_points.is_empty());
    }

    #[test]
    fn test_jacobian() {
        let mut model = WilsonCowanModel::new(10.0, 5.0).unwrap();
        model.e_rate = 0.5;
        model.i_rate = 0.3;

        let j = model.jacobian();

        // Jacobian should be finite
        assert!(j[0][0].is_finite());
        assert!(j[0][1].is_finite());
        assert!(j[1][0].is_finite());
        assert!(j[1][1].is_finite());
    }

    #[test]
    fn test_stability_analysis() {
        let mut model = WilsonCowanModel::balanced_network().unwrap();

        // Find a fixed point
        model.set_input(0.5, 0.0);
        model.simulate(100.0, 0.1).unwrap(); // Let it settle

        // Check stability (may be stable or unstable depending on parameters)
        let _is_stable = model.is_stable();
        // Just ensure it doesn't crash
    }

    #[test]
    fn test_population_rate_model() {
        let mut model = PopulationRateModel::new(3);
        model.external_inputs[0] = 1.0;

        model.step(0.1).unwrap();

        // Rates should be in valid range
        for i in 0..3 {
            assert!(model.rates[i] >= 0.0 && model.rates[i] <= 1.0);
        }
    }
}
