#!/usr/bin/env bash
set -xe

./bootstrap/raptor -O3 ./src/raptor.rp
diff -q ./build/raptor.ll ./bootstrap/raptor.ll
