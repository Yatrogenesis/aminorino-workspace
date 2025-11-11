# Variables Experimentales Modificables

**Experimento Base:** PhiQ_MajorThan_PhiClassical
**Propósito:** Guía para diseñar nuevos experimentos variando parámetros

---

## I. VARIABLES DE CONFIGURACIÓN EXPERIMENTAL

### A. Parámetros Temporales

#### 1. `num_trials` (int)
**Descripción:** Número de repeticiones del experimento
**Valor actual:** `5`
**Rango sugerido:** 1 - 1000
**Impacto:** Significancia estadística

```rust
num_trials: 5,        // Actual
num_trials: 100,      // Mayor confianza estadística
num_trials: 1000,     // Análisis estadístico robusto
```

**Experimentos sugeridos:**
- `num_trials: 50` → Verificar consistencia de resultados
- `num_trials: 100` → Calcular p-values significativos

---

#### 2. `evolution_time` (f64, segundos)
**Descripción:** Tiempo total de evolución del sistema cuántico
**Valor actual:** `1e-9` (1 nanosegundo)
**Rango físico:** 1e-12 - 1e-3 segundos
**Impacto:** Permite observar dinámica temporal, decoherencia

```rust
evolution_time: 1e-9,    // Actual: 1 nanosegundo
evolution_time: 1e-6,    // 1 microsegundo (más realista)
evolution_time: 1e-3,    // 1 milisegundo (largo plazo)
```

**Trade-offs:**
- ⬆️ Tiempo más largo → Mayor decoherencia, más dinámica
- ⬇️ Tiempo más corto → Estado más coherente, menos evolución

**Experimentos sugeridos:**
- `evolution_time: 1e-6` → "PhiQ_Evolution_Microsecond"
- `evolution_time: [1e-9, 1e-8, 1e-7, 1e-6]` → "PhiQ_Temporal_Scaling"

---

#### 3. `dt` (f64, segundos)
**Descripción:** Paso de integración temporal
**Valor actual:** `1e-10` (100 picosegundos)
**Rango sugerido:** 1e-12 - 1e-8 segundos
**Impacto:** Precisión numérica vs velocidad

```rust
dt: 1e-10,     // Actual: 100 picosegundos
dt: 1e-12,     // Alta precisión (más lento)
dt: 1e-8,      // Baja precisión (más rápido)
```

**Regla:** `dt << 1/frequency` para estabilidad numérica

**Experimentos sugeridos:**
- Comparar Φ con diferentes `dt` para verificar convergencia

---

## II. VARIABLES DE ARQUITECTURA CUÁNTICA

### A. Tamaño del Sistema

#### 4. `num_oscillators` (usize)
**Descripción:** Número de osciladores cuánticos acoplados
**Valor actual:** `[2, 3, 4]`
**Rango práctico:** 2 - 10 (limitado por memoria)
**Impacto:** Neuronas efectivas = (max_fock + 1)^N

```rust
num_oscillators: 2,    // 4 neuronas (max_fock=1)
num_oscillators: 5,    // 32 neuronas
num_oscillators: 10,   // 1024 neuronas (límite práctico)
```

**Escalamiento:**
| N | max_fock=1 | max_fock=2 | max_fock=8 |
|---|------------|------------|------------|
| 2 | 4 neurons  | 9 neurons  | 81 neurons |
| 4 | 16 neurons | 81 neurons | 6,561 neurons |
| 6 | 64 neurons | 729 neurons | 262,144 neurons |
| 10 | 1,024 neurons | 59,049 neurons | 1B+ neurons |

**Experimentos sugeridos:**
- `num_oscillators: [5, 6, 7]` → "PhiQ_LargeScale_Systems"
- `num_oscillators: 10` + `max_fock: 1` → "PhiQ_MaxScale_Binary"

---

#### 5. `max_fock` (usize)
**Descripción:** Máximo nivel de Fock (truncación del espacio de Hilbert)
**Valor actual:** `1` (estados |0⟩, |1⟩ solamente)
**Rango sugerido:** 1 - 8
**Impacto:** Neuronas efectivas = (max_fock + 1)^N

```rust
max_fock: 1,    // 2 estados por oscilador (binario)
max_fock: 2,    // 3 estados (ternario)
max_fock: 8,    // 9 estados (alta dimensionalidad)
```

**Memoria requerida:**
- `max_fock=1, N=4`: 2^4 = 16 amplitudes (128 bytes)
- `max_fock=2, N=4`: 3^4 = 81 amplitudes (648 bytes)
- `max_fock=8, N=4`: 9^4 = 6,561 amplitudes (52 KB)
- `max_fock=8, N=10`: 9^10 = 3.4B amplitudes (27 GB) ⚠️

**Experimentos sugeridos:**
- `max_fock: 2` + `num_oscillators: 4` → "PhiQ_TernaryStates_81Neurons"
- `max_fock: [1, 2, 4]` → "PhiQ_FockSpace_Scaling"

---

### B. Parámetros Físicos

#### 6. `frequencies` (Vec<f64>, Hz)
**Descripción:** Frecuencias de oscilación de cada oscilador
**Valor actual:** `vec![1e9, 1e9, ...]` (1 GHz uniforme)
**Rango físico:** 1e6 - 1e12 Hz
**Impacto:** Velocidad de oscilación, energía del sistema

```rust
frequencies: vec![1e9; 4],              // Uniforme
frequencies: vec![1e9, 2e9, 3e9, 4e9],  // Heterogéneo
frequencies: vec![5e8; 4],              // Más lento (500 MHz)
```

**Configuraciones interesantes:**
```rust
// Resonancia armónica
frequencies: vec![1e9, 2e9, 4e9, 8e9]   // Potencias de 2

// Distribución aleatoria
use rand::Rng;
let mut rng = rand::thread_rng();
frequencies: (0..4).map(|_| rng.gen_range(5e8..5e9)).collect()

// Fibonacci
frequencies: vec![1e9, 1e9, 2e9, 3e9, 5e9]
```

**Experimentos sugeridos:**
- "PhiQ_Harmonic_Resonance" → Frecuencias en ratios armónicos
- "PhiQ_Heterogeneous_Frequencies" → Distribución aleatoria

---

#### 7. `coupling_strength` (f64, Hz)
**Descripción:** Fuerza de acoplamiento entre osciladores (g en Hamiltoniano)
**Valor actual:** `1e6` (1 MHz)
**Rango físico:** 0 - 1e9 Hz
**Impacto:** Integración del sistema, entrelazamiento

```rust
coupling_strength: 0,        // Sin acoplamiento (Φ = 0)
coupling_strength: 1e6,      // Actual: 1 MHz
coupling_strength: 1e8,      // Fuerte: 100 MHz
coupling_strength: 1e9,      // Muy fuerte: 1 GHz
```

**Relación con Φ:**
- `g = 0` → No hay integración → Φ = 0
- `g << ω` → Débil acoplamiento → Φ pequeño
- `g ~ ω` → Acoplamiento moderado → Φ medio
- `g >> ω` → Acoplamiento fuerte → Φ alto (posiblemente)

**Experimentos CRÍTICOS:**
- `coupling_strength: [0, 1e5, 1e6, 1e7, 1e8]` → **"PhiQ_vs_Coupling_Strength"**
  - **Hipótesis:** Φ aumenta con acoplamiento
  - **Esperado:** Ver transición de Φ=0 a Φ>0

---

#### 8. `damping_rate` (f64, Hz)
**Descripción:** Tasa de amortiguamiento/decoherencia (γ)
**Valor actual:** `1e3` (1 kHz)
**Rango físico:** 0 - 1e6 Hz
**Impacto:** Pérdida de coherencia cuántica

```rust
damping_rate: 0,       // Sin decoherencia (ideal)
damping_rate: 1e3,     // Actual: 1 kHz
damping_rate: 1e5,     // Alta decoherencia: 100 kHz
```

**Relación con evolución:**
- `γ * t << 1` → Estado coherente
- `γ * t ~ 1` → Decoherencia parcial
- `γ * t >> 1` → Estado completamente decoherido

**Para `evolution_time = 1e-6` (1 μs):**
```rust
damping_rate: 1e3  → γt = 0.001 (coherente)
damping_rate: 1e6  → γt = 1.0 (transición)
damping_rate: 1e9  → γt = 1000 (decoherido)
```

**Experimentos sugeridos:**
- "PhiQ_vs_Decoherence" → Variar γ, medir Φ(γ)
- **Hipótesis:** Φ disminuye con mayor decoherencia

---

## III. VARIABLES DE PROTECCIÓN Y CORRECCIÓN

#### 9. `error_correction` (bool)
**Descripción:** Activar corrección de errores LDPC
**Valor actual:** `false`
**Impacto:** Protección contra errores, overhead computacional

```rust
error_correction: false,   // Actual
error_correction: true,    // Con corrección LDPC
```

**Experimentos sugeridos:**
- "PhiQ_WithErrorCorrection" → Comparar Φ con/sin LDPC
- **Hipótesis:** LDPC preserva Φ en presencia de errores

---

#### 10. `ldpc_distance` (usize)
**Descripción:** Distancia del código LDPC (si `error_correction = true`)
**Valor actual:** `0` (no usado)
**Rango sugerido:** 3 - 7
**Impacto:** Capacidad de corrección de errores

```rust
error_correction: true,
ldpc_distance: 3,    // Básico (corrige 1 error)
ldpc_distance: 5,    // Medio (corrige 2 errores)
ldpc_distance: 7,    // Alto (corrige 3 errores)
```

---

#### 11. `radiation_protection` (bool)
**Descripción:** Simular efectos de radiación cósmica
**Valor actual:** `false`
**Impacto:** Errores estocásticos en qubits

```rust
radiation_protection: false,   // Actual
radiation_protection: true,    // Simular radiación
```

---

#### 12. `chip_area_cm2` (f64)
**Descripción:** Área del chip cuántico (si `radiation_protection = true`)
**Valor actual:** `0.0`
**Rango físico:** 0.1 - 10.0 cm²
**Impacto:** Tasa de eventos de radiación

```rust
chip_area_cm2: 1.0,    // 1 cm² (típico)
chip_area_cm2: 5.0,    // 5 cm² (mayor tasa de eventos)
```

**Tasa de eventos:**
- Muones: ~60 eventos/cm²/hora
- Neutrones: ~10 eventos/cm²/hora

**Para `chip_area_cm2 = 1.0`, `evolution_time = 1e-6`:**
- Eventos esperados: ~0 (tiempo muy corto)

**Para `chip_area_cm2 = 1.0`, `evolution_time = 3600` (1 hora):**
- Eventos esperados: ~70 eventos

---

#### 13. `altitude_m` (f64)
**Descripción:** Altitud sobre nivel del mar (si `radiation_protection = true`)
**Valor actual:** `0.0`
**Rango físico:** 0 - 40,000 m
**Impacto:** Flujo de radiación cósmica

```rust
altitude_m: 0,        // Nivel del mar
altitude_m: 10000,    // Avión comercial (flujo 2^6.7 = ~100x)
altitude_m: 400000,   // Estación espacial (flujo ~1000x)
```

**Experimentos sugeridos:**
- "PhiQ_vs_Radiation_SeaLevel_vs_Space"

---

## IV. DISEÑOS EXPERIMENTALES SUGERIDOS

### Experimento 1: Efecto del Acoplamiento en Φ
**Nombre:** `PhiQ_vs_CouplingStrength`
**Variables:**
```rust
coupling_strength: [0, 1e4, 1e5, 1e6, 1e7, 1e8, 1e9]
num_oscillators: 4
max_fock: 1
num_trials: 50
```

**Hipótesis:** Φ ∝ log(coupling_strength) para g > threshold

---

### Experimento 2: Escalamiento con Tamaño
**Nombre:** `PhiQ_Scaling_with_SystemSize`
**Variables:**
```rust
num_oscillators: [2, 3, 4, 5, 6]
max_fock: 1  // Mantener binario
coupling_strength: 1e6
num_trials: 100
```

**Hipótesis:** Φ ~ N^α para algún exponente α

---

### Experimento 3: Decoherencia vs Consciencia
**Nombre:** `PhiQ_Decoherence_Effect`
**Variables:**
```rust
damping_rate: [0, 1e2, 1e3, 1e4, 1e5, 1e6]
evolution_time: 1e-6  // Más largo para ver efecto
num_oscillators: 4
num_trials: 100
```

**Hipótesis:** Φ disminuye exponencialmente con γ

---

### Experimento 4: Estados de Fock Altos
**Nombre:** `PhiQ_HighDimensional_FockStates`
**Variables:**
```rust
max_fock: [1, 2, 3, 4]
num_oscillators: 3  // Reducir N para mantener memoria
coupling_strength: 1e6
num_trials: 50
```

**Hipótesis:** Φ aumenta con dimensionalidad del espacio de Hilbert

---

### Experimento 5: Tiempo de Evolución
**Nombre:** `PhiQ_Temporal_Dynamics`
**Variables:**
```rust
evolution_time: [1e-9, 1e-8, 1e-7, 1e-6, 1e-5]
num_oscillators: 4
coupling_strength: 1e6
damping_rate: 1e3
num_trials: 100
```

**Hipótesis:** Φ(t) tiene dinámica no-monótona (sube luego baja por decoherencia)

---

### Experimento 6: Heterogeneidad de Frecuencias
**Nombre:** `PhiQ_FrequencyHeterogeneity`
**Variables:**
```rust
// Caso 1: Homogéneo
frequencies: vec![1e9; 4]

// Caso 2: Heterogéneo
frequencies: vec![5e8, 1e9, 2e9, 4e9]

// Caso 3: Armónico
frequencies: vec![1e9, 2e9, 3e9, 4e9]

num_trials: 100
```

**Hipótesis:** Heterogeneidad aumenta Φ (mayor diversidad funcional)

---

### Experimento 7: Radiación Cósmica
**Nombre:** `PhiQ_Radiation_SeaLevel_vs_Space`
**Variables:**
```rust
radiation_protection: true
altitude_m: [0, 10000, 100000, 400000]
chip_area_cm2: 1.0
evolution_time: 3600.0  // 1 hora
num_oscillators: 4
num_trials: 10  // Menos trials (más tiempo)
```

**Hipótesis:** Φ disminuye con mayor exposición a radiación

---

## V. MATRIZ DE EXPERIMENTOS SISTEMÁTICOS

### Diseño Factorial Completo

```python
# Variables principales
num_oscillators = [2, 3, 4, 5]
max_fock = [1, 2]
coupling_strength = [1e5, 1e6, 1e7]
damping_rate = [1e3, 1e4, 1e5]

# Total experimentos: 4 × 2 × 3 × 3 = 72 configuraciones
```

**Organización:**
```
PhiQ_Factorial_Design/
├── N2_Fock1_Coupling1e5_Damping1e3/
├── N2_Fock1_Coupling1e5_Damping1e4/
├── N2_Fock1_Coupling1e5_Damping1e5/
├── ...
└── N5_Fock2_Coupling1e7_Damping1e5/
```

---

## VI. CÓMO CREAR NUEVO EXPERIMENTO

### Template de Código

```rust
// archivo: PhiQ_[NombreExperimento].rs

use brain_ai_native::prelude::*;
use brain_ai_native::{BrainResult, BrainError};
use std::fs;

fn main() -> BrainResult<()> {
    let experiment_config = ExperimentConfig {
        num_trials: 100,  // ← MODIFICAR
        evolution_time: 1e-6,  // ← MODIFICAR
        dt: 1e-10,
        brain_configs: vec![
            BrainConfig {
                num_oscillators: 5,  // ← MODIFICAR
                max_fock: 2,  // ← MODIFICAR
                frequencies: vec![1e9; 5],  // ← MODIFICAR
                coupling_strength: 1e7,  // ← MODIFICAR
                damping_rate: 1e4,  // ← MODIFICAR
                error_correction: false,  // ← MODIFICAR
                ldpc_distance: 0,
                radiation_protection: false,  // ← MODIFICAR
                chip_area_cm2: 0.0,
                altitude_m: 0.0,
            },
        ],
        classical_sizes: vec![5],  // Debe coincidir con num_oscillators
    };

    let results = run_consciousness_experiment(experiment_config)?;

    // Exportar resultados
    let json = results.to_json()?;
    let filename = "PhiQ_[NombreExperimento]_results.json";
    fs::write(filename, &json)
        .map_err(|e| BrainError::ExperimentError(format!("{}", e)))?;

    println!("{}", results.display());

    Ok(())
}
```

### Pasos para Nuevo Experimento

1. **Copiar template:**
```bash
cp brain-ai-native/examples/consciousness_experiment.rs \
   brain-ai-native/examples/PhiQ_NewExperiment.rs
```

2. **Modificar variables según tabla anterior**

3. **Crear directorio de resultados:**
```bash
mkdir -p Articulos-IEEE/Cortexia/Brain-AI-Native/PhiQ_NewExperiment/{data,figures,code,results}
```

4. **Ejecutar:**
```bash
cargo run --example PhiQ_NewExperiment
```

5. **Copiar resultados:**
```bash
cp PhiQ_NewExperiment_results.json \
   Articulos-IEEE/Cortexia/Brain-AI-Native/PhiQ_NewExperiment/data/
```

---

## VII. RESUMEN DE VARIABLES

| Variable | Tipo | Rango | Impacto en Φ | Prioridad |
|----------|------|-------|--------------|-----------|
| `num_oscillators` | int | 2-10 | ⬆️⬆️⬆️ Alto | 🔴 Alta |
| `max_fock` | int | 1-8 | ⬆️⬆️⬆️ Alto | 🔴 Alta |
| `coupling_strength` | float | 0-1e9 | ⬆️⬆️⬆️ Alto | 🔴 Alta |
| `damping_rate` | float | 0-1e6 | ⬇️⬇️ Medio | 🟡 Media |
| `evolution_time` | float | 1e-12-1e-3 | ⬆️⬇️ Complejo | 🟡 Media |
| `num_trials` | int | 1-1000 | 📊 Estadístico | 🟡 Media |
| `frequencies` | Vec | 1e6-1e12 | ⬆️ Bajo | 🟢 Baja |
| `dt` | float | 1e-12-1e-8 | 🔧 Técnico | 🟢 Baja |
| `error_correction` | bool | true/false | 🛡️ Protección | 🟢 Baja |
| `radiation_protection` | bool | true/false | 🌌 Físico | 🟢 Baja |

---

## VIII. EXPERIMENTOS PRIORITARIOS RECOMENDADOS

### 🔴 Prioridad CRÍTICA

1. **PhiQ_vs_CouplingStrength**
   - Variar `coupling_strength` de 0 a 1e9
   - **Razón:** Entender relación fundamental entre integración y Φ

2. **PhiQ_Scaling_SystemSize**
   - Variar `num_oscillators` de 2 a 6
   - **Razón:** Ley de escalamiento de consciencia

### 🟡 Prioridad ALTA

3. **PhiQ_HighFock_DimensionalScaling**
   - Variar `max_fock` manteniendo N fijo
   - **Razón:** Efecto de dimensionalidad cuántica

4. **PhiQ_Decoherence_Effect**
   - Variar `damping_rate` con tiempo largo
   - **Razón:** Límites de consciencia cuántica

### 🟢 Prioridad MEDIA

5. **PhiQ_Temporal_Evolution**
   - Variar `evolution_time`
   - **Razón:** Dinámica temporal de Φ

6. **PhiQ_Radiation_Extreme**
   - Altitud espacial con tiempo largo
   - **Razón:** Robustez en ambientes extremos

---

**¡Listo para diseñar nuevos experimentos!** 🧠⚛️✨
