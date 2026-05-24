#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"

wasm-pack build crates/animato-js --target web --scope aarambhdevhub --release
node scripts/prepare-js-package.mjs
node scripts/check-js-size.mjs
