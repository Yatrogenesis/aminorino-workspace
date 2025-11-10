//! Network simulation example.
//!
//! This example demonstrates:
//! - Creating a small neural network
//! - Simulating spike propagation
//! - Excitatory-inhibitory balance
//! - Network statistics

use synapse_models::{Synapse, SynapticNetwork};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Neural Network Simulation ===\n");

    // Create a feedforward network
    feedforward_network()?;

    // Create a balanced excitatory-inhibitory network
    println!("\n");
    balanced_network()?;

    // Demonstrate gap junctions
    println!("\n");
    gap_junction_network()?;

    Ok(())
}

fn feedforward_network() -> Result<(), Box<dyn std::error::Error>> {
    println!("1. Feedforward Network (5 neurons)");

    let mut network = SynapticNetwork::new(5);

    // Create chain of excitatory connections
    for i in 0..4 {
        let syn = Synapse::excitatory(1.5, 2.0)?; // 2 ms delay
        network.add_connection(i, i + 1, syn)?;
    }

    let stats = network.connectivity_stats();
    println!("   Neurons: {}", stats.n_neurons);
    println!("   Connections: {}", stats.n_connections);
    println!("   Avg in-degree: {:.2}", stats.avg_in_degree);
    println!("   Avg out-degree: {:.2}", stats.avg_out_degree);

    // Spike the first neuron
    println!("\n   Spiking neuron 0 at t=0");
    network.spike(0)?;

    // Simulate
    let voltages = vec![-65.0; 5];
    println!("\n   Time (ms) | Neuron 0 | Neuron 1 | Neuron 2 | Neuron 3 | Neuron 4");
    println!   ("   ----------|----------|----------|----------|----------|----------");

    for t in 0..100 {
        let time = t as f64 * 0.5; // 0.5 ms steps
        network.update(&voltages, 0.5)?;

        if t % 4 == 0 {
            print!("   {:8.1}  |", time);
            for neuron_id in 0..5 {
                let current = network.get_synaptic_current(neuron_id)?;
                print!(" {:8.2} |", current);
            }
            println!();
        }
    }

    println!("\n   Note: Spike propagates through the chain with delays");

    Ok(())
}

fn balanced_network() -> Result<(), Box<dyn std::error::Error>> {
    println!("2. Balanced Excitatory-Inhibitory Network");

    let n_exc = 8;  // Excitatory neurons
    let n_inh = 2;  // Inhibitory neurons
    let n_total = n_exc + n_inh;

    let mut network = SynapticNetwork::new(n_total);

    // Excitatory neurons (0-7) connect to all with 30% probability
    let mut n_exc_conn = 0;
    for i in 0..n_exc {
        for j in 0..n_total {
            if i != j && rand::random::<f64>() < 0.3 {
                let syn = Synapse::excitatory(1.0, 1.0)?;
                network.add_connection(i, j, syn)?;
                n_exc_conn += 1;
            }
        }
    }

    // Inhibitory neurons (8-9) connect to all excitatory
    let mut n_inh_conn = 0;
    for i in n_exc..n_total {
        for j in 0..n_exc {
            let syn = Synapse::inhibitory(2.0, 1.0)?; // Strong inhibition
            network.add_connection(i, j, syn)?;
            n_inh_conn += 1;
        }
    }

    println!("   Total neurons: {}", n_total);
    println!("   Excitatory: {}, Inhibitory: {}", n_exc, n_inh);
    println!("   Excitatory connections: {}", n_exc_conn);
    println!("   Inhibitory connections: {}", n_inh_conn);
    println!("   E/I ratio: {:.2}", n_exc_conn as f64 / n_inh_conn as f64);

    let stats = network.connectivity_stats();
    println!("\n   Network statistics:");
    println!("   Total connections: {}", stats.n_connections);
    println!("   Average in-degree: {:.2}", stats.avg_in_degree);
    println!("   Average out-degree: {:.2}", stats.avg_out_degree);

    Ok(())
}

fn gap_junction_network() -> Result<(), Box<dyn std::error::Error>> {
    println!("3. Gap Junction Network");

    let mut network = SynapticNetwork::new(4);

    // Create chemical synapses
    let syn1 = Synapse::excitatory(1.0, 1.0)?;
    let syn2 = Synapse::excitatory(1.0, 1.0)?;
    network.add_connection(0, 1, syn1)?;
    network.add_connection(1, 2, syn2)?;

    // Add gap junctions (electrical coupling)
    network.add_gap_junction(2, 3, 0.5)?; // 0.5 nS coupling

    println!("   Network structure:");
    println!("   Neuron 0 --chemical--> Neuron 1 --chemical--> Neuron 2 --gap junction-- Neuron 3");
    println!("   Gap junction conductance: 0.5 nS");

    let stats = network.connectivity_stats();
    println!("\n   Chemical synapses: {}", stats.n_connections);
    println!("   Gap junctions: {}", stats.n_gap_junctions);

    // Simulate
    println!("\n   Simulating with voltage difference across gap junction:");
    println!("   V2 = -60 mV, V3 = -70 mV");

    let mut voltages = vec![-65.0; 4];
    voltages[2] = -60.0; // Depolarized
    voltages[3] = -70.0; // Hyperpolarized

    network.update(&voltages, 0.1)?;

    let i_chem_2 = network.get_synaptic_current(2)?;
    let i_chem_3 = network.get_synaptic_current(3)?;

    println!("\n   Current to neuron 2: {:.4} pA (chemical)", i_chem_2);
    println!("   Current to neuron 3: {:.4} pA (includes gap junction)", i_chem_3);
    println!("\n   Note: Gap junction provides bidirectional current flow");

    Ok(())
}
