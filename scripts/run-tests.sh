#!/usr/bin/env bash
set -xe

python3 ./tests/test_runner.py run --compiler ./bootstrap/raptor --memory arc
