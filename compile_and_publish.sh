#!/bin/bash
# Script de compilación y publicación de CORTEXIA
# Ejecutar con: bash compile_and_publish.sh

set -e  # Salir si hay error

echo "🚀 CORTEXIA - Script de Compilación y Publicación"
echo "=================================================="
echo ""

# Colores
GREEN='\033[0;32m'
RED='\033[0;31m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Función para imprimir con color
print_success() {
    echo -e "${GREEN}✅ $1${NC}"
}

print_error() {
    echo -e "${RED}❌ $1${NC}"
}

print_info() {
    echo -e "${YELLOW}ℹ️  $1${NC}"
}

# Verificar que estamos en el directorio correcto
if [ ! -f "Cargo.toml" ]; then
    print_error "Debes ejecutar este script desde cortexia-workspace/"
    exit 1
fi

print_info "Directorio de trabajo: $(pwd)"
echo ""

# ============================================
# FASE 1: COMPILACIÓN LOCAL
# ============================================

echo "📦 FASE 1: Compilando todas las librerías localmente..."
echo ""

CRATES=("hodgkin-huxley" "iit" "tda" "synapse-models")

for crate in "${CRATES[@]}"; do
    echo "Compilando $crate..."
    if cd "$crate" && cargo build --release; then
        print_success "$crate compilado correctamente"
        cd ..
    else
        print_error "$crate falló al compilar"
        exit 1
    fi
done

echo ""
print_success "Todas las librerías base compiladas exitosamente"
echo ""

# ============================================
# FASE 2: DRY-RUN DE PUBLICACIÓN
# ============================================

echo "🔍 FASE 2: Verificando paquetes para publicación (dry-run)..."
echo ""

for crate in "${CRATES[@]}"; do
    echo "Verificando $crate..."
    if cd "$crate" && cargo publish --dry-run; then
        print_success "$crate está listo para publicación"
        cd ..
    else
        print_error "$crate falló dry-run"
        exit 1
    fi
done

echo ""
print_success "Todas las verificaciones pasaron"
echo ""

# ============================================
# FASE 3: PUBLICACIÓN (INTERACTIVA)
# ============================================

echo "📤 FASE 3: Publicación a crates.io"
echo ""
read -p "¿Quieres publicar ahora? (y/n) " -n 1 -r
echo
if [[ ! $REPLY =~ ^[Yy]$ ]]; then
    print_info "Publicación cancelada. Puedes publicar manualmente más tarde."
    exit 0
fi

echo ""
print_info "Publicando librerías base..."
echo ""

for crate in "${CRATES[@]}"; do
    echo "Publicando $crate..."
    if cd "$crate" && cargo publish; then
        print_success "$crate publicado en crates.io"
        cd ..
        echo "Esperando 30 segundos para indexación..."
        sleep 30
    else
        print_error "$crate falló al publicar"
        print_info "Verifica el error arriba y vuelve a intentar"
        exit 1
    fi
done

echo ""
print_success "¡Librerías base publicadas exitosamente!"
echo ""

# ============================================
# FASE 4: INSTRUCCIONES PARA NEURAL-DYNAMICS
# ============================================

echo "⚠️  SIGUIENTE PASO: Actualizar neural-dynamics"
echo ""
echo "Debes editar manualmente:"
echo "  neural-dynamics/Cargo.toml"
echo ""
echo "Cambia estas líneas:"
echo "  hodgkin-huxley = { path = \"../hodgkin-huxley\" }"
echo "  synapse-models = { path = \"../synapse-models\" }"
echo ""
echo "Por:"
echo "  hodgkin-huxley = \"0.1.0\""
echo "  synapse-models = \"0.1.0\""
echo ""
read -p "¿Ya hiciste los cambios? (y/n) " -n 1 -r
echo

if [[ $REPLY =~ ^[Yy]$ ]]; then
    echo ""
    echo "Compilando neural-dynamics..."
    if cd neural-dynamics && cargo build --release; then
        print_success "neural-dynamics compilado"

        echo "Publicando neural-dynamics..."
        if cargo publish; then
            print_success "neural-dynamics publicado"
            cd ..
            sleep 30
        else
            print_error "neural-dynamics falló al publicar"
            exit 1
        fi
    else
        print_error "neural-dynamics falló al compilar"
        exit 1
    fi
fi

# ============================================
# FASE 5: INSTRUCCIONES PARA CORTEXIA
# ============================================

echo ""
echo "⚠️  ÚLTIMO PASO: Actualizar cortexia (meta-crate)"
echo ""
echo "Debes editar manualmente:"
echo "  cortexia/Cargo.toml"
echo ""
echo "Cambia todas las dependencias path = \"...\" a versiones:"
echo "  hodgkin-huxley = \"0.1.0\""
echo "  iit = \"0.1.0\""
echo "  tda = \"0.1.0\""
echo "  synapse-models = \"0.1.0\""
echo "  neural-dynamics = \"0.1.0\""
echo ""
read -p "¿Ya hiciste los cambios? (y/n) " -n 1 -r
echo

if [[ $REPLY =~ ^[Yy]$ ]]; then
    echo ""
    echo "Compilando cortexia..."
    if cd cortexia && cargo build --release; then
        print_success "cortexia compilado"

        echo "Publicando cortexia..."
        if cargo publish; then
            print_success "cortexia publicado"
            cd ..
        else
            print_error "cortexia falló al publicar"
            exit 1
        fi
    else
        print_error "cortexia falló al compilar"
        exit 1
    fi
fi

# ============================================
# FIN
# ============================================

echo ""
echo "🎉 ¡PUBLICACIÓN COMPLETADA!"
echo ""
echo "Tus crates están disponibles en:"
echo "  https://crates.io/crates/hodgkin-huxley"
echo "  https://crates.io/crates/iit"
echo "  https://crates.io/crates/tda"
echo "  https://crates.io/crates/synapse-models"
echo "  https://crates.io/crates/neural-dynamics"
echo "  https://crates.io/crates/cortexia"
echo ""
echo "Los usuarios pueden instalar con:"
echo "  cargo add cortexia"
echo ""
print_success "¡Todo listo!"
