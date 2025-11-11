//! CORTEXIA CLI - Quantum Consciousness Research Tool
//!
//! Professional command-line interface for running consciousness experiments

use brain_ai_native::prelude::*;
use brain_ai_native::{BrainResult, BrainError};
use clap::{Parser, Subcommand};
use std::path::PathBuf;
use std::fs;

#[derive(Parser)]
#[command(name = "cortexia")]
#[command(author = "Francisco Molina Burgos <fmolina@avermex.com>")]
#[command(version = "0.1.0")]
#[command(about = "Quantum Consciousness Research Platform", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,

    /// Enable verbose output
    #[arg(short, long)]
    verbose: bool,
}

#[derive(Subcommand)]
enum Commands {
    /// Run standard consciousness experiment (Φ_quantum vs Φ_classical)
    Experiment {
        /// Number of trials
        #[arg(short, long, default_value_t = 5)]
        trials: usize,

        /// Output file (JSON)
        #[arg(short, long, default_value = "consciousness_results.json")]
        output: PathBuf,
    },

    /// Run noise analysis experiment
    Noise {
        /// Maximum noise amplitude
        #[arg(short = 'a', long, default_value_t = 5.0)]
        max_amplitude: f64,

        /// Number of noise levels to test
        #[arg(short, long, default_value_t = 5)]
        levels: usize,

        /// Output file (JSON)
        #[arg(short, long, default_value = "noise_results.json")]
        output: PathBuf,
    },

    /// Run maximum entanglement experiment (intensive)
    Maximum {
        /// System size (oscillators)
        #[arg(short, long, default_value_t = 4)]
        size: usize,

        /// Maximum Fock state
        #[arg(short, long, default_value_t = 2)]
        fock: usize,

        /// Evolution time (seconds)
        #[arg(short = 't', long, default_value_t = 1e-4)]
        time: f64,

        /// Coupling strength (Hz)
        #[arg(short, long, default_value_t = 1e9)]
        coupling: f64,

        /// Output file (JSON)
        #[arg(short, long, default_value = "maximum_results.json")]
        output: PathBuf,
    },

    /// Measure Φ for custom configuration
    Measure {
        /// Number of oscillators
        #[arg(short = 'n', long, default_value_t = 4)]
        oscillators: usize,

        /// Maximum Fock state
        #[arg(short, long, default_value_t = 2)]
        fock: usize,

        /// Input values (comma-separated, e.g., "0.5,0.6,0.7,0.8")
        #[arg(short, long)]
        input: String,

        /// Evolution time (seconds)
        #[arg(short = 't', long, default_value_t = 1e-9)]
        time: f64,
    },

    /// Show system information
    Info,

    /// Validate installation
    Validate,
}

fn main() -> BrainResult<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Experiment { trials, output } => {
            run_standard_experiment(trials, &output, cli.verbose)
        }

        Commands::Noise { max_amplitude, levels, output } => {
            run_noise_experiment(max_amplitude, levels, &output, cli.verbose)
        }

        Commands::Maximum { size, fock, time, coupling, output } => {
            run_maximum_experiment(size, fock, time, coupling, &output, cli.verbose)
        }

        Commands::Measure { oscillators, fock, input, time } => {
            run_single_measurement(oscillators, fock, &input, time, cli.verbose)
        }

        Commands::Info => {
            show_system_info();
            Ok(())
        }

        Commands::Validate => {
            validate_installation()
        }
    }
}

fn run_standard_experiment(trials: usize, output: &PathBuf, verbose: bool) -> BrainResult<()> {
    println!("\n╔══════════════════════════════════════════════════════════════════╗");
    println!("║           STANDARD CONSCIOUSNESS EXPERIMENT                      ║");
    println!("╚══════════════════════════════════════════════════════════════════╝\n");

    println!("Configuration:");
    println!("  Trials: {}", trials);
    println!("  Output: {}", output.display());
    println!();

    // Run experiment
    let configs = vec![
        (2, 1),  // 2 oscillators, max_fock=1
        (3, 1),
        (4, 1),
    ];

    let mut results = Vec::new();

    for trial in 1..=trials {
        println!("[Trial {}/{}]", trial, trials);

        for (num_osc, max_fock) in &configs {
            if verbose {
                println!("  Testing {} oscillators, max_fock={}", num_osc, max_fock);
            }

            let config = BrainConfig {
                num_oscillators: *num_osc,
                max_fock: *max_fock,
                frequencies: vec![1e9; *num_osc],
                coupling_strength: 1e7,
                damping_rate: 1e3,
                error_correction: false,
                ldpc_distance: 0,
                radiation_protection: false,
                chip_area_cm2: 0.0,
                altitude_m: 0.0,
            };

            let mut brain = AIBrain::new(config)?;
            let input = vec![0.5; *num_osc];
            brain.set_input(&input)?;
            brain.evolve(1e-9)?;

            let phi = measure_phi_quantum(&brain)?;

            if verbose {
                println!("    Φ = {:.9}", phi.phi);
            }

            results.push(serde_json::json!({
                "trial": trial,
                "oscillators": num_osc,
                "max_fock": max_fock,
                "phi": phi.phi,
            }));
        }
    }

    // Export
    let export = serde_json::json!({
        "experiment": "Standard_Consciousness",
        "trials": trials,
        "results": results,
    });

    fs::write(output, serde_json::to_string_pretty(&export).unwrap())
        .map_err(|e| BrainError::ExperimentError(format!("{}", e)))?;

    println!("\n✅ Experiment complete!");
    println!("📊 Results saved to: {}", output.display());
    println!();

    Ok(())
}

fn run_noise_experiment(max_amp: f64, levels: usize, output: &PathBuf, verbose: bool) -> BrainResult<()> {
    println!("\n╔══════════════════════════════════════════════════════════════════╗");
    println!("║              NOISE ANALYSIS EXPERIMENT                           ║");
    println!("╚══════════════════════════════════════════════════════════════════╝\n");

    println!("Configuration:");
    println!("  Max amplitude: {}", max_amp);
    println!("  Levels: {}", levels);
    println!("  Output: {}", output.display());
    println!();

    // Implementation similar to consciousness_with_noise.rs
    println!("⚠️  Not yet implemented - use:");
    println!("  cargo run --release --example consciousness_with_noise");
    println!();

    Ok(())
}

fn run_maximum_experiment(size: usize, fock: usize, time: f64, coupling: f64, output: &PathBuf, verbose: bool) -> BrainResult<()> {
    println!("\n╔══════════════════════════════════════════════════════════════════╗");
    println!("║          MAXIMUM ENTANGLEMENT EXPERIMENT                         ║");
    println!("╚══════════════════════════════════════════════════════════════════╝\n");

    println!("Configuration:");
    println!("  System size: {} oscillators", size);
    println!("  Max Fock: {}", fock);
    println!("  Evolution time: {} s", time);
    println!("  Coupling: {} Hz", coupling);
    println!("  Output: {}", output.display());
    println!();

    let config = BrainConfig {
        num_oscillators: size,
        max_fock: fock,
        frequencies: vec![1e9; size],
        coupling_strength: coupling,
        damping_rate: 1e5,
        error_correction: false,
        ldpc_distance: 0,
        radiation_protection: false,
        chip_area_cm2: 0.0,
        altitude_m: 0.0,
    };

    let mut brain = AIBrain::new(config)?;
    let input = vec![0.5; size];
    brain.set_input(&input)?;

    println!("Running evolution...");
    brain.evolve(time)?;

    println!("Measuring Φ...");
    let phi = measure_phi_quantum(&brain)?;

    println!("\n📊 Results:");
    println!("  Φ = {:.9} bits", phi.phi);
    println!("  State space size: {}", phi.state_space_size);
    println!("  Partitions analyzed: {}", phi.num_partitions);
    println!();

    let export = serde_json::json!({
        "experiment": "Maximum_Entanglement",
        "config": {
            "oscillators": size,
            "max_fock": fock,
            "evolution_time": time,
            "coupling": coupling,
        },
        "phi": phi.phi,
        "state_space_size": phi.state_space_size,
        "num_partitions": phi.num_partitions,
    });

    fs::write(output, serde_json::to_string_pretty(&export).unwrap())
        .map_err(|e| BrainError::ExperimentError(format!("{}", e)))?;

    println!("✅ Results saved to: {}", output.display());
    println!();

    Ok(())
}

fn run_single_measurement(oscillators: usize, fock: usize, input_str: &str, time: f64, verbose: bool) -> BrainResult<()> {
    println!("\n╔══════════════════════════════════════════════════════════════════╗");
    println!("║               SINGLE Φ MEASUREMENT                               ║");
    println!("╚══════════════════════════════════════════════════════════════════╝\n");

    // Parse input
    let input: Result<Vec<f64>, _> = input_str.split(',')
        .map(|s| s.trim().parse())
        .collect();

    let input = input.map_err(|e| BrainError::InvalidConfig(format!("Invalid input: {}", e)))?;

    if input.len() != oscillators {
        return Err(BrainError::InvalidConfig(
            format!("Input length ({}) doesn't match oscillators ({})", input.len(), oscillators)
        ));
    }

    let config = BrainConfig {
        num_oscillators: oscillators,
        max_fock: fock,
        frequencies: vec![1e9; oscillators],
        coupling_strength: 1e7,
        damping_rate: 1e3,
        error_correction: false,
        ldpc_distance: 0,
        radiation_protection: false,
        chip_area_cm2: 0.0,
        altitude_m: 0.0,
    };

    if verbose {
        println!("Configuration:");
        println!("  Oscillators: {}", oscillators);
        println!("  Max Fock: {}", fock);
        println!("  Input: {:?}", input);
        println!("  Evolution time: {} s", time);
        println!();
    }

    let mut brain = AIBrain::new(config)?;
    brain.set_input(&input)?;
    brain.evolve(time)?;

    let phi = measure_phi_quantum(&brain)?;

    println!("📊 Integrated Information (Φ): {:.9} bits", phi.phi);
    println!();
    println!("Details:");
    println!("  State space size: {}", phi.state_space_size);
    println!("  Partitions analyzed: {}", phi.num_partitions);
    println!("  Method: {}", phi.method);

    if let Some(ref mip) = phi.mip {
        println!("  MIP subsets: {:?} | {:?}", mip.subset_a, mip.subset_b);
    }

    println!();

    Ok(())
}

fn show_system_info() {
    println!("\n╔══════════════════════════════════════════════════════════════════╗");
    println!("║                     CORTEXIA SYSTEM INFO                         ║");
    println!("╚══════════════════════════════════════════════════════════════════╝\n");

    println!("Version: {}", env!("CARGO_PKG_VERSION"));
    println!("Authors: {}", env!("CARGO_PKG_AUTHORS"));
    println!("Repository: {}", env!("CARGO_PKG_REPOSITORY"));
    println!();

    println!("Components:");
    println!("  • brain-ai-native: AI-Native quantum consciousness platform");
    println!("  • quantum-processor: Quantum reservoir computing");
    println!("  • iit: Integrated Information Theory implementation");
    println!("  • hodgkin-huxley: Neuron modeling");
    println!("  • neural-dynamics: Neural network dynamics");
    println!();

    println!("Capabilities:");
    println!("  ✓ Real quantum entanglement simulation");
    println!("  ✓ Integrated Information (Φ) measurement");
    println!("  ✓ LDPC error correction");
    println!("  ✓ Radiation environment modeling");
    println!("  ✓ Hodgkin-Huxley neuron integration");
    println!();

    println!("For more information:");
    println!("  https://github.com/Yatrogenesis/cortexia");
    println!();
}

fn validate_installation() -> BrainResult<()> {
    println!("\n╔══════════════════════════════════════════════════════════════════╗");
    println!("║              VALIDATING INSTALLATION                             ║");
    println!("╚══════════════════════════════════════════════════════════════════╝\n");

    print!("Checking quantum processor... ");
    let config = BrainConfig {
        num_oscillators: 2,
        max_fock: 1,
        frequencies: vec![1e9, 1e9],
        coupling_strength: 1e6,
        damping_rate: 1e3,
        error_correction: false,
        ldpc_distance: 0,
        radiation_protection: false,
        chip_area_cm2: 0.0,
        altitude_m: 0.0,
    };
    let brain = AIBrain::new(config)?;
    println!("✅");

    print!("Checking Φ measurement... ");
    let phi = measure_phi_quantum(&brain)?;
    println!("✅ (Φ = {:.9})", phi.phi);

    println!();
    println!("✅ Installation is valid!");
    println!();
    println!("Quick start:");
    println!("  cortexia measure -n 4 --input \"0.5,0.6,0.7,0.8\"");
    println!();

    Ok(())
}
