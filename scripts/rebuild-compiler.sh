#!/usr/bin/env bash
set -xe

# First recompilation: compile the updated compiler source with the current compiler
# This generates an updated LLVM IR file containing the new compiler implementation
# I think it can skip optimization so it can be faster
./bootstrap/raptor ./src/raptor.rp -v
cp ./build/raptor.ll ./bootstrap/raptor.ll
./bootstrap/bootstrap.sh

# Second recompilation: rebuild the compiler using the updated compiler
# This ensures the generated compiler is built with the latest compiler implementation
./bootstrap/raptor ./src/raptor.rp -v -O3
cp ./build/raptor.ll ./bootstrap/raptor.ll
./bootstrap/bootstrap.sh
