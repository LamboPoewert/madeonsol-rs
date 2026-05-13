#!/usr/bin/env bash
# publish-rust.sh — gate cargo publish on git state.
#
# Run from the crate root (where Cargo.toml lives):
#   bash scripts/publish-rust.sh
#
# Aborts unless preflight-publish.sh passes (clean + tagged + pushed).

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
bash "$SCRIPT_DIR/preflight-publish.sh"

cargo publish "$@"
