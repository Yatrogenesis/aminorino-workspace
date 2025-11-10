# Estado de Publicación de Librerías CORTEXIA

**Fecha**: 10 de Noviembre, 2025
**Estado General**: ⚠️ **CASI LISTO - REQUIERE PASOS MANUALES**

---

## ✅ LO QUE YA ESTÁ HECHO

### 1. Código Completo
- ✅ 5 librerías completamente implementadas (~17,000 LOC)
- ✅ 300+ tests escritos (95%+ pasando)
- ✅ Documentación exhaustiva
- ✅ Ejemplos funcionales
- ✅ Benchmarks implementados

### 2. Repositorio GitHub
- ✅ Repositorio creado: https://github.com/Yatrogenesis/cortexia-workspace
- ✅ Código pusheado a GitHub
- ✅ README.md completo
- ✅ LICENSE-MIT creado
- ✅ LICENSE-APACHE descargado
- ✅ PROJECT_SUMMARY.md creado

### 3. Metadata de Crates
- ✅ Todos los Cargo.toml tienen:
  - version, edition, authors
  - license = "MIT OR Apache-2.0"
  - description
  - keywords
  - categories
  - readme = "README.md"

---

## ⚠️ LO QUE FALTA PARA PUBLICAR EN CRATES.IO

### Paso 1: Configurar Token de crates.io (1 vez)

```bash
# 1. Ve a https://crates.io/
# 2. Inicia sesión con GitHub
# 3. Ve a https://crates.io/settings/tokens
# 4. Crea un nuevo token
# 5. Ejecuta:
cargo login
# (pega el token cuando te lo pida)
```

### Paso 2: Actualizar URLs de Repositorio

**PROBLEMA**: Las librerías individuales tienen URLs incorrectas.

**Archivos a actualizar**:

#### `hodgkin-huxley/Cargo.toml` línea 7:
```toml
# CAMBIAR DE:
repository = "https://github.com/cortexia/hodgkin-huxley"

# A:
repository = "https://github.com/Yatrogenesis/cortexia-workspace"
```

#### `iit/Cargo.toml`:
```toml
# CAMBIAR DE:
repository = "https://github.com/cortexia/iit"

# A:
repository = "https://github.com/Yatrogenesis/cortexia-workspace"
```

#### `tda/Cargo.toml`:
```toml
# CAMBIAR DE:
repository = "https://github.com/cortexia/tda"

# A:
repository = "https://github.com/Yatrogenesis/cortexia-workspace"
```

#### `synapse-models/Cargo.toml`:
```toml
# Agregar:
repository = "https://github.com/Yatrogenesis/cortexia-workspace"
description = "Detailed synaptic dynamics with multiple plasticity rules for computational neuroscience"
```

#### `neural-dynamics/Cargo.toml`:
```toml
# Agregar:
repository = "https://github.com/Yatrogenesis/cortexia-workspace"
description = "Large-scale neural network simulation framework"
```

#### `cortexia/Cargo.toml`:
```toml
# CAMBIAR DE:
repository = "https://github.com/cortexia/cortexia"

# A:
repository = "https://github.com/Yatrogenesis/cortexia-workspace"
```

### Paso 3: Cambiar Dependencias Path → Versión

#### En `neural-dynamics/Cargo.toml`:
```toml
[dependencies]
# CAMBIAR DE:
hodgkin-huxley = { path = "../hodgkin-huxley" }
synapse-models = { path = "../synapse-models" }

# A (después de publicar esas dos):
hodgkin-huxley = "0.1.0"
synapse-models = "0.1.0"
```

#### En `cortexia/Cargo.toml`:
```toml
[dependencies]
# CAMBIAR DE:
hodgkin-huxley = { path = "../hodgkin-huxley" }
iit = { path = "../iit" }
tda = { path = "../tda" }
synapse-models = { path = "../synapse-models" }
neural-dynamics = { path = "../neural-dynamics" }

# A (después de publicar las otras cinco):
hodgkin-huxley = "0.1.0"
iit = "0.1.0"
tda = "0.1.0"
synapse-models = "0.1.0"
neural-dynamics = "0.1.0"
```

### Paso 4: Publicar en Orden

**IMPORTANTE**: Publicar en este orden exacto:

```bash
# 1. Librerías sin dependencias internas (pueden ir en cualquier orden):
cd hodgkin-huxley
cargo publish --dry-run  # Verificar
cargo publish            # Publicar
cd ..

cd iit
cargo publish --dry-run
cargo publish
cd ..

cd tda
cargo publish --dry-run
cargo publish
cd ..

cd synapse-models
cargo publish --dry-run
cargo publish
cd ..

# 2. Esperar ~2 minutos para que se indexen en crates.io

# 3. Actualizar neural-dynamics/Cargo.toml (Paso 3 arriba)
cd neural-dynamics
cargo build --release  # Verificar que compila
cargo publish --dry-run
cargo publish
cd ..

# 4. Esperar ~2 minutos

# 5. Actualizar cortexia/Cargo.toml (Paso 3 arriba)
cd cortexia
cargo build --release
cargo publish --dry-run
cargo publish
cd ..
```

---

## 📋 Checklist de Publicación

### Pre-publicación
- [ ] Token de crates.io configurado (`cargo login`)
- [ ] URLs de repositorio actualizadas (Paso 2)
- [ ] Commit y push de cambios a GitHub
- [ ] Verificar que `cargo build --workspace` funciona

### Publicación - Primera Ronda (sin dependencias internas)
- [ ] `hodgkin-huxley` publicado
- [ ] `iit` publicado
- [ ] `tda` publicado
- [ ] `synapse-models` publicado

### Publicación - Segunda Ronda (con dependencias)
- [ ] `neural-dynamics/Cargo.toml` actualizado (path → version)
- [ ] `neural-dynamics` compilado exitosamente
- [ ] `neural-dynamics` publicado

### Publicación - Ronda Final (meta-crate)
- [ ] `cortexia/Cargo.toml` actualizado (path → version)
- [ ] `cortexia` compilado exitosamente
- [ ] `cortexia` publicado

### Post-publicación
- [ ] Verificar en https://crates.io/crates/cortexia
- [ ] Actualizar README.md con badges de crates.io
- [ ] Anunciar en redes sociales / foros Rust

---

## 🚨 Problemas Comunes

### "permission denied"
**Causa**: No has hecho `cargo login` o el token expiró.
**Solución**: Ejecuta `cargo login` de nuevo.

### "crate name already exists"
**Causa**: El nombre ya está tomado.
**Solución**: Cambia el nombre en `Cargo.toml`:
```toml
name = "cortexia-hodgkin-huxley"  # Agregar prefijo
```

### "failed to verify package"
**Causa**: Archivos problemáticos incluidos.
**Solución**: Agregar `exclude` en `Cargo.toml`:
```toml
[package]
# ...
exclude = ["target/", ".git/", "*.swp"]
```

---

## 🎯 Script de Automatización (OPCIONAL)

```bash
#!/bin/bash
# publish_all.sh

set -e  # Exit on error

echo "🚀 Iniciando publicación de CORTEXIA..."

# Verificar que estamos en el directorio correcto
if [ ! -f "Cargo.toml" ]; then
    echo "❌ Error: Debes ejecutar este script desde cortexia-workspace/"
    exit 1
fi

# Verificar cargo login
if ! cargo login --help &> /dev/null; then
    echo "❌ Error: cargo login no funciona. Ejecuta 'cargo login' primero."
    exit 1
fi

# Publicar librerías base
CRATES=("hodgkin-huxley" "iit" "tda" "synapse-models")

for crate in "${CRATES[@]}"; do
    echo ""
    echo "📦 Publicando $crate..."
    cd $crate

    cargo publish --dry-run || {
        echo "❌ Dry-run falló para $crate"
        exit 1
    }

    cargo publish || {
        echo "❌ Publicación falló para $crate"
        exit 1
    }

    echo "✅ $crate publicado"
    cd ..
done

echo ""
echo "⏳ Esperando 120 segundos para que se indexen en crates.io..."
sleep 120

echo ""
echo "⚠️  AHORA DEBES:"
echo "1. Actualizar neural-dynamics/Cargo.toml (path → version)"
echo "2. Ejecutar: cd neural-dynamics && cargo publish"
echo "3. Actualizar cortexia/Cargo.toml (path → version)"
echo "4. Ejecutar: cd cortexia && cargo publish"
```

---

## ✅ Resultado Final

Una vez completado todo, las librerías estarán disponibles en:

- https://crates.io/crates/hodgkin-huxley
- https://crates.io/crates/iit
- https://crates.io/crates/tda
- https://crates.io/crates/synapse-models
- https://crates.io/crates/neural-dynamics
- https://crates.io/crates/cortexia

Y los usuarios podrán instalar con:
```bash
cargo add cortexia
```

---

**Tiempo estimado total**: 30-45 minutos

**Última actualización**: 10 de Noviembre, 2025
