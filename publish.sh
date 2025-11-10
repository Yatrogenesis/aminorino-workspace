#!/bin/bash
# CORTEXIA Publication Script
# Publishes all libraries to crates.io in correct dependency order

set -e

echo "🚀 CORTEXIA Publication to crates.io"
echo "===================================="
echo ""

cd /Users/yatrogenesis/cortexia-workspace

# Round 1: Base libraries (no internal dependencies)
echo "📦 Round 1: Publishing base libraries..."
echo ""

echo "1️⃣  Publishing hodgkin-huxley..."
cd hodgkin-huxley && ~/.cargo/bin/cargo publish
echo "✅ hodgkin-huxley published!"
echo ""
sleep 15

echo "2️⃣  Publishing iit..."
cd ../iit && ~/.cargo/bin/cargo publish
echo "✅ iit published!"
echo ""
sleep 15

echo "3️⃣  Publishing tda..."
cd ../tda && ~/.cargo/bin/cargo publish
echo "✅ tda published!"
echo ""
sleep 15

echo "4️⃣  Publishing synapse-models..."
cd ../synapse-models && ~/.cargo/bin/cargo publish
echo "✅ synapse-models published!"
echo ""
sleep 15

echo ""
echo "⚠️  NEXT STEPS:"
echo "   1. Update neural-dynamics/Cargo.toml"
echo "   2. Change hodgkin-huxley and synapse-models from path to version = \"0.1.0\""
echo "   3. Run: cd neural-dynamics && cargo publish"
echo ""
echo "   4. Update cortexia/Cargo.toml"
echo "   5. Change all dependencies from path to version = \"0.1.0\""
echo "   6. Run: cd cortexia && cargo publish"
echo ""
echo "✅ Base libraries published successfully!"
