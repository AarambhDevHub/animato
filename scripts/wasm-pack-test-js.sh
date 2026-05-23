#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"

chrome_bin="${CHROME:-}"
if [ -z "$chrome_bin" ]; then
  chrome_bin="$(command -v google-chrome || command -v google-chrome-stable || command -v chrome || true)"
fi

if [ -z "$chrome_bin" ]; then
  wasm-pack test --headless --chrome crates/animato-js
  exit $?
fi

chrome_version="$("$chrome_bin" --version | grep -Eo '[0-9]+\.[0-9]+\.[0-9]+\.[0-9]+' | head -n1)"

if [ -z "$chrome_version" ]; then
  wasm-pack test --headless --chrome crates/animato-js
  exit $?
fi

driver_root="${TMPDIR:-/tmp}/animato-chromedriver-${chrome_version}"
driver_path="${driver_root}/chromedriver-linux64/chromedriver"

if [ ! -x "$driver_path" ]; then
  mkdir -p "$driver_root"
  zip_path="${driver_root}/chromedriver.zip"
  curl -fsSL \
    "https://storage.googleapis.com/chrome-for-testing-public/${chrome_version}/linux64/chromedriver-linux64.zip" \
    -o "$zip_path"
  unzip -q -o "$zip_path" -d "$driver_root"
  chmod +x "$driver_path"
fi

wasm-pack test --headless --chrome --chromedriver "$driver_path" crates/animato-js
