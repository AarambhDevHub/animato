import {
  copyFileSync,
  existsSync,
  mkdirSync,
  readFileSync,
  readdirSync,
  rmSync,
  statSync,
} from "node:fs";
import { join, resolve } from "node:path";

const example = process.argv[2] ? resolve(process.argv[2]) : process.cwd();
const root = resolve(new URL("..", import.meta.url).pathname);
const pkg = resolve(root, "crates/animato-js/pkg/package.json");

if (!existsSync(pkg)) {
  throw new Error("Build @animato/core first with scripts/build-js-package.sh");
}

const src = join(example, "src");
if (!existsSync(src)) {
  throw new Error(`Missing src directory in ${example}`);
}

const files = collect(src);
if (!files.some((file) => /@animato\/core/.test(read(file)))) {
  throw new Error(`Example ${example} does not import @animato/core`);
}

const dist = join(example, "dist");
rmSync(dist, { recursive: true, force: true });
mkdirSync(dist, { recursive: true });

for (const file of files) {
  const relative = file.slice(src.length + 1);
  const target = join(dist, relative);
  mkdirSync(resolve(target, ".."), { recursive: true });
  copyFileSync(file, target);
}

console.log(`Verified ${example}`);

function collect(dir) {
  const out = [];
  for (const entry of readdirSync(dir)) {
    const path = join(dir, entry);
    if (statSync(path).isDirectory()) out.push(...collect(path));
    else out.push(path);
  }
  return out;
}

function read(path) {
  return readFileSync(path, "utf8");
}
