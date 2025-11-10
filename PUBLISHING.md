# Guía de Publicación en crates.io

Esta guía te ayudará a publicar las librerías CORTEXIA en crates.io.

## ⚠️ Estado Actual

**Las librerías están CASI listas para publicación. Necesitas hacer lo siguiente:**

## 📋 Checklist Pre-Publicación

### 1. Obtener Token de crates.io

```bash
# Ve a https://crates.io/me
# Genera un token de API
# Luego ejecuta:
cargo login
# Pega tu token cuando te lo pida
```

### 2. Problema Actual: Path Dependencies

**PROBLEMA**: Las librerías usan `path = "../..."` en sus dependencias internas, lo cual NO funciona en crates.io.

**Necesitas cambiar esto ANTES de publicar:**

#### En `neural-dynamics/Cargo.toml`:
```toml
# ACTUAL (no funciona en crates.io):
[dependencies]
hodgkin-huxley = { path = "../hodgkin-huxley" }
synapse-models = { path = "../synapse-models" }

# CAMBIAR A (después de publicar las otras):
[dependencies]
hodgkin-huxley = "0.1.0"
synapse-models = "0.1.0"
```

#### En `cortexia/Cargo.toml`:
```toml
# ACTUAL (no funciona en crates.io):
[dependencies]
hodgkin-huxley = { path = "../hodgkin-huxley" }
iit = { path = "../iit" }
tda = { path = "../tda" }
synapse-models = { path = "../synapse-models" }
neural-dynamics = { path = "../neural-dynamics" }

# CAMBIAR A (después de publicar las otras):
[dependencies]
hodgkin-huxley = "0.1.0"
iit = "0.1.0"
tda = "0.1.0"
synapse-models = "0.1.0"
neural-dynamics = "0.1.0"
```

## 🚀 Orden de Publicación

**IMPORTANTE**: Debes publicar en este orden (de menos a más dependencias):

### Paso 1: Publicar Librerías Base (sin dependencias internas)

```bash
# 1. hodgkin-huxley (no depende de otras)
cd hodgkin-huxley
cargo publish --dry-run  # Verificar primero
cargo publish

# 2. iit (no depende de otras)
cd ../iit
cargo publish --dry-run
cargo publish

# 3. tda (no depende de otras)
cd ../tda
cargo publish --dry-run
cargo publish

# 4. synapse-models (no depende de otras)
cd ../synapse-models
cargo publish --dry-run
cargo publish
```

### Paso 2: Actualizar neural-dynamics

```bash
cd ../neural-dynamics
```

**Edita `Cargo.toml` y cambia:**
```toml
[dependencies]
hodgkin-huxley = "0.1.0"  # En lugar de { path = "../hodgkin-huxley" }
synapse-models = "0.1.0"  # En lugar de { path = "../synapse-models" }
# ... resto de dependencias normales
```

```bash
cargo build --release  # Verificar que compila
cargo test             # Verificar que tests pasan
cargo publish --dry-run
cargo publish
```

### Paso 3: Actualizar cortexia (meta-crate)

```bash
cd ../cortexia
```

**Edita `Cargo.toml` y cambia:**
```toml
[dependencies]
hodgkin-huxley = "0.1.0"
iit = "0.1.0"
tda = "0.1.0"
synapse-models = "0.1.0"
neural-dynamics = "0.1.0"
# ... resto de dependencias normales
```

```bash
cargo build --release
cargo test
cargo publish --dry-run
cargo publish
```

## 🔧 Script Automatizado de Verificación

```bash
#!/bin/bash
# verify_publish.sh

echo "Verificando que todo está listo para publicación..."

# Verificar que tenemos token de cargo
if ! cargo login --help &> /dev/null; then
    echo "❌ Error: cargo login no disponible"
    exit 1
fi

# Verificar cada crate
CRATES=("hodgkin-huxley" "iit" "tda" "synapse-models")

for crate in "${CRATES[@]}"; do
    echo ""
    echo "📦 Verificando $crate..."
    cd $crate

    # Verificar que compila
    if ! cargo build --release &> /dev/null; then
        echo "❌ $crate no compila"
        exit 1
    fi

    # Verificar que tests pasan
    if ! cargo test &> /dev/null; then
        echo "⚠️  $crate tiene tests fallidos"
    fi

    # Dry-run de publicación
    if ! cargo publish --dry-run &> /dev/null; then
        echo "❌ $crate falló dry-run"
        exit 1
    fi

    echo "✅ $crate está listo"
    cd ..
done

echo ""
echo "✅ Las librerías base están listas para publicación"
echo ""
echo "⚠️  RECUERDA: Debes actualizar las dependencias de:"
echo "   - neural-dynamics"
echo "   - cortexia"
echo "   Después de publicar las librerías base."
```

## 📝 Checklist Final

Antes de publicar, verifica:

- [ ] Tienes token de crates.io configurado (`cargo login`)
- [ ] LICENSE-MIT existe en el workspace
- [ ] LICENSE-APACHE existe en el workspace
- [ ] Todos los README.md están completos
- [ ] Las 4 librerías base compilan: `cargo build --release`
- [ ] Tests pasan (o conoces cuáles fallan y por qué)
- [ ] Has hecho commit de todos los cambios
- [ ] Has pusheado a GitHub

## 🎯 Comandos Rápidos

```bash
# Verificar todas las librerías
cargo build --workspace --release

# Ejecutar todos los tests
cargo test --workspace

# Ver qué se incluirá en cada publicación
cargo package --list --package hodgkin-huxley
cargo package --list --package iit
cargo package --list --package tda
cargo package --list --package synapse-models

# Dry-run completo
cargo publish --dry-run --package hodgkin-huxley
cargo publish --dry-run --package iit
cargo publish --dry-run --package tda
cargo publish --dry-run --package synapse-models
```

## ⏱️ Tiempo Estimado

- Configuración inicial: 5 minutos
- Publicación de 4 librerías base: 10 minutos
- Actualizar dependencias: 5 minutos
- Publicar neural-dynamics: 3 minutos
- Publicar cortexia: 3 minutos

**Total**: ~30 minutos

## 🆘 Troubleshooting

### Error: "repository not found"
Las URLs de repositorio apuntan a `https://github.com/cortexia/...` pero deberían ser `https://github.com/Yatrogenesis/cortexia-workspace/tree/main/CRATE_NAME`

**Solución**: Actualizar cada `Cargo.toml`:
```toml
repository = "https://github.com/Yatrogenesis/cortexia-workspace/tree/main/hodgkin-huxley"
```

### Error: "failed to verify package"
Puede haber archivos que causan problemas.

**Solución**: Agregar exclusiones en `Cargo.toml`:
```toml
exclude = [
    "target/",
    ".git/",
    "*.swp",
    ".DS_Store"
]
```

### Error: "crate name already exists"
El nombre ya está tomado en crates.io.

**Solución**: Cambiar el nombre o agregar prefijo:
```toml
name = "cortexia-hodgkin-huxley"
```

## 📚 Referencias

- [Guía oficial de publicación en crates.io](https://doc.rust-lang.org/cargo/reference/publishing.html)
- [Manifest Format](https://doc.rust-lang.org/cargo/reference/manifest.html)
- [Workspaces](https://doc.rust-lang.org/book/ch14-03-cargo-workspaces.html)

## ✅ Una vez publicado

Las librerías estarán disponibles en:
- https://crates.io/crates/hodgkin-huxley
- https://crates.io/crates/iit
- https://crates.io/crates/tda
- https://crates.io/crates/synapse-models
- https://crates.io/crates/neural-dynamics
- https://crates.io/crates/cortexia

Y los usuarios podrán instalarlas con:
```bash
cargo add cortexia
```

---

**Nota**: Este proceso puede requerir intervención manual para resolver problemas específicos.
