#!/usr/bin/env bash
set -euo pipefail

cargo test -p terminal-core
cargo test -p terminal-core --test fixtures
cargo test -p terminal-core --test compatibility
cargo test -p terminal-core --test tui_replay
scripts/check-compatibility-docs.sh
