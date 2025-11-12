#!/usr/bin/env python3
"""
Generate figures for Paper #5: Cálculo de Significados y MOM

Figures to generate:
1. MOM (Multi-Layer Operational Model) Architecture
2. Fourier Analysis of Cognitive Trajectories
3. PVE (Pruebas de Validación Experiencial) Protocol Flowchart
"""

import matplotlib.pyplot as plt
import matplotlib.patches as mpatches
from matplotlib.patches import FancyBboxPatch, FancyArrowPatch, Circle
import numpy as np

# Set style
plt.style.use('seaborn-v0_8-darkgrid')
plt.rcParams['figure.figsize'] = (12, 8)
plt.rcParams['font.size'] = 10
plt.rcParams['font.family'] = 'DejaVu Sans'

def figure1_mom_architecture():
    """
    Figure 1: Multi-Layer Operational Model (MOM) Architecture

    Shows 4 layers:
    - Layer 1: Sensory Inputs (S)
    - Layer 2: Epistemological Rhythm (Rτ)
    - Layer 3: Semantic Space (Hilbert)
    - Layer 4: Meta-Cognitive Reflection
    """
    fig, ax = plt.subplots(figsize=(14, 10))
    ax.set_xlim(0, 10)
    ax.set_ylim(0, 10)
    ax.axis('off')

    # Title
    ax.text(5, 9.5, 'Multi-Layer Operational Model (MOM)',
            fontsize=20, fontweight='bold', ha='center')
    ax.text(5, 9.0, 'Four-Layer Architecture for Consciousness Expansion',
            fontsize=14, ha='center', style='italic')

    # Layer dimensions
    layer_height = 1.5
    layer_width = 8.0
    layer_x = 1.0

    # Layer 4: Meta-Cognitive Reflection (Top)
    y4 = 7.0
    layer4 = FancyBboxPatch((layer_x, y4), layer_width, layer_height,
                            boxstyle="round,pad=0.1",
                            edgecolor='#9b59b6', facecolor='#e8daef', linewidth=3)
    ax.add_patch(layer4)
    ax.text(layer_x + layer_width/2, y4 + layer_height - 0.3,
            'Layer 4: Meta-Cognitive Reflection',
            fontsize=14, fontweight='bold', ha='center')
    ax.text(layer_x + layer_width/2, y4 + 0.7,
            r'Self-awareness: $\Gamma \vdash M : \text{Type}\{S | C\}$',
            fontsize=11, ha='center', family='monospace')
    ax.text(layer_x + layer_width/2, y4 + 0.3,
            'Context-dependent types, meta-cognition',
            fontsize=10, ha='center', style='italic')

    # Layer 3: Semantic Space (Hilbert)
    y3 = 5.2
    layer3 = FancyBboxPatch((layer_x, y3), layer_width, layer_height,
                            boxstyle="round,pad=0.1",
                            edgecolor='#3498db', facecolor='#d6eaf8', linewidth=3)
    ax.add_patch(layer3)
    ax.text(layer_x + layer_width/2, y3 + layer_height - 0.3,
            'Layer 3: Semantic Space (Hilbert)',
            fontsize=14, fontweight='bold', ha='center')
    ax.text(layer_x + layer_width/2, y3 + 0.7,
            r'Meaning vectors: $\vec{m} \in \mathcal{H}, \|\vec{m}\| < \infty$',
            fontsize=11, ha='center', family='monospace')
    ax.text(layer_x + layer_width/2, y3 + 0.3,
            'Infinite-dimensional Hilbert space of meanings',
            fontsize=10, ha='center', style='italic')

    # Layer 2: Epistemological Rhythm
    y2 = 3.4
    layer2 = FancyBboxPatch((layer_x, y2), layer_width, layer_height,
                            boxstyle="round,pad=0.1",
                            edgecolor='#e74c3c', facecolor='#fadbd8', linewidth=3)
    ax.add_patch(layer2)
    ax.text(layer_x + layer_width/2, y2 + layer_height - 0.3,
            'Layer 2: Epistemological Rhythm',
            fontsize=14, fontweight='bold', ha='center')
    ax.text(layer_x + layer_width/2, y2 + 0.7,
            r'$R_\tau = \{(f_i, a_i, \phi_i) : i = 1, 2, ..., n\}$',
            fontsize=11, ha='center', family='monospace')
    ax.text(layer_x + layer_width/2, y2 + 0.3,
            'Frequency, amplitude, phase triplets of cognition',
            fontsize=10, ha='center', style='italic')

    # Layer 1: Sensory Inputs
    y1 = 1.6
    layer1 = FancyBboxPatch((layer_x, y1), layer_width, layer_height,
                            boxstyle="round,pad=0.1",
                            edgecolor='#27ae60', facecolor='#d5f4e6', linewidth=3)
    ax.add_patch(layer1)
    ax.text(layer_x + layer_width/2, y1 + layer_height - 0.3,
            'Layer 1: Sensory Inputs & Perceptual Experience',
            fontsize=14, fontweight='bold', ha='center')
    ax.text(layer_x + layer_width/2, y1 + 0.7,
            r'$S(t) = \{\text{visual}, \text{auditory}, \text{somatic}, \text{proprioceptive}\}$',
            fontsize=11, ha='center', family='monospace')
    ax.text(layer_x + layer_width/2, y1 + 0.3,
            'Multimodal sensory streams',
            fontsize=10, ha='center', style='italic')

    # Arrows between layers (bidirectional)
    arrow_props = dict(arrowstyle='<->', lw=2.5, color='#34495e')

    # Layer 1 <-> Layer 2
    ax.annotate('', xy=(layer_x + layer_width/2, y2 - 0.1),
                xytext=(layer_x + layer_width/2, y1 + layer_height + 0.1),
                arrowprops=arrow_props)
    ax.text(layer_x + layer_width + 0.3, (y1 + layer_height + y2)/2,
            'Fourier\nTransform', fontsize=9, ha='left', va='center')

    # Layer 2 <-> Layer 3
    ax.annotate('', xy=(layer_x + layer_width/2, y3 - 0.1),
                xytext=(layer_x + layer_width/2, y2 + layer_height + 0.1),
                arrowprops=arrow_props)
    ax.text(layer_x + layer_width + 0.3, (y2 + layer_height + y3)/2,
            'Semantic\nProjection', fontsize=9, ha='left', va='center')

    # Layer 3 <-> Layer 4
    ax.annotate('', xy=(layer_x + layer_width/2, y4 - 0.1),
                xytext=(layer_x + layer_width/2, y3 + layer_height + 0.1),
                arrowprops=arrow_props)
    ax.text(layer_x + layer_width + 0.3, (y3 + layer_height + y4)/2,
            'Type\nInference', fontsize=9, ha='left', va='center')

    # External feedback loop (Layer 4 -> Layer 1)
    from matplotlib.patches import Arc
    arc = Arc((5, 5), width=10, height=8, angle=0, theta1=60, theta2=300,
              color='#e67e22', linewidth=2.5, linestyle='--')
    ax.add_patch(arc)
    ax.text(0.5, 5, 'Consciousness\nFeedback', fontsize=10, ha='center',
            color='#e67e22', fontweight='bold', rotation=90, va='center')

    # Legend
    legend_y = 0.5
    ax.text(1, legend_y, '⬤ Green: Perception  ⬤ Red: Cognition  ⬤ Blue: Semantics  ⬤ Purple: Meta-cognition',
            fontsize=9, ha='left', va='center')

    plt.tight_layout()
    plt.savefig('figures/mom_architecture.png', dpi=300, bbox_inches='tight')
    print("✅ Figure 1 saved: figures/mom_architecture.png")
    plt.close()

def figure2_fourier_spectrum():
    """
    Figure 2: Fourier Analysis of Cognitive Trajectories

    Shows:
    - Time-domain cognitive trajectory τ(t)
    - Frequency-domain decomposition (Fourier spectrum)
    - Epistemological rhythm components
    """
    fig, axes = plt.subplots(2, 1, figsize=(14, 10))

    # Time domain: Cognitive trajectory
    ax1 = axes[0]
    t = np.linspace(0, 10, 1000)

    # Simulate complex cognitive trajectory with multiple frequency components
    # τ(t) = Σ c_i * exp(j*2π*f_i*t)
    frequencies = [0.5, 1.2, 2.3, 4.1]  # Hz (different cognitive processes)
    amplitudes = [1.0, 0.6, 0.4, 0.2]
    phases = [0, np.pi/4, np.pi/2, np.pi]

    tau_t = np.zeros_like(t)
    for f, a, phi in zip(frequencies, amplitudes, phases):
        tau_t += a * np.sin(2 * np.pi * f * t + phi)

    # Add some noise to make it realistic
    tau_t += 0.1 * np.random.randn(len(t))

    ax1.plot(t, tau_t, linewidth=2, color='#2c3e50', label=r'Cognitive Trajectory $\tau(t)$')
    ax1.set_xlabel('Time (arbitrary units)', fontsize=12)
    ax1.set_ylabel('Semantic Position', fontsize=12)
    ax1.set_title('Time Domain: Cognitive Trajectory in Semantic Space', fontsize=14, fontweight='bold')
    ax1.grid(True, alpha=0.3)
    ax1.legend(fontsize=11, loc='upper right')

    # Mark key events
    events = [(2.1, 'Insight'), (5.3, 'Confusion'), (8.2, 'Resolution')]
    for t_event, label in events:
        idx = np.argmin(np.abs(t - t_event))
        ax1.axvline(t_event, color='#e74c3c', linestyle='--', alpha=0.6, linewidth=1.5)
        ax1.annotate(label, xy=(t_event, tau_t[idx]), xytext=(t_event + 0.5, tau_t[idx] + 0.5),
                    arrowprops=dict(arrowstyle='->', color='#e74c3c', lw=1.5),
                    fontsize=10, color='#e74c3c', fontweight='bold')

    # Frequency domain: Epistemological Rhythm
    ax2 = axes[1]

    # Theoretical frequencies and their cognitive interpretations
    freq_labels = ['δ\n(0.5 Hz)\nSlow\nReflection',
                   'θ\n(1.2 Hz)\nWorking\nMemory',
                   'α\n(2.3 Hz)\nAttention',
                   'β\n(4.1 Hz)\nActive\nProcessing']

    colors = ['#27ae60', '#3498db', '#e74c3c', '#9b59b6']

    x_pos = np.array(frequencies)
    y_amplitudes = np.array(amplitudes)

    bars = ax2.bar(x_pos, y_amplitudes, width=0.3, color=colors, alpha=0.7, edgecolor='black', linewidth=1.5)

    # Add labels above bars
    for i, (f, a, label) in enumerate(zip(frequencies, amplitudes, freq_labels)):
        ax2.text(f, a + 0.05, label, ha='center', va='bottom', fontsize=9, fontweight='bold')

    ax2.set_xlabel('Frequency (Hz)', fontsize=12)
    ax2.set_ylabel('Amplitude (normalized)', fontsize=12)
    ax2.set_title('Frequency Domain: Epistemological Rhythm Decomposition', fontsize=14, fontweight='bold')
    ax2.set_xlim(-0.5, 5)
    ax2.set_ylim(0, 1.3)
    ax2.grid(True, alpha=0.3, axis='y')

    # Mathematical annotation
    ax2.text(4.5, 1.1, r'$R_\tau = \{(f_i, a_i, \phi_i)\}_{i=1}^4$',
            fontsize=12, ha='right', bbox=dict(boxstyle='round', facecolor='wheat', alpha=0.5))

    plt.tight_layout()
    plt.savefig('figures/fourier_cognitive_spectrum.png', dpi=300, bbox_inches='tight')
    print("✅ Figure 2 saved: figures/fourier_cognitive_spectrum.png")
    plt.close()

def figure3_pve_protocol():
    """
    Figure 3: PVE (Pruebas de Validación Experiencial) Protocol Flowchart

    Shows:
    - PVE-1: Inter-subject synchronization
    - PVE-2: Frequency modulation
    - PVE-3: Semantic coherence analysis
    """
    fig, ax = plt.subplots(figsize=(14, 12))
    ax.set_xlim(0, 10)
    ax.set_ylim(0, 12)
    ax.axis('off')

    # Title
    ax.text(5, 11.5, 'PVE: Pruebas de Validación Experiencial',
            fontsize=18, fontweight='bold', ha='center')
    ax.text(5, 11.0, 'Three-Protocol Framework for Empirical Validation',
            fontsize=13, ha='center', style='italic')

    # Protocol boxes
    box_width = 2.8
    box_height = 2.0

    # PVE-1: Inter-subject Synchronization
    x1, y1 = 0.5, 8.0
    pve1 = FancyBboxPatch((x1, y1), box_width, box_height,
                          boxstyle="round,pad=0.15",
                          edgecolor='#27ae60', facecolor='#d5f4e6', linewidth=3)
    ax.add_patch(pve1)
    ax.text(x1 + box_width/2, y1 + box_height - 0.3,
            'PVE-1', fontsize=14, fontweight='bold', ha='center', color='#27ae60')
    ax.text(x1 + box_width/2, y1 + 1.3,
            'Inter-Subject\nSynchronization', fontsize=11, ha='center', fontweight='bold')
    ax.text(x1 + box_width/2, y1 + 0.6,
            'Measure: PLV\n(Phase-Locking\nValue)', fontsize=9, ha='center')
    ax.text(x1 + box_width/2, y1 + 0.1,
            'n=20 subjects', fontsize=8, ha='center', style='italic')

    # PVE-2: Frequency Modulation
    x2, y2 = 3.6, 8.0
    pve2 = FancyBboxPatch((x2, y2), box_width, box_height,
                          boxstyle="round,pad=0.15",
                          edgecolor='#3498db', facecolor='#d6eaf8', linewidth=3)
    ax.add_patch(pve2)
    ax.text(x2 + box_width/2, y2 + box_height - 0.3,
            'PVE-2', fontsize=14, fontweight='bold', ha='center', color='#3498db')
    ax.text(x2 + box_width/2, y2 + 1.3,
            'Frequency\nModulation', fontsize=11, ha='center', fontweight='bold')
    ax.text(x2 + box_width/2, y2 + 0.6,
            'Shift: θ→α→β\nObserve Δτ', fontsize=9, ha='center')
    ax.text(x2 + box_width/2, y2 + 0.1,
            '100 trials', fontsize=8, ha='center', style='italic')

    # PVE-3: Semantic Coherence
    x3, y3 = 6.7, 8.0
    pve3 = FancyBboxPatch((x3, y3), box_width, box_height,
                          boxstyle="round,pad=0.15",
                          edgecolor='#e74c3c', facecolor='#fadbd8', linewidth=3)
    ax.add_patch(pve3)
    ax.text(x3 + box_width/2, y3 + box_height - 0.3,
            'PVE-3', fontsize=14, fontweight='bold', ha='center', color='#e74c3c')
    ax.text(x3 + box_width/2, y3 + 1.3,
            'Semantic\nCoherence', fontsize=11, ha='center', fontweight='bold')
    ax.text(x3 + box_width/2, y3 + 0.6,
            'Measure: ⟨m₁|m₂⟩\ninner product', fontsize=9, ha='center')
    ax.text(x3 + box_width/2, y3 + 0.1,
            '64-ch EEG', fontsize=8, ha='center', style='italic')

    # Flow diagram below
    flow_y = 6.5

    # Step 1: Preparation
    step1 = FancyBboxPatch((1, flow_y), 8, 0.8,
                           boxstyle="round,pad=0.1",
                           edgecolor='#34495e', facecolor='#ecf0f1', linewidth=2)
    ax.add_patch(step1)
    ax.text(5, flow_y + 0.4, 'Step 1: Subject Preparation & Baseline Measurement',
            fontsize=11, ha='center', fontweight='bold')

    # Arrow down
    ax.annotate('', xy=(5, flow_y - 0.1), xytext=(5, flow_y - 0.6),
                arrowprops=dict(arrowstyle='->', lw=2, color='#34495e'))

    # Step 2: Stimulus Presentation
    step2_y = flow_y - 1.5
    step2 = FancyBboxPatch((1, step2_y), 8, 0.8,
                           boxstyle="round,pad=0.1",
                           edgecolor='#34495e', facecolor='#ecf0f1', linewidth=2)
    ax.add_patch(step2)
    ax.text(5, step2_y + 0.4, 'Step 2: Controlled Stimulus (Visual/Auditory/Semantic)',
            fontsize=11, ha='center', fontweight='bold')

    # Arrow down
    ax.annotate('', xy=(5, step2_y - 0.1), xytext=(5, step2_y - 0.6),
                arrowprops=dict(arrowstyle='->', lw=2, color='#34495e'))

    # Step 3: Data Acquisition
    step3_y = step2_y - 1.5
    step3 = FancyBboxPatch((1, step3_y), 8, 0.8,
                           boxstyle="round,pad=0.1",
                           edgecolor='#34495e', facecolor='#ecf0f1', linewidth=2)
    ax.add_patch(step3)
    ax.text(5, step3_y + 0.4, 'Step 3: Multi-Modal Data Acquisition (EEG, fMRI, Behavioral)',
            fontsize=11, ha='center', fontweight='bold')

    # Three branches for the 3 protocols
    branch_y = step3_y - 1.2

    # Branch arrows
    ax.annotate('', xy=(x1 + box_width/2, branch_y - 0.1), xytext=(5, step3_y - 0.1),
                arrowprops=dict(arrowstyle='->', lw=1.5, color='#27ae60', linestyle='--'))
    ax.annotate('', xy=(x2 + box_width/2, branch_y - 0.1), xytext=(5, step3_y - 0.1),
                arrowprops=dict(arrowstyle='->', lw=1.5, color='#3498db', linestyle='--'))
    ax.annotate('', xy=(x3 + box_width/2, branch_y - 0.1), xytext=(5, step3_y - 0.1),
                arrowprops=dict(arrowstyle='->', lw=1.5, color='#e74c3c', linestyle='--'))

    # Analysis boxes for each protocol
    analysis_y = branch_y - 1.5

    # Analysis 1
    analysis1 = FancyBboxPatch((x1, analysis_y), box_width, 1.0,
                               boxstyle="round,pad=0.1",
                               edgecolor='#27ae60', facecolor='#e8f8f5', linewidth=2)
    ax.add_patch(analysis1)
    ax.text(x1 + box_width/2, analysis_y + 0.7, 'PLV Analysis', fontsize=10, ha='center', fontweight='bold')
    ax.text(x1 + box_width/2, analysis_y + 0.35, r'$PLV = |\langle e^{i\Delta\phi}\rangle|$', fontsize=9, ha='center')

    # Analysis 2
    analysis2 = FancyBboxPatch((x2, analysis_y), box_width, 1.0,
                               boxstyle="round,pad=0.1",
                               edgecolor='#3498db', facecolor='#ebf5fb', linewidth=2)
    ax.add_patch(analysis2)
    ax.text(x2 + box_width/2, analysis_y + 0.7, 'Spectral Shift', fontsize=10, ha='center', fontweight='bold')
    ax.text(x2 + box_width/2, analysis_y + 0.35, r'$\Delta f = f_{post} - f_{pre}$', fontsize=9, ha='center')

    # Analysis 3
    analysis3 = FancyBboxPatch((x3, analysis_y), box_width, 1.0,
                               boxstyle="round,pad=0.1",
                               edgecolor='#e74c3c', facecolor='#fdedec', linewidth=2)
    ax.add_patch(analysis3)
    ax.text(x3 + box_width/2, analysis_y + 0.7, 'Semantic Dist.', fontsize=10, ha='center', fontweight='bold')
    ax.text(x3 + box_width/2, analysis_y + 0.35, r'$d(m_1, m_2) = \|\vec{m}_1 - \vec{m}_2\|$', fontsize=9, ha='center')

    # Convergence to final result
    result_y = analysis_y - 1.5

    # Arrows converging
    ax.annotate('', xy=(5, result_y + 0.5), xytext=(x1 + box_width/2, analysis_y - 0.1),
                arrowprops=dict(arrowstyle='->', lw=1.5, color='#27ae60'))
    ax.annotate('', xy=(5, result_y + 0.5), xytext=(x2 + box_width/2, analysis_y - 0.1),
                arrowprops=dict(arrowstyle='->', lw=1.5, color='#3498db'))
    ax.annotate('', xy=(5, result_y + 0.5), xytext=(x3 + box_width/2, analysis_y - 0.1),
                arrowprops=dict(arrowstyle='->', lw=1.5, color='#e74c3c'))

    # Final result box
    result = FancyBboxPatch((2.5, result_y - 0.5), 5, 1.0,
                            boxstyle="round,pad=0.15",
                            edgecolor='#9b59b6', facecolor='#f4ecf7', linewidth=3)
    ax.add_patch(result)
    ax.text(5, result_y + 0.2, 'Integrated Validation Result', fontsize=12, ha='center', fontweight='bold')
    ax.text(5, result_y - 0.2, 'Φ correlation with subjective experience', fontsize=10, ha='center', style='italic')

    # Footer with expected outcomes
    footer_y = 0.5
    ax.text(5, footer_y, 'Expected Outcomes: (1) PLV > 0.6 for shared semantics  (2) Δf proportional to cognitive load  (3) d(m₁,m₂) < 0.3 for similar meanings',
            fontsize=9, ha='center', bbox=dict(boxstyle='round', facecolor='#ffffcc', alpha=0.8))

    plt.tight_layout()
    plt.savefig('figures/pve_protocol_flowchart.png', dpi=300, bbox_inches='tight')
    print("✅ Figure 3 saved: figures/pve_protocol_flowchart.png")
    plt.close()

if __name__ == "__main__":
    print("\n╔══════════════════════════════════════════════════════════════════╗")
    print("║  Generating Figures for Paper #5: Cálculo de Significados       ║")
    print("╚══════════════════════════════════════════════════════════════════╝\n")

    # Create figures directory if it doesn't exist
    import os
    os.makedirs('figures', exist_ok=True)

    print("Generating Figure 1: MOM Architecture...")
    figure1_mom_architecture()

    print("Generating Figure 2: Fourier Spectrum of Cognitive Trajectories...")
    figure2_fourier_spectrum()

    print("Generating Figure 3: PVE Protocol Flowchart...")
    figure3_pve_protocol()

    print("\n╔══════════════════════════════════════════════════════════════════╗")
    print("║                  ALL FIGURES GENERATED ✅                        ║")
    print("╚══════════════════════════════════════════════════════════════════╝\n")

    print("Location: ./figures/")
    print("  - mom_architecture.png")
    print("  - fourier_cognitive_spectrum.png")
    print("  - pve_protocol_flowchart.png")
