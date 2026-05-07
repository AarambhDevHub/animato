---
name: Bug Report
about: Something is broken or behaving unexpectedly
title: "fix: "
labels: bug
assignees: ""
---

## Description

A clear, one-paragraph description of the bug.

## Minimal Reproduction

```rust
// The smallest possible code that demonstrates the bug.
// Remove everything unrelated.
// This must compile (or explain why it doesn't).
use animato::{Tween, Easing, Update};

fn main() {
    // reproduce the bug here
}
```

## Expected Behavior

What you expected to happen.

## Actual Behavior

What actually happened. Include the full error message, panic output, or unexpected value.

## Environment

| | |
|---|---|
| Animato version | `0.x.x` |
| Rust version | `rustc --version` output |
| OS | e.g. Pop OS 22.04 / macOS 14 / Windows 11 |
| Active features | e.g. `default`, `bevy`, `wasm` |

## Additional Context

Anything else that might be relevant — related issues, workarounds you've tried, etc.
