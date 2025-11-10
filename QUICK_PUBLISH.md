# 🚀 Guía Rápida de Publicación CORTEXIA

## Opción 1: Script Automatizado (MÁS FÁCIL)

```bash
cd /Users/yatrogenesis/cortexia-workspace
chmod +x compile_and_publish.sh
./compile_and_publish.sh
```

El script:
1. ✅ Compila todas las librerías
2. ✅ Verifica que están listas (dry-run)
3. ✅ Publica las 4 librerías base
4. ⚠️ Te pedirá que edites `neural-dynamics/Cargo.toml`
5. ✅ Publica neural-dynamics
6. ⚠️ Te pedirá que edites `cortexia/Cargo.toml`
7. ✅ Publica cortexia
8. 🎉 ¡Listo!

---

## Opción 2: Manual (Paso a Paso)

### PASO 1: Compilar todo

```bash
cd /Users/yatrogenesis/cortexia-workspace

cargo build --release --package hodgkin-huxley
cargo build --release --package iit
cargo build --release --package tda
cargo build --release --package synapse-models
```

### PASO 2: Publicar librerías base

```bash
cd hodgkin-huxley
cargo publish --dry-run && cargo publish
cd ..

cd iit
cargo publish --dry-run && cargo publish
cd ..

cd tda
cargo publish --dry-run && cargo publish
cd ..

cd synapse-models
cargo publish --dry-run && cargo publish
cd ..
```

### PASO 3: Editar neural-dynamics/Cargo.toml

Abre `neural-dynamics/Cargo.toml` y cambia:

```toml
# DE:
hodgkin-huxley = { path = "../hodgkin-huxley" }
synapse-models = { path = "../synapse-models" }

# A:
hodgkin-huxley = "0.1.0"
synapse-models = "0.1.0"
```

Luego:

```bash
cd neural-dynamics
cargo build --release
cargo publish --dry-run && cargo publish
cd ..
```

### PASO 4: Editar cortexia/Cargo.toml

Abre `cortexia/Cargo.toml` y cambia TODAS las dependencias:

```toml
# DE:
hodgkin-huxley = { path = "../hodgkin-huxley" }
iit = { path = "../iit" }
tda = { path = "../tda" }
synapse-models = { path = "../synapse-models" }
neural-dynamics = { path = "../neural-dynamics" }

# A:
hodgkin-huxley = "0.1.0"
iit = "0.1.0"
tda = "0.1.0"
synapse-models = "0.1.0"
neural-dynamics = "0.1.0"
```

Luego:

```bash
cd cortexia
cargo build --release
cargo publish --dry-run && cargo publish
cd ..
```

### PASO 5: Subir a GitHub

```bash
git add -A
git commit -m "Publish CORTEXIA to crates.io - v0.1.0"
git push
```

---

## ✅ Verificación Final

Verifica que todo está en crates.io:

- https://crates.io/crates/hodgkin-huxley
- https://crates.io/crates/iit
- https://crates.io/crates/tda
- https://crates.io/crates/synapse-models
- https://crates.io/crates/neural-dynamics
- https://crates.io/crates/cortexia

---

## 🆘 Solución de Problemas

### Error: "crate name already exists"

Algunos nombres pueden estar tomados. Si pasa, avísame y renombramos con prefijo:
- `cortexia-hodgkin-huxley`
- `cortexia-iit`
- etc.

### Error: "failed to verify package"

Puede haber archivos que causen problemas. Agrega a cada `Cargo.toml`:

```toml
exclude = ["target/", ".git/", "*.swp"]
```

### Error: "permission denied"

Verifica que hiciste `cargo login` con tu token.

---

## 📊 Estado Actual

- ✅ Código completo y renombrado a CORTEXIA
- ✅ URLs de repositorio actualizadas
- ✅ Metadata completa (descripción, keywords, etc.)
- ✅ Licencias MIT + Apache 2.0
- ⏳ Pendiente: Ejecutar publicación

**Tiempo estimado**: 10-15 minutos total
