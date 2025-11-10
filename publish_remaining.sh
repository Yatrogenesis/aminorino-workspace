#!/bin/bash
# Script para publicar neural-dynamics y cortexia después del rate limit

set -e

echo "🚀 Publishing remaining CORTEXIA libraries"
echo "=========================================="
echo ""

cd /Users/yatrogenesis/cortexia-workspace

# Publish neural-dynamics
echo "📦 Publishing neural-dynamics..."
cd neural-dynamics
~/.cargo/bin/cargo publish --allow-dirty
echo "✅ neural-dynamics published!"
echo ""
sleep 15

# Update cortexia dependencies
echo "📝 Updating cortexia/Cargo.toml..."
cd ../cortexia

# Backup original
cp Cargo.toml Cargo.toml.backup

# Update dependencies to use versions
cat > Cargo.toml << 'EOF'
[package]
name = "cortexia"
version.workspace = true
edition.workspace = true
authors.workspace = true
license.workspace = true
repository = "https://github.com/Yatrogenesis/cortexia-workspace"
description = "Complete neural-quantum consciousness bridge framework - Computational Orchestration for Reality Transformation: EXtended Intelligence Architecture"
keywords = ["consciousness", "neuroscience", "quantum", "ai", "cognitive"]
categories = ["science", "simulation"]
readme = "README.md"

[dependencies]
hodgkin-huxley = "0.1.0"
iit = "0.1.0"
tda = "0.1.0"
synapse-models = "0.1.0"
neural-dynamics = "0.1.0"

# Re-export core dependencies
nalgebra = { workspace = true }
ndarray = { workspace = true }
rayon = { workspace = true }
serde = { workspace = true }

[dev-dependencies]
approx = { workspace = true }
criterion = { workspace = true }
EOF

echo "✅ cortexia/Cargo.toml updated!"
echo ""

# Publish cortexia
echo "📦 Publishing cortexia..."
~/.cargo/bin/cargo publish --allow-dirty
echo "✅ cortexia published!"
echo ""

echo "🎉 All CORTEXIA libraries published successfully!"
echo ""
echo "Published crates:"
echo "  • hodgkin-huxley v0.1.0"
echo "  • iit v0.1.0"
echo "  • tda v0.1.0"
echo "  • synapse-models v0.1.0"
echo "  • neural-dynamics v0.1.0"
echo "  • cortexia v0.1.0"
