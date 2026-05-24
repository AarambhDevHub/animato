import { readFileSync, writeFileSync } from "node:fs";
import { resolve } from "node:path";

const root = resolve(new URL("..", import.meta.url).pathname);
const cargoToml = readFileSync(resolve(root, "Cargo.toml"), "utf8");
const version = cargoToml.match(/^version\s*=\s*"([^"]+)"/m)?.[1];

if (!version) {
  throw new Error("Unable to read workspace version from Cargo.toml");
}

const pkgPath = resolve(root, "crates/animato-js/pkg/package.json");
const pkg = JSON.parse(readFileSync(pkgPath, "utf8"));

Object.assign(pkg, {
  name: "@aarambhdevhub/animato-core",
  version,
  description: "WASM-powered animation engine for JavaScript, React, Svelte, Vue, Angular, and vanilla apps.",
  license: "MIT OR Apache-2.0",
  repository: {
    type: "git",
    url: "git+https://github.com/AarambhDevHub/animato.git",
    directory: "crates/animato-js",
  },
  keywords: ["animation", "wasm", "javascript", "tween", "spring", "motion"],
  sideEffects: false,
  files: [
    "animato_js_bg.wasm",
    "animato_js.js",
    "animato_js.d.ts",
    "package.json",
    "README.md",
  ],
  exports: {
    ".": {
      types: "./animato_js.d.ts",
      import: "./animato_js.js",
    },
  },
  module: "./animato_js.js",
  types: "./animato_js.d.ts",
});

delete pkg.author;

writeFileSync(pkgPath, `${JSON.stringify(pkg, null, 2)}\n`);
