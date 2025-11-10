//! Basic synapse simulation example.
//!
//! This example demonstrates:
//! - Creating excitatory and inhibitory synapses
//! - Simulating synaptic transmission
//! - Observing short-term plasticity
//! - STDP learning

use synapse_models::{Synapse, SynapseError};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Synapse Models Library Example ===\n");

    // Example 1: Basic synaptic transmission
    println!("1. Basic Synaptic Transmission");
    basic_transmission()?;

    // Example 2: Short-term depression
    println!("\n2. Short-Term Depression");
    short_term_depression()?;

    // Example 3: STDP learning
    println!("\n3. Spike-Timing Dependent Plasticity (STDP)");
    stdp_learning()?;

    // Example 4: Voltage dependence of NMDA receptors
    println!("\n4. NMDA Voltage Dependence");
    nmda_voltage_dependence()?;

    Ok(())
}

fn basic_transmission() -> Result<(), SynapseError> {
    let mut synapse = Synapse::excitatory(1.0, 1.0)?;

    println!("Created excitatory synapse with weight {} and delay {} ms",
             synapse.weight, synapse.delay);

    // Presynaptic spike at t=0
    synapse.presynaptic_spike(0.0)?;
    println!("Presynaptic spike at t=0 ms");

    // Simulate for 20 ms
    println!("\nTime (ms) | Conductance (nS) | Current (pA) @ -65mV");
    println!("----------|------------------|---------------------");

    for t in 0..200 {
        let time = t as f64 * 0.1;
        synapse.update(time, -65.0, 0.1)?;

        if t % 10 == 0 {
            let g = synapse.conductance();
            let i = synapse.current(-65.0);
            println!("{:8.1}  | {:16.4} | {:19.4}", time, g, i);
        }
    }

    Ok(())
}

fn short_term_depression() -> Result<(), SynapseError> {
    let mut synapse = Synapse::depressing_excitatory(1.0, 1.0)?;

    println!("Created depressing synapse");
    println!("\nSpike # | Release Probability | Available Vesicles");
    println!("--------|--------------------|-----------------");

    for spike_num in 1..=10 {
        let time = (spike_num - 1) as f64 * 20.0;

        let prob_before = synapse.vesicle_pool.release_probability();
        let avail_before = synapse.vesicle_pool.available_fraction;

        synapse.presynaptic_spike(time)?;

        // Update for 20 ms
        for _ in 0..200 {
            synapse.update(time + 0.1, -65.0, 0.1)?;
        }

        println!("{:7}  | {:18.4} | {:16.4}",
                 spike_num, prob_before, avail_before);
    }

    println!("\nNote: Release probability and available vesicles decrease with rapid firing");

    Ok(())
}

fn stdp_learning() -> Result<(), SynapseError> {
    println!("Testing STDP learning window");
    println!("\nΔt (ms) | Weight Change");
    println!("--------|-------------");

    // Test different spike timing offsets
    for dt in &[-40, -20, -10, -5, 0, 5, 10, 20, 40] {
        let mut synapse = Synapse::excitatory(0.5, 1.0)?;
        let initial_weight = synapse.weight;

        if *dt < 0 {
            // Post before pre (depression)
            synapse.postsynaptic_spike(0.0)?;
            synapse.presynaptic_spike((-dt) as f64)?;
        } else {
            // Pre before post (potentiation)
            synapse.presynaptic_spike(0.0)?;
            synapse.postsynaptic_spike(*dt as f64)?;
        }

        let weight_change = synapse.weight - initial_weight;
        println!("{:7}  | {:+11.6}", dt, weight_change);
    }

    println!("\nPositive Δt (pre before post) → Potentiation (LTP)");
    println!("Negative Δt (post before pre) → Depression (LTD)");

    Ok(())
}

fn nmda_voltage_dependence() -> Result<(), SynapseError> {
    use synapse_models::receptor::{NMDAReceptor, ReceptorDynamics};

    let mut nmda = NMDAReceptor::new();

    println!("NMDA receptor Mg2+ block at different voltages");
    println!("\nActivating with 1 mM glutamate for 5 ms...\n");

    // Activate receptor
    for _ in 0..50 {
        nmda.update(1.0, -65.0, 0.1).unwrap();
    }

    println!("Voltage (mV) | Mg2+ Block | Effective Conductance");
    println!("-------------|------------|--------------------");

    for v in &[-80, -70, -60, -50, -40, -30, -20, -10, 0, 10, 20, 40] {
        let block = nmda.mg_block(*v as f64);
        let g = nmda.get_conductance();
        let g_eff = g * block;
        println!("{:12} | {:10.4} | {:18.4}", v, block, g_eff);
    }

    println!("\nNote: NMDA channels are blocked by Mg2+ at hyperpolarized potentials");
    println!("      Block is relieved by depolarization, enabling Ca2+ influx");

    Ok(())
}
