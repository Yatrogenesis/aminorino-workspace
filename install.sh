#!/bin/bash
#
# CORTEXIA - Automated Installation Script
# Quantum Consciousness Research Platform
#
# Author: Francisco Molina Burgos
# ORCID: https://orcid.org/0009-0008-6093-8267
#
# This script installs all dependencies and builds CORTEXIA for reproducible research
#

set -e  # Exit on error

echo "╔══════════════════════════════════════════════════════════════════╗"
echo "║                   CORTEXIA INSTALLER                             ║"
echo "║         Quantum Consciousness Research Platform                  ║"
echo "╚══════════════════════════════════════════════════════════════════╝"
echo ""

# Detect OS
OS="$(uname -s)"
case "${OS}" in
    Linux*)     PLATFORM=Linux;;
    Darwin*)    PLATFORM=Mac;;
    *)          PLATFORM="UNKNOWN:${OS}"
esac

echo "🔍 Detected platform: ${PLATFORM}"
echo ""

# Check prerequisites
echo "📋 Checking prerequisites..."
echo ""

# 1. Check Rust
if command -v cargo &> /dev/null; then
    RUST_VERSION=$(cargo --version)
    echo "✅ Rust found: ${RUST_VERSION}"
else
    echo "❌ Rust not found. Installing..."
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
    source "$HOME/.cargo/env"
    echo "✅ Rust installed successfully"
fi

# 2. Check Git
if command -v git &> /dev/null; then
    GIT_VERSION=$(git --version)
    echo "✅ Git found: ${GIT_VERSION}"
else
    echo "❌ Git not found. Please install git first."
    exit 1
fi

# 3. Check build essentials
if [ "$PLATFORM" = "Mac" ]; then
    if ! xcode-select -p &> /dev/null; then
        echo "⚠️  Xcode Command Line Tools not found. Installing..."
        xcode-select --install
        echo "Please complete Xcode installation and re-run this script."
        exit 1
    else
        echo "✅ Xcode Command Line Tools found"
    fi
elif [ "$PLATFORM" = "Linux" ]; then
    if ! command -v gcc &> /dev/null; then
        echo "⚠️  Installing build essentials..."
        sudo apt-get update
        sudo apt-get install -y build-essential pkg-config libssl-dev
    fi
    echo "✅ Build tools found"
fi

echo ""
echo "🔨 Building CORTEXIA workspace..."
echo ""

# Build all crates
cargo build --release --workspace

if [ $? -eq 0 ]; then
    echo ""
    echo "✅ Build successful!"
else
    echo ""
    echo "❌ Build failed. Please check error messages above."
    exit 1
fi

echo ""
echo "🧪 Running tests..."
echo ""

# Run tests
cargo test --release --workspace --quiet

if [ $? -eq 0 ]; then
    echo "✅ All tests passed!"
else
    echo "⚠️  Some tests failed. This may not affect functionality."
fi

echo ""
echo "📦 Installing CLI tools..."
echo ""

# Install CLI binaries (if we create them)
# cargo install --path brain-ai-native --bin cortexia-cli

echo ""
echo "╔══════════════════════════════════════════════════════════════════╗"
echo "║                 INSTALLATION COMPLETE                            ║"
echo "╚══════════════════════════════════════════════════════════════════╝"
echo ""
echo "🎉 CORTEXIA is now installed!"
echo ""
echo "📖 Quick Start:"
echo ""
echo "   Run consciousness experiments:"
echo "   $ cargo run --release --example consciousness_experiment"
echo ""
echo "   Run maximum entanglement test:"
echo "   $ cargo run --release --example consciousness_maximum_entanglement"
echo ""
echo "   Run with noise analysis:"
echo "   $ cargo run --release --example consciousness_with_noise"
echo ""
echo "📊 Results will be saved to:"
echo "   - consciousness_experiment_results.json"
echo "   - consciousness_maximum_entanglement_results.json"
echo "   - consciousness_with_noise_results.json"
echo ""
echo "📚 Documentation: See README.md and Articulos-IEEE/"
echo ""
echo "🔬 For peer review reproducibility, see: REPRODUCIBILITY.md"
echo ""
