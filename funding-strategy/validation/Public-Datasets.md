# Public Neuroimaging Datasets for CORTEXIA Validation

**Purpose:** Identify and document publicly available datasets for validating CORTEXIA's Φ calculations

**Priority:** HIGH - Required for R21 preliminary data

---

## 🎯 Validation Strategy

### Primary Objectives
1. Validate CORTEXIA Φ calculations against published IIT results
2. Demonstrate scalability (from artificial to biological scales)
3. Establish baseline consciousness metrics
4. Generate publication-quality results

### Success Criteria
- ✅ Φ values match published IIT benchmarks (±10%)
- ✅ Reproducible results (CV < 15%)
- ✅ Computational efficiency (runtime < 24hr per analysis)
- ✅ Publication-ready figures and statistics

---

## 📊 Tier 1 Datasets (Immediate Priority)

### 1. Human Connectome Project (HCP)

**URL:** https://www.humanconnectome.org/

**Description:**
- High-resolution fMRI, structural MRI, diffusion MRI
- 1,200 healthy adults (ages 22-35)
- Resting state + task-based fMRI
- Connectivity matrices available

**Relevance for CORTEXIA:**
- ✅ Gold standard for human brain connectivity
- ✅ Published IIT studies for comparison
- ✅ Multiple consciousness states (rest, task, sleep)
- ✅ Large sample size (statistical power)

**Access:**
- Free registration at ConnectomeDB
- ~500GB download (full dataset)
- Preprocessed connectomes available

**CORTEXIA Application:**
1. Load HCP connectivity matrices → `iit` crate
2. Compute Φ for resting state networks
3. Compare wake vs sleep states (consciousness contrast)
4. Validate against Tononi lab published results

**Timeline:** 1-2 months
- Week 1-2: Data download & preprocessing
- Week 3-6: CORTEXIA analysis
- Week 7-8: Statistical validation & figures

**Expected Output:**
- Φ distributions for n=100-1000 subjects
- Wake vs sleep Φ differences
- First publication draft

---

### 2. Allen Brain Observatory

**URL:** https://observatory.brain-map.org/

**Description:**
- Mouse visual cortex electrophysiology
- 2-photon calcium imaging
- 10,000+ neurons recorded simultaneously
- Stimulus-response paradigms

**Relevance for CORTEXIA:**
- ✅ Single-neuron resolution
- ✅ Multiple consciousness states (awake, anesthetized)
- ✅ Hodgkin-Huxley validation possible
- ✅ TDA applicable (cell assemblies)

**Access:**
- Free via Allen SDK (Python)
- ~50GB for relevant experiments
- API access available

**CORTEXIA Application:**
1. Import spike trains → `tda` crate
2. Compute persistent homology (cell assemblies)
3. Apply `neural-dynamics` for network analysis
4. Compare awake vs anesthetized Φ

**Timeline:** 1-2 months
- Week 1-2: Allen SDK integration
- Week 3-5: TDA + IIT analysis
- Week 6-8: Anesthesia consciousness analysis

**Expected Output:**
- Awake vs anesthetized Φ trajectories
- Cell assembly topology
- Second publication (TDA + consciousness)

---

### 3. NeuroData Without Borders (NWB) Datasets

**URL:** https://www.nwb.org/example-datasets/

**Description:**
- Standardized neurophysiology format
- Multiple species (human, monkey, rat, mouse)
- Electrophysiology, imaging, behavior
- 100+ publicly available datasets

**Relevance for CORTEXIA:**
- ✅ Diverse data types (multi-scale validation)
- ✅ Standardized format (easy integration)
- ✅ Multiple consciousness manipulations
- ✅ Cross-species comparison

**Access:**
- Free download via DANDI Archive
- Variable sizes (1GB - 500GB)
- Python API (NWB tools)

**CORTEXIA Application:**
1. NWB → Rust data structures
2. Multi-scale Φ analysis (neurons to regions)
3. Cross-species consciousness comparison
4. Validate `hodgkin-huxley` on real spike trains

**Timeline:** 2-3 months
- Week 1-3: NWB integration (Rust bindings)
- Week 4-8: Analysis across datasets
- Week 9-12: Comparative consciousness paper

**Expected Output:**
- Cross-species Φ comparison
- Multi-scale consciousness architecture
- Methods paper (NWB + CORTEXIA integration)

---

## 📊 Tier 2 Datasets (Secondary Priority)

### 4. UK Biobank Brain Imaging

**URL:** https://www.ukbiobank.ac.uk/

**Description:**
- 100,000+ participants (largest brain imaging study)
- MRI, genetics, health data
- Aging, disease, genetics interactions

**Relevance:**
- Large sample size (population studies)
- Disease states (consciousness disorders)
- Genetic associations with Φ

**Access:** Application required (~6 weeks)

**Timeline:** 3-6 months (with access approval)

---

### 5. CRCNS (Collaborative Research in Computational Neuroscience)

**URL:** https://crcns.org/

**Description:**
- 50+ electrophysiology datasets
- Multiple species and brain regions
- Sleep, anesthesia, behavior

**Relevance:**
- Consciousness state transitions
- Multi-region recordings
- Sleep architecture analysis

**Access:** Free registration

**Timeline:** 2-3 months

---

### 6. OpenNeuro (fMRI Repository)

**URL:** https://openneuro.org/

**Description:**
- 700+ fMRI datasets
- Task and resting state
- Multiple psychiatric conditions

**Relevance:**
- Disease states (schizophrenia, depression)
- Altered consciousness
- Large dataset variety

**Access:** Free download

**Timeline:** 1-2 months per dataset

---

## 🔬 Validation Experiments

### Experiment 1: Consciousness States (HCP)

**Hypothesis:** Φ(wake) > Φ(sleep) > Φ(anesthesia)

**Method:**
1. Load HCP resting state connectivity (n=100 subjects)
2. Load sleep fMRI data (HCP subset)
3. Compute Φ for each state using `iit` crate
4. Statistical comparison (paired t-test, effect size)

**Expected Results:**
- Φ_wake: 2.5-3.5 bits (based on Tononi papers)
- Φ_sleep: 1.5-2.5 bits
- Δ Φ: 1.0-1.5 bits (p < 0.001)

**Validation:** Compare to published IIT values

---

### Experiment 2: Network Topology (Allen Brain)

**Hypothesis:** Awake networks have richer topology than anesthetized

**Method:**
1. Extract spike trains (awake vs isoflurane anesthesia)
2. Compute persistent homology with `tda` crate
3. Compare Betti numbers (topological complexity)
4. Correlate topology with Φ

**Expected Results:**
- Awake: Higher H1 (loops), higher Φ
- Anesthesia: Reduced topology, lower Φ
- Strong correlation: Topology ↔ Φ (r > 0.7)

**Validation:** Consistent with consciousness as integrated information

---

### Experiment 3: Scale Invariance (NWB Multi-Scale)

**Hypothesis:** Φ is scale-invariant (neurons → regions → whole-brain)

**Method:**
1. Analyze same data at multiple scales
   - Single neurons (10-100)
   - Local circuits (100-1000)
   - Brain regions (10-100 regions)
2. Compute Φ at each scale
3. Test scale-invariance predictions

**Expected Results:**
- Φ follows power-law across scales
- Critical exponent β ≈ -1 to -0.5
- Consistent with integrated information across scales

**Validation:** Novel finding (first multi-scale Φ analysis)

---

## 📈 Publication Strategy

### Paper 1: "CORTEXIA Framework and Validation"
**Dataset:** HCP + Allen Brain
**Journal:** PLOS Computational Biology
**Timeline:** Submit June 2025

**Content:**
- CORTEXIA architecture
- IIT 3.0 implementation
- Validation on HCP (wake vs sleep Φ)
- Performance benchmarks

---

### Paper 2: "Topological Signatures of Consciousness"
**Dataset:** Allen Brain + NWB
**Journal:** Network Neuroscience
**Timeline:** Submit October 2025

**Content:**
- TDA + IIT integration
- Awake vs anesthesia topology
- Persistent homology of consciousness
- Cell assembly dynamics

---

### Paper 3: "Multi-Scale Consciousness Architecture"
**Dataset:** NWB Multi-Region + HCP
**Journal:** eLife or Nature Communications
**Timeline:** Submit March 2026

**Content:**
- Cross-scale Φ analysis
- Integration across neural hierarchies
- Criticality and consciousness
- Novel theoretical insights

---

## 🛠️ Technical Implementation

### Data Pipeline

```rust
// Pseudo-code for HCP → CORTEXIA pipeline

use cortexia::iit::{IITSystem, PhiCalculation};
use hcp_loader::ConnectivityMatrix;

fn analyze_hcp_subject(subject_id: &str) -> Result<f64> {
    // 1. Load HCP connectivity matrix
    let connectivity = ConnectivityMatrix::load(subject_id)?;

    // 2. Convert to IIT system
    let system = IITSystem::from_connectivity(connectivity);

    // 3. Compute Φ (geometric approximation for speed)
    let phi = system.calculate_phi(PhiCalculation::Geometric)?;

    Ok(phi)
}

// Batch analysis
fn analyze_hcp_cohort(n_subjects: usize) -> Vec<f64> {
    (0..n_subjects)
        .into_par_iter() // Parallel processing
        .map(|i| analyze_hcp_subject(&format!("HCP_{}", i)))
        .filter_map(Result::ok)
        .collect()
}
```

---

### Performance Requirements

| Dataset | Size | Subjects | Compute Time | Hardware |
|---------|------|----------|--------------|----------|
| HCP Resting | 500 GB | 1,000 | 100-200 hrs | 32-core CPU |
| Allen Brain | 50 GB | 10 experiments | 20-50 hrs | 16-core CPU |
| NWB (various) | 100 GB | 50 datasets | 50-100 hrs | 32-core CPU |

**Total Compute:** ~300 hrs = 12 days continuous
**Cost Estimate:** $500-1000 (AWS/cloud)

---

## ✅ Validation Checklist

### Before R21 Submission (September 2025)

- [ ] HCP analysis complete (n≥100 subjects)
- [ ] Wake vs sleep Φ comparison (statistical significance)
- [ ] Allen Brain awake vs anesthesia analysis
- [ ] At least 1 preprint/publication submitted
- [ ] Publication-quality figures (3-5 main figures)
- [ ] Reproducibility code published (GitHub)
- [ ] Statistical validation (power analysis, effect sizes)

### For First Publication

- [ ] Methods section written (CORTEXIA + datasets)
- [ ] Results section (Φ calculations, statistics)
- [ ] Discussion (comparison to published IIT work)
- [ ] Supplementary material (all subjects, raw data)
- [ ] Code availability (crates.io + GitHub)
- [ ] Data availability statement

---

## 📞 Contact Information

### Dataset Access Support

**HCP:**
- ConnectomeDB: https://db.humanconnectome.org/
- Email: connectome-help@humanconnectome.org

**Allen Brain:**
- Observatory support: https://community.brain-map.org/
- Email: help@alleninstitute.org

**NWB/DANDI:**
- DANDI Archive: https://dandiarchive.org/
- Email: info@dandiarchive.org

---

**Status:** Planning Phase
**Priority:** HIGH (blocking for R21 submission)
**Owner:** [Primary investigator]
**Last Updated:** 2025-01-13
**Next Review:** 2025-02-01
