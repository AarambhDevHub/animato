import { readFileSync } from "node:fs";
import { gzipSync } from "node:zlib";
import { resolve } from "node:path";

const root = resolve(new URL("..", import.meta.url).pathname);
const wasmPath = resolve(root, "crates/animato-js/pkg/animato_js_bg.wasm");
const bytes = readFileSync(wasmPath);
const gzipBytes = gzipSync(bytes, { level: 9 });
const limit = Number.parseInt(process.env.ANIMATO_JS_GZIP_LIMIT ?? "122880", 10);

console.log(`@aarambhdevhub/animato-core wasm: ${bytes.length} bytes raw, ${gzipBytes.length} bytes gzip`);

if (gzipBytes.length > limit) {
  throw new Error(`gzipped WASM size ${gzipBytes.length} exceeds budget ${limit}`);
}
