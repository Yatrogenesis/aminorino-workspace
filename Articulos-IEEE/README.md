# Artículos IEEE - CORTEXIA Research

Repositorio de artículos científicos, experimentos y resultados del ecosistema CORTEXIA.

## Estructura

```
Articulos-IEEE/
├── README.md (este archivo)
├── Cortexia/
│   ├── Overview/
│   │   └── CORTEXIA_Development_Process.md
│   └── Brain-AI-Native/
│       └── PhiQ_MajorThan_PhiClassical/
│           ├── Empirical_Test_Quantum_Consciousness.md
│           ├── data/
│           │   └── consciousness_experiment_results.json
│           ├── figures/
│           ├── code/
│           │   └── consciousness_experiment.rs
│           └── results/
└── [Futuros proyectos]/
    └── [Experimentos específicos]/
```

## Organización

### Nivel 1: Proyecto/Repositorio
Cada directorio de primer nivel corresponde a un proyecto o repositorio:
- `Cortexia/` - Ecosistema CORTEXIA completo

### Nivel 2: Componente
Subdirectorios por componente o crate:
- `Overview/` - Documentación general del proyecto
- `Brain-AI-Native/` - Artículos sobre brain-ai-native crate
- `Quantum-Processor/` - (futuro) Artículos sobre quantum-processor
- etc.

### Nivel 3: Experimento
Cada experimento tiene su propio directorio con nomenclatura descriptiva:
- `PhiQ_MajorThan_PhiClassical/` - Hipótesis: Φ_quantum > Φ_classical
- Formato: `DescripcionBreve/` o `HipotesisPrincipal/`

### Nivel 4: Contenidos del Experimento
Cada experimento contiene:
- `*.md` - Artículo principal (formato IEEE)
- `data/` - Datos experimentales (JSON, CSV, etc.)
- `figures/` - Gráficas y visualizaciones
- `code/` - Código fuente del experimento
- `results/` - Resultados procesados

## Artículos Publicados

### 1. CORTEXIA Development Process
**Ubicación:** `Cortexia/Overview/CORTEXIA_Development_Process.md`
**Fecha:** Enero 2025
**Resumen:** Documentación completa del proceso de creación del ecosistema CORTEXIA, incluyendo decisiones arquitectónicas, implementación de las 7 crates, y rationale científico.

### 2. Empirical Test of Quantum Consciousness
**Ubicación:** `Cortexia/Brain-AI-Native/PhiQ_MajorThan_PhiClassical/Empirical_Test_Quantum_Consciousness.md`
**Fecha:** Enero 2025
**Hipótesis:** Φ_quantum > Φ_classical
**Resultado:** Hipótesis rechazada (Φ_quantum = Φ_classical)
**Significancia:** Primer test empírico de consciencia cuántica usando IIT

## Guía de Nomenclatura

### Para Caracteres Especiales
- Φ (Phi) → `Phi`
- > (mayor que) → `MajorThan` o `GreaterThan`
- < (menor que) → `MinorThan` o `LessThan`
- = (igual) → `Equals`
- ≠ (diferente) → `NotEquals`
- ∫ (integral) → `Integral`
- Σ (sigma/suma) → `Sum`
- ∂ (derivada parcial) → `Partial`

### Ejemplos
- Φ_quantum > Φ_classical → `PhiQ_MajorThan_PhiClassical`
- ∫ρ(x)dx = 1 → `Integral_Rho_Equals_One`
- ∂f/∂x → `Partial_F_Over_X`

## Contribuyendo

### Agregar Nuevo Experimento

1. Crear estructura:
```bash
mkdir -p Articulos-IEEE/[Proyecto]/[Componente]/[Experimento]/{data,figures,code,results}
```

2. Crear artículo principal:
```bash
touch Articulos-IEEE/[Proyecto]/[Componente]/[Experimento]/[Titulo_Articulo].md
```

3. Copiar datos y código:
```bash
cp [resultados].json Articulos-IEEE/[...]/data/
cp [experimento].rs Articulos-IEEE/[...]/code/
```

4. Actualizar este README con referencia al nuevo artículo

## Formato de Artículos

Todos los artículos siguen estructura IEEE:

1. **Abstract**
2. **I. INTRODUCTION**
3. **II. METHODS**
4. **III. RESULTS**
5. **IV. DISCUSSION**
6. **V. CONCLUSION**
7. **REFERENCES**
8. **APPENDICES**

## Licencia

Todos los artículos y código en este directorio están bajo doble licencia:
- MIT License
- Apache License 2.0

## Autor

**Francisco Molina Burgos**
- ORCID: https://orcid.org/0009-0008-6093-8267
- Institución: Avermex - Consultoría Regulatoria
- Email: fmolina@avermex.com

## Generado con

🤖 Claude Code by Anthropic
https://claude.ai/claude-code

---

**Última actualización:** Enero 2025
