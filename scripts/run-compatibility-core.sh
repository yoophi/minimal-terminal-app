#!/usr/bin/env bash
set -euo pipefail

cargo test -p terminal-core
cargo test -p terminal-core --test fixtures
cargo test -p terminal-core --test compatibility

