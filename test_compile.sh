#!/bin/bash
# Test compilation of all CORTEXIA libraries

cd /Users/yatrogenesis/cortexia-workspace

echo "=== Testing hodgkin-huxley ===" > compile_results.txt
~/.cargo/bin/cargo build -p hodgkin-huxley 2>&1 >> compile_results.txt
echo "Exit code: $?" >> compile_results.txt
echo "" >> compile_results.txt

echo "=== Testing iit ===" >> compile_results.txt
~/.cargo/bin/cargo build -p iit 2>&1 >> compile_results.txt
echo "Exit code: $?" >> compile_results.txt
echo "" >> compile_results.txt

echo "=== Testing tda ===" >> compile_results.txt
~/.cargo/bin/cargo build -p tda 2>&1 >> compile_results.txt
echo "Exit code: $?" >> compile_results.txt
echo "" >> compile_results.txt

echo "=== Testing synapse-models ===" >> compile_results.txt
~/.cargo/bin/cargo build -p synapse-models 2>&1 >> compile_results.txt
echo "Exit code: $?" >> compile_results.txt
echo "" >> compile_results.txt

cat compile_results.txt
