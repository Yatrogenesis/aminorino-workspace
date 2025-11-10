#!/bin/bash
# Quick publish script for CORTEXIA
set -e

echo "🚀 Compilando y publicando CORTEXIA..."
echo ""

cd /Users/yatrogenesis/cortexia-workspace

# Compilar y publicar las 4 librerías base
echo "1️⃣  Publishing hodgkin-huxley..."
cd hodgkin-huxley && cargo publish && cd ..
sleep 10

echo "2️⃣  Publishing iit..."
cd iit && cargo publish && cd ..
sleep 10

echo "3️⃣  Publishing tda..."
cd tda && cargo publish && cd ..
sleep 10

echo "4️⃣  Publishing synapse-models..."
cd synapse-models && cargo publish && cd ..
sleep 10

echo ""
echo "✅ Librerías base publicadas!"
echo ""
echo "⚠️  SIGUIENTE: Edita neural-dynamics/Cargo.toml"
echo "   Cambia las líneas 14-15 de path a version 0.1.0"
echo "   Luego ejecuta:"
echo "   cd neural-dynamics && cargo publish && cd .."
echo ""
echo "⚠️  FINAL: Edita cortexia/Cargo.toml"
echo "   Cambia todas las dependencias a version 0.1.0"
echo "   Luego ejecuta:"
echo "   cd cortexia && cargo publish && cd .."
