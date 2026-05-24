#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"

cargo llvm-cov \
  -p animato-core \
  -p animato-tween \
  -p animato-timeline \
  -p animato-spring \
  -p animato-path \
  -p animato-physics \
  -p animato-color \
  -p animato-driver \
  -p animato-gpu \
  --all-features \
  --fail-under-lines 90
