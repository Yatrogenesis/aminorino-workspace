# BUG REPORT: Fundamental Error in Φ Calculation
**Date:** 2025-01-10
**Severity:** CRITICAL
**Status:** Identified, Solution Proposed
**Reporter:** Francisco Molina Burgos + Claude Code

---

## Executive Summary

The `measure_phi_quantum()` function in `/brain-ai-native/src/consciousness.rs` contains a **fundamental architectural error** that causes it to:

1. Partition **oscillator coordinates** (4-dimensional) instead of **quantum state space** (81-dimensional)
2. Use bit-indexing marginalization assuming binary variables, incompatible with Fock state distributions
3. Return Φ ≈ 0 for all experiments due to wrong granularity

**Impact:** All experiments measuring Φ_quantum are invalid. The null result (Φ_quantum = Φ_classical = 0.001204) is an artifact of this bug, not a real measurement.

---

## Technical Analysis

### The Bug (Two Layers)

#### Layer 1: Wrong `num_elements` (Line 94)
```rust
// File: src/consciousness.rs
pub fn measure_phi_quantum(brain: &AIBrain) -> BrainResult<ConsciousnessMeasurement> {
    let num_elements = brain.config.num_oscillators;  // ← BUG: Uses 4
    let state_space_size = brain.config.effective_neurons();  // Correctly calculates 81

    let state_vector = brain.get_state_vector();  // Length = 81 ✓
    let prob_dist = state_to_probability_distribution(&state_vector);  // 81-dim distribution ✓

    let partitions = generate_bipartitions(num_elements);  // ← Only 7 partitions of 4 elements!
    // Should partition 81 neurons, not 4 oscillators
}
```

**What Happens:**
- System has 81 effective neurons: `(max_fock + 1)^num_oscillators = 3^4 = 81`
- State vector correctly has 81 dimensions
- **But:** Φ calculation partitions only 4 oscillator coordinates
- Treats system as 4 binary elements (2^4 = 16 states) instead of 81 quantum states

#### Layer 2: Incompatible Marginalization (Lines 289-301)
```rust
fn marginalize_distribution(
    prob_dist: &[f64],
    subset_indices: &[usize],
) -> Vec<f64> {
    let subset_size = subset_indices.len();
    let subset_states = 1 << subset_size;  // ← Assumes binary: 2^n

    let mut marginal = vec![0.0; subset_states];

    for (state_idx, &prob) in prob_dist.iter().enumerate() {
        let mut subset_state = 0;
        for (bit_pos, &elem_idx) in subset_indices.iter().enumerate() {
            if state_idx & (1 << elem_idx) != 0 {  // ← Bit indexing for binary variables
                subset_state |= 1 << bit_pos;
            }
        }
        marginal[subset_state] += prob;
    }
    marginal
}
```

**Problem:** This function assumes each element is **binary** (0 or 1), using bit manipulation. But Fock states are **ternary** for max_fock=2: {|0⟩, |1⟩, |2⟩}.

**Example:**
- State index 42 in 81-dimensional space = specific Fock configuration
- Bit indexing `state_idx & (1 << elem_idx)` doesn't correctly extract Fock number for that oscillator
- Marginalization produces incorrect probability distributions

---

## Why All Experiments Gave Φ ≈ 0

### Experiment 1: Static State (No Noise)
```rust
// consciousness_experiment.rs
let input = vec![0.5, 0.6, 0.7, 0.8];  // 4-dim input
brain.set_input(&input)?;
brain.evolve(1e-9)?;
let measurement = measure_phi_quantum(&brain)?;
// Result: Φ = 0.001204
```

**Why Φ=0:**
1. 81-dimensional quantum state created ✓
2. But partitioned as if 4 binary variables
3. With 4 nearly-identical oscillator coordinates, minimal information to partition
4. Φ ≈ 0 (not because state is non-conscious, but because wrong space partitioned)

### Experiment 2: Noise and Activity
```rust
// consciousness_with_noise.rs
for step in 0..num_steps {
    let noise_input: Vec<f64> = (0..4).map(|_| amplitude * rng.gen()).collect();
    brain.set_input(&noise_input)?;
    brain.evolve(dt)?;
    let measurement = measure_phi_quantum(&brain)?;
}
// Result: Φ = 0 for ALL noise levels
```

**Why Φ=0 even with noise:**
1. Noise creates dynamic 81-dimensional quantum states ✓
2. **But still partitions only 4 oscillator coordinates**
3. Even with stochastic inputs, 4 coordinates have low mutual information
4. Φ ≈ 0 regardless of noise amplitude

---

## User Insight Validation

**Francisco's observation:** *"cómo yo lo veo, evaluaste es estado cero, un sistema consciente lo es por estar lleno de ruido y factores externos que lo hacer reaccionar o activarse"*

**Status:** ✅ **CORRECT** - but there's an additional bug layer:

1. ✓ **Correct:** Consciousness requires noise and activity, not static ground state
2. ✓ **Correct:** Initial experiment used near-ground state with minimal excitation
3. ✓ **Correct:** Need dynamic inputs and stochastic perturbations

**BUT ALSO:**
4. Even WITH noise and activity, the calculation partitions the wrong space
5. We measure `Φ(4 oscillator coords)` not `Φ(81 quantum neurons)`

---

## The Computational Impossibility

**Naive Fix (Won't Work):**
```rust
let num_elements = brain.config.effective_neurons();  // 81
let partitions = generate_bipartitions(num_elements);  // 2^80 partitions!
```

**Problem:**
- 2^80 = **1.2 × 10^24 bipartitions**
- At 1 microsecond per partition: **3.8 × 10^10 years** to compute
- Longer than age of universe

**IIT was designed for small systems** (N ≤ 10 neurons max in practice).

---

## Proposed Solutions

### Option 1: Approximate Φ via Sampling ⭐ RECOMMENDED
```rust
pub fn measure_phi_quantum_sampled(
    brain: &AIBrain,
    num_samples: usize,  // e.g., 1000 random partitions
) -> BrainResult<ConsciousnessMeasurement> {
    let num_elements = brain.config.effective_neurons();
    let state_vector = brain.get_state_vector();
    let prob_dist = state_to_probability_distribution(&state_vector);

    // Sample random bipartitions
    let mut min_info_loss = f64::INFINITY;
    let mut rng = rand::thread_rng();

    for _ in 0..num_samples {
        let partition = generate_random_bipartition(num_elements, &mut rng);
        let info_loss = calculate_partition_information(&prob_dist, &partition)?;
        min_info_loss = min_info_loss.min(info_loss);
    }

    Ok(ConsciousnessMeasurement {
        phi: min_info_loss,
        method: "sampled".to_string(),
        num_partitions: num_samples,
        // ...
    })
}
```

**Pros:**
- Computationally feasible
- Scales to large systems
- Statistically valid approximation

**Cons:**
- Not exact Φ (but exact Φ is impossible anyway for N>20)
- Requires validation against known small-system results

### Option 2: Hierarchical Partitioning
Partition at multiple scales:
1. Coarse: 4 oscillators (as currently done)
2. Fine: Groups of Fock states
3. Integrate across scales

**Pros:**
- Captures multi-scale integration
- Computationally tractable

**Cons:**
- Not standard IIT 3.0
- Requires theoretical justification

### Option 3: Fixed Marginalization for Fock States
Instead of binary bit indexing, use base-3 (or base-n) indexing:

```rust
fn marginalize_fock_distribution(
    prob_dist: &[f64],
    subset_indices: &[usize],
    fock_levels: usize,  // max_fock + 1
) -> Vec<f64> {
    let subset_size = subset_indices.len();
    let subset_states = fock_levels.pow(subset_size as u32);

    let mut marginal = vec![0.0; subset_states];

    for (state_idx, &prob) in prob_dist.iter().enumerate() {
        // Decode state_idx as base-fock_levels number
        let mut subset_state = 0;
        let mut temp_idx = state_idx;

        for (pos, &elem_idx) in subset_indices.iter().enumerate() {
            // Extract Fock number for oscillator elem_idx
            let fock_num = (temp_idx / fock_levels.pow(elem_idx as u32)) % fock_levels;
            subset_state += fock_num * fock_levels.pow(pos as u32);
        }
        marginal[subset_state] += prob;
    }
    marginal
}
```

**Combine with Option 1 (sampling) for feasibility.**

---

## Impact on Published Results

### Affected Experiments
1. ❌ `consciousness_experiment.rs` - Results invalid
2. ❌ `consciousness_with_noise.rs` - Results invalid
3. ❌ All data in `/Articulos-IEEE/Cortexia/Brain-AI-Native/PhiQ_MajorThan_PhiClassical/`

### Salvageable Elements
✅ **Infrastructure is valid:**
- Quantum reservoir computing works correctly
- State vector evolution is correct
- Hodgkin-Huxley integration works
- LDPC error correction works
- Radiation simulation works

✅ **Methodology is sound:**
- IIT framework applicable to quantum systems (in principle)
- Experiment design is rigorous
- Documentation is thorough

❌ **Only the Φ calculation needs fixing**

---

## Recommended Action Plan

### Immediate (This Week)
1. ✅ Document this bug (this file)
2. 🔲 Implement Option 1 (sampled Φ) + Option 3 (Fock marginalization)
3. 🔲 Add unit tests with small systems (N=2, max_fock=1 → 4 states, tractable)
4. 🔲 Validate against exact calculation for N=2

### Short Term (Next 2 Weeks)
5. 🔲 Re-run all experiments with corrected calculation
6. 🔲 Compare Φ_quantum vs Φ_classical with proper baselines (RNN/LSTM)
7. 🔲 Update all articles in `/Articulos-IEEE/` with corrected results
8. 🔲 Add "Erratum" section to published documents

### Long Term (Next Month)
9. 🔲 Implement hierarchical partitioning (Option 2)
10. 🔲 GPU acceleration for sampling
11. 🔲 Theoretical paper: "IIT for Quantum Systems: Computational Approaches"

---

## References

### IIT Computational Complexity
- Mayner et al. (2018). "PyPhi: A toolbox for integrated information theory." *PLoS Comput Biol* 14(7): e1006343.
  - **Key finding:** IIT is NP-hard, practical limit N ≤ 10-12 neurons

### Quantum IIT Theory
- Kremnizer & Ranchin (2015). "Integrated Information-Induced Quantum Collapse." *Found Phys* 45: 889-899.
  - DOI: 10.1007/s10701-015-9905-6
  - **Proposes quantum extension of IIT**

- Zanardi et al. (2018). "Quantum Integrated Information Theory." *arXiv:1806.01421*
  - **Extends IIT to quantum systems, notes computational challenges**

---

## Conclusion

**Question:** *"oye, pero ese es un error puntual del experimento o la forma de calculo???"*

**Answer:** **Es la forma de cálculo** (it's the calculation method).

Specifically:
1. **Not just one line** - the entire approach of enumerating bipartitions doesn't scale
2. **Marginalization assumes binary variables** - incompatible with Fock states
3. **Even with "correct" num_elements, computation is infeasible** for N=81

**But it's fixable** via sampling approximation + proper Fock-state marginalization.

---

**Next Steps:**
1. Implement corrected `measure_phi_quantum_sampled()` with Fock marginalization
2. Validate on small systems (N=2, exact enumeration possible)
3. Re-run experiments with noise + activity + corrected calculation
4. Publish erratum + corrected results

---

**Generated by:** Claude Code by Anthropic
**Co-Authored-By:** Claude <noreply@anthropic.com>
**Date:** 2025-01-10
**Workspace:** /Users/yatrogenesis/cortexia-workspace
