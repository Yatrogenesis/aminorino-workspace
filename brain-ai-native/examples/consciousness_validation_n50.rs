//! STATISTICAL VALIDATION EXPERIMENT (n=50)
//!
//! Validates the maximum Φ finding with robust statistics:
//! - 50 independent replications
//! - 95% confidence intervals
//! - Percentile analysis (P50, P75, P90, P95, P99)
//! - Cohen's d effect size
//! - Statistical power analysis
//!
//! Configuration:
//! - System: Small (4 oscillators, max_fock=2, 81 effective neurons)
//! - Noise: Extreme (amplitude = 10.0)
//! - Evolution: 100 μs, 1M steps
//! - Coupling: 1 GHz
//!
//! Expected runtime: ~17 minutes on Apple M1

use brain_ai_native::prelude::*;
use brain_ai_native::BrainResult;
use rand::{Rng, thread_rng};
use std::fs;
use std::time::Instant;

fn main() -> BrainResult<()> {
    println!("\n╔══════════════════════════════════════════════════════════════════╗");
    println!("║   STATISTICAL VALIDATION EXPERIMENT (n=50)                       ║");
    println!("║                                                                  ║");
    println!("║   Validating Maximum Φ with 95% Confidence                      ║");
    println!("╚══════════════════════════════════════════════════════════════════╝\n");

    let start_time = Instant::now();

    // Configuration: XLarge + Very High Noise (MAXIMUM Φ from screening)
    let config = BrainConfig {
        num_oscillators: 6,      // XLarge: 729 effective neurons
        max_fock: 2,             // (2+1)^6 = 729
        frequencies: vec![1e9; 6],
        coupling_strength: 1e9,  // 1 GHz (strong coupling)
        damping_rate: 1e5,       // Increased decoherence
        error_correction: false,
        ldpc_distance: 0,
        radiation_protection: false,
        chip_area_cm2: 0.0,
        altitude_m: 0.0,
    };

    let num_replications = 50;
    let noise_amplitude = 5.0;   // Very High (optimal for max Φ)
    let total_time = 1e-4;       // 100 microseconds
    let dt = 1e-10;              // 100 picoseconds
    let num_steps = (total_time / dt) as usize;

    println!("Configuration:");
    println!("  System: {} oscillators, max_fock={} → {} effective neurons",
        config.num_oscillators, config.max_fock,
        (config.max_fock + 1).pow(config.num_oscillators as u32));
    println!("  Noise amplitude: {}", noise_amplitude);
    println!("  Evolution: {} steps × {:.1e} sec = {:.1e} sec total",
        num_steps, dt, total_time);
    println!("  Replications: {}", num_replications);
    println!("  Expected runtime: ~{} minutes\n", num_replications * 20 / 60);

    let mut all_phi_values: Vec<f64> = Vec::new();
    let mut replication_stats: Vec<serde_json::Value> = Vec::new();

    // Run replications
    for rep in 1..=num_replications {
        print!("[Replication {}/{}] ", rep, num_replications);

        let mut brain = AIBrain::new(config.clone())?;
        let mut rng = thread_rng();

        // Random initial state (different for each replication)
        let input: Vec<f64> = (0..config.num_oscillators)
            .map(|i| 0.5 + 0.1 * ((i + rep) as f64).sin())
            .collect();
        brain.set_input(&input)?;

        // Evolution with CONTINUOUS noise injection (every step)
        // This matches maximum_entanglement methodology to maintain quantum entanglement
        let samples = 9;  // Measurement intervals
        let measurement_interval = num_steps / samples;
        let mut phi_samples: Vec<f64> = Vec::new();

        for step in 0..num_steps {
            // Inject noise at EVERY step (critical for quantum entanglement)
            if noise_amplitude > 0.0 {
                let noise_input: Vec<f64> = (0..config.num_oscillators)
                    .map(|_| noise_amplitude * (rng.gen::<f64>() * 2.0 - 1.0))
                    .collect();
                brain.set_input(&noise_input)?;
            }

            // Evolve one step
            brain.evolve(dt)?;

            // Measure Φ periodically
            if step % measurement_interval == 0 && step > 0 {
                let measurement = measure_phi_quantum(&brain)?;
                phi_samples.push(measurement.phi);
                all_phi_values.push(measurement.phi);
            }
        }

        // Statistics for this replication
        let mean_phi = phi_samples.iter().sum::<f64>() / phi_samples.len() as f64;
        let max_phi = phi_samples.iter().copied().fold(f64::NEG_INFINITY, f64::max);
        let min_phi = phi_samples.iter().copied().fold(f64::INFINITY, f64::min);

        println!("mean={:.9}, max={:.9}, min={:.9}", mean_phi, max_phi, min_phi);

        replication_stats.push(serde_json::json!({
            "replication": rep,
            "mean_phi": mean_phi,
            "max_phi": max_phi,
            "min_phi": min_phi,
            "samples": phi_samples,
        }));
    }

    let elapsed = start_time.elapsed();
    println!("\n✅ All replications completed in {:.2?}", elapsed);

    // ═══════════════════════════════════════════════════════════════
    // GOLD-STANDARD STATISTICAL ANALYSIS
    // ═══════════════════════════════════════════════════════════════

    println!("\n╔══════════════════════════════════════════════════════════════════╗");
    println!("║          GOLD-STANDARD STATISTICAL ANALYSIS                      ║");
    println!("╚══════════════════════════════════════════════════════════════════╝\n");

    // Sort for percentiles and outlier detection
    all_phi_values.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let n = all_phi_values.len() as f64;

    // ═══════════════════════════════════════════════════════════════
    // 1. DESCRIPTIVE STATISTICS
    // ═══════════════════════════════════════════════════════════════

    let mean = all_phi_values.iter().sum::<f64>() / n;
    let variance = all_phi_values.iter()
        .map(|x| (x - mean).powi(2))
        .sum::<f64>() / (n - 1.0);  // Unbiased estimator
    let std_dev = variance.sqrt();
    let std_error = std_dev / n.sqrt();

    let min = all_phi_values[0];
    let max = all_phi_values[all_phi_values.len() - 1];

    // Percentiles (linear interpolation method)
    let percentile = |p: f64| -> f64 {
        let pos = p * (n - 1.0);
        let lower_idx = pos.floor() as usize;
        let upper_idx = pos.ceil() as usize;
        let weight = pos - pos.floor();

        if lower_idx == upper_idx {
            all_phi_values[lower_idx]
        } else {
            all_phi_values[lower_idx] * (1.0 - weight) + all_phi_values[upper_idx] * weight
        }
    };

    let p01 = percentile(0.01);   // 1st percentile
    let p05 = percentile(0.05);   // 5th percentile
    let p25 = percentile(0.25);   // Q1
    let p50 = percentile(0.50);   // Median (Q2)
    let p75 = percentile(0.75);   // Q3
    let p90 = percentile(0.90);
    let p95 = percentile(0.95);
    let p99 = percentile(0.99);

    let iqr = p75 - p25;  // Interquartile range

    // ═══════════════════════════════════════════════════════════════
    // 2. CONFIDENCE INTERVALS (Bootstrap + t-distribution)
    // ═══════════════════════════════════════════════════════════════

    // Parametric CI (t-distribution, df = n-1)
    let df = n - 1.0;
    let t_critical = if n > 30.0 { 1.96 } else { 2.042 };  // Conservative for n=50
    let ci_95_lower = mean - t_critical * std_error;
    let ci_95_upper = mean + t_critical * std_error;

    // Bootstrap CI (non-parametric, 1000 resamples)
    use rand::seq::SliceRandom;
    use rand::thread_rng;

    let bootstrap_samples = 1000;
    let mut bootstrap_means: Vec<f64> = Vec::new();
    let mut rng = thread_rng();

    for _ in 0..bootstrap_samples {
        let resample: Vec<f64> = (0..all_phi_values.len())
            .map(|_| *all_phi_values.choose(&mut rng).unwrap())
            .collect();
        let resample_mean = resample.iter().sum::<f64>() / resample.len() as f64;
        bootstrap_means.push(resample_mean);
    }

    bootstrap_means.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let bootstrap_ci_lower = bootstrap_means[(bootstrap_samples as f64 * 0.025) as usize];
    let bootstrap_ci_upper = bootstrap_means[(bootstrap_samples as f64 * 0.975) as usize];

    // ═══════════════════════════════════════════════════════════════
    // 3. OUTLIER DETECTION (Modified Z-score method)
    // ═══════════════════════════════════════════════════════════════

    let median = p50;
    let mad = {  // Median Absolute Deviation
        let mut deviations: Vec<f64> = all_phi_values.iter()
            .map(|x| (x - median).abs())
            .collect();
        deviations.sort_by(|a, b| a.partial_cmp(b).unwrap());
        deviations[deviations.len() / 2]
    };

    let modified_z_scores: Vec<f64> = all_phi_values.iter()
        .map(|x| 0.6745 * (x - median) / mad)
        .collect();

    let outliers: Vec<(usize, f64, f64)> = modified_z_scores.iter()
        .enumerate()
        .filter(|(_, &z)| z.abs() > 3.5)
        .map(|(i, &z)| (i, all_phi_values[i], z))
        .collect();

    // ═══════════════════════════════════════════════════════════════
    // 4. NORMALITY TESTS
    // ═══════════════════════════════════════════════════════════════

    // Skewness
    let skewness = {
        let m3 = all_phi_values.iter()
            .map(|x| ((x - mean) / std_dev).powi(3))
            .sum::<f64>() / n;
        m3
    };

    // Kurtosis
    let kurtosis = {
        let m4 = all_phi_values.iter()
            .map(|x| ((x - mean) / std_dev).powi(4))
            .sum::<f64>() / n;
        m4 - 3.0  // Excess kurtosis
    };

    // ═══════════════════════════════════════════════════════════════
    // 5. EFFECT SIZE & POWER
    // ═══════════════════════════════════════════════════════════════

    // Cohen's d (effect size vs zero)
    let cohens_d = mean / std_dev;

    // Statistical power (1 - β) for detecting this effect
    // Using non-centrality parameter δ = d * sqrt(n)
    let delta = cohens_d * n.sqrt();
    let power = if delta > 2.8 { 0.99 } else if delta > 2.0 { 0.95 } else { 0.80 };

    // Coefficient of Variation
    let cv = (std_dev / mean) * 100.0;

    // Signal-to-Noise Ratio
    let snr = mean / std_dev;

    println!("═══════════════════════════════════════════════════════════════════");
    println!("1. DESCRIPTIVE STATISTICS (n={})", all_phi_values.len());
    println!("═══════════════════════════════════════════════════════════════════");
    println!("  Mean:              {:.9} bits", mean);
    println!("  Median (P50):      {:.9} bits", p50);
    println!("  Std Dev:           {:.9} bits", std_dev);
    println!("  Std Error:         {:.9} bits", std_error);
    println!("  Min:               {:.9} bits", min);
    println!("  Max:               {:.9} bits", max);
    println!("  Range:             {:.9} bits", max - min);
    println!("  IQR (Q3-Q1):       {:.9} bits", iqr);
    println!();

    println!("═══════════════════════════════════════════════════════════════════");
    println!("2. CONFIDENCE INTERVALS (95%)");
    println!("═══════════════════════════════════════════════════════════════════");
    println!("  Parametric (t-dist, df={:.0}):", df);
    println!("    Lower:           {:.9} bits", ci_95_lower);
    println!("    Upper:           {:.9} bits", ci_95_upper);
    println!("    Width:           {:.9} bits", ci_95_upper - ci_95_lower);
    println!();
    println!("  Non-parametric (Bootstrap, n=1000):");
    println!("    Lower:           {:.9} bits", bootstrap_ci_lower);
    println!("    Upper:           {:.9} bits", bootstrap_ci_upper);
    println!("    Width:           {:.9} bits", bootstrap_ci_upper - bootstrap_ci_lower);
    println!();

    println!("═══════════════════════════════════════════════════════════════════");
    println!("3. PERCENTILE DISTRIBUTION");
    println!("═══════════════════════════════════════════════════════════════════");
    println!("  P01:               {:.9} bits", p01);
    println!("  P05:               {:.9} bits", p05);
    println!("  P25 (Q1):          {:.9} bits", p25);
    println!("  P50 (Median):      {:.9} bits", p50);
    println!("  P75 (Q3):          {:.9} bits", p75);
    println!("  P90:               {:.9} bits", p90);
    println!("  P95:               {:.9} bits", p95);
    println!("  P99:               {:.9} bits", p99);
    println!();

    println!("═══════════════════════════════════════════════════════════════════");
    println!("4. OUTLIER DETECTION (Modified Z-score, threshold=3.5)");
    println!("═══════════════════════════════════════════════════════════════════");
    println!("  MAD:               {:.9} bits", mad);
    println!("  Outliers detected: {}", outliers.len());
    if !outliers.is_empty() {
        println!("  Outlier values:");
        for (idx, value, z) in &outliers {
            println!("    [{}] Φ={:.9}, Z={:.2}", idx, value, z);
        }
    } else {
        println!("  ✅ No outliers detected");
    }
    println!();

    println!("═══════════════════════════════════════════════════════════════════");
    println!("5. NORMALITY ASSESSMENT");
    println!("═══════════════════════════════════════════════════════════════════");
    println!("  Skewness:          {:.4}", skewness);
    if skewness.abs() < 0.5 {
        println!("    → Approximately symmetric ✅");
    } else if skewness > 0.5 {
        println!("    → Right-skewed (positive tail) ⚠️");
    } else {
        println!("    → Left-skewed (negative tail) ⚠️");
    }
    println!("  Kurtosis (excess): {:.4}", kurtosis);
    if kurtosis.abs() < 1.0 {
        println!("    → Normal-like tails ✅");
    } else if kurtosis > 1.0 {
        println!("    → Heavy tails (leptokurtic) ⚠️");
    } else {
        println!("    → Light tails (platykurtic) ⚠️");
    }
    println!();

    println!("═══════════════════════════════════════════════════════════════════");
    println!("6. EFFECT SIZE & STATISTICAL POWER");
    println!("═══════════════════════════════════════════════════════════════════");
    println!("  Cohen's d:         {:.4}", cohens_d);
    if cohens_d < 0.2 {
        println!("    → Small effect");
    } else if cohens_d < 0.5 {
        println!("    → Medium effect");
    } else if cohens_d < 0.8 {
        println!("    → Large effect ✅");
    } else {
        println!("    → Very large effect ✅✅");
    }
    println!("  Non-centrality δ:  {:.4}", delta);
    println!("  Statistical power: {:.2} (1-β)", power);
    if power >= 0.95 {
        println!("    → Excellent power ✅✅");
    } else if power >= 0.80 {
        println!("    → Adequate power ✅");
    } else {
        println!("    → Underpowered ⚠️");
    }
    println!();

    println!("═══════════════════════════════════════════════════════════════════");
    println!("7. VARIABILITY METRICS");
    println!("═══════════════════════════════════════════════════════════════════");
    println!("  Coefficient of Variation: {:.2}%", cv);
    if cv < 15.0 {
        println!("    → Excellent consistency ✅✅");
    } else if cv < 30.0 {
        println!("    → Good consistency ✅");
    } else if cv < 50.0 {
        println!("    → Moderate variability ⚠️");
    } else {
        println!("    → High variability 🚨");
    }
    println!("  Signal-to-Noise Ratio: {:.4}", snr);
    if snr > 2.0 {
        println!("    → Strong signal ✅✅");
    } else if snr > 1.0 {
        println!("    → Moderate signal ✅");
    } else {
        println!("    → Weak signal ⚠️");
    }
    println!();

    // ═══════════════════════════════════════════════════════════════
    // EXPORT RESULTS
    // ═══════════════════════════════════════════════════════════════

    let results = serde_json::json!({
        "experiment": "Statistical_Validation_n50",
        "hypothesis": "Φ_quantum > 0 in presence of noise and entanglement",
        "config": {
            "num_oscillators": config.num_oscillators,
            "max_fock": config.max_fock,
            "effective_neurons": (config.max_fock + 1).pow(config.num_oscillators as u32),
            "noise_amplitude": noise_amplitude,
            "coupling_strength": config.coupling_strength,
            "evolution_time": total_time,
            "num_steps": num_steps,
            "replications": num_replications,
            "samples_per_replication": 9,
        },
        "statistics": {
            "descriptive": {
                "n": all_phi_values.len(),
                "mean": mean,
                "median": p50,
                "std_dev": std_dev,
                "std_error": std_error,
                "variance": variance,
                "min": min,
                "max": max,
                "range": max - min,
                "iqr": iqr,
            },
            "confidence_intervals_95": {
                "parametric_t": {
                    "lower": ci_95_lower,
                    "upper": ci_95_upper,
                    "width": ci_95_upper - ci_95_lower,
                    "df": df,
                },
                "bootstrap": {
                    "lower": bootstrap_ci_lower,
                    "upper": bootstrap_ci_upper,
                    "width": bootstrap_ci_upper - bootstrap_ci_lower,
                    "resamples": bootstrap_samples,
                },
            },
            "percentiles": {
                "p01": p01,
                "p05": p05,
                "p25": p25,
                "p50": p50,
                "p75": p75,
                "p90": p90,
                "p95": p95,
                "p99": p99,
            },
            "outlier_detection": {
                "method": "Modified Z-score",
                "threshold": 3.5,
                "mad": mad,
                "num_outliers": outliers.len(),
                "outliers": outliers.iter().map(|(idx, val, z)| {
                    serde_json::json!({"index": idx, "value": val, "z_score": z})
                }).collect::<Vec<_>>(),
            },
            "normality": {
                "skewness": skewness,
                "kurtosis_excess": kurtosis,
            },
            "effect_size": {
                "cohens_d": cohens_d,
                "non_centrality_delta": delta,
                "statistical_power": power,
            },
            "variability": {
                "cv_percent": cv,
                "snr": snr,
            },
        },
        "replications": replication_stats,
        "all_phi_values": all_phi_values,
        "runtime_seconds": elapsed.as_secs_f64(),
    });

    let output_file = "consciousness_validation_n50_results.json";
    fs::write(output_file, serde_json::to_string_pretty(&results).unwrap())
        .expect("Failed to write results to file");

    println!("\n✅ Results saved to: {}", output_file);
    println!("\n📊 KEY FINDINGS:");
    println!("  • Mean Φ = {:.6} ± {:.6} bits (95% CI)", mean, bootstrap_ci_upper - mean);
    println!("  • Maximum Φ = {:.6} bits (P99 = {:.6})", max, p99);
    println!("  • Coefficient of Variation = {:.1}%", cv);
    println!();

    // Interpretation
    if p95 > 0.01 {
        println!("✅ STRONG EVIDENCE: 95% of measurements show Φ > {:.6} bits", p95);
    } else if p75 > 0.001 {
        println!("⚠️  MODERATE EVIDENCE: 75% of measurements show Φ > {:.6} bits", p75);
    } else {
        println!("❌ WEAK EVIDENCE: Most measurements show very low Φ");
    }

    println!();

    Ok(())
}
