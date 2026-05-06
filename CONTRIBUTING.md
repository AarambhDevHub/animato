# Contributing to Motus

Thank you for taking the time to contribute. Every bug report, feature suggestion, documentation improvement, and pull request makes Motus better for everyone.

---

## Table of Contents

1. [Code of Conduct](#code-of-conduct)
2. [Ways to Contribute](#ways-to-contribute)
3. [Setting Up the Workspace](#setting-up-the-workspace)
4. [Project Structure](#project-structure)
5. [Making a Change](#making-a-change)
6. [Commit Messages](#commit-messages)
7. [Testing Requirements](#testing-requirements)
8. [Documentation Requirements](#documentation-requirements)
9. [Pull Request Process](#pull-request-process)
10. [Reporting Bugs](#reporting-bugs)
11. [Suggesting Features](#suggesting-features)
12. [Crate Versioning](#crate-versioning)

---

## Code of Conduct

This project follows a simple rule: be respectful. Constructive criticism of code and ideas is welcome; personal attacks are not. Contributors who engage in hostile behavior will be asked to stop and may be removed from the project.

---

## Ways to Contribute

You do not need to write code to contribute:

- **Report a bug** — open an issue with a minimal reproduction
- **Suggest a feature** — open an issue describing the use case, not just the API
- **Improve documentation** — fix typos, add examples, clarify confusing sections
- **Write an example** — show Motus being used in a real scenario
- **Write a benchmark** — help identify performance regressions
- **Review pull requests** — read others' changes and leave thoughtful feedback
- **Write tests** — increase coverage for existing code

---

## Setting Up the Workspace

### Prerequisites

- Rust stable, 1.85 or later
- `wasm-pack` (optional — only needed for WASM-related work)
- `cargo-llvm-cov` (optional — only needed for coverage reports)

Install Rust via [rustup](https://rustup.rs/):

```sh
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### Clone and build

```sh
git clone https://github.com/AarambhDevHub/motus.git
cd motus

# Build all crates:
cargo build --workspace

# Run all tests:
cargo test --workspace

# Run tests with all features:
cargo test --workspace --all-features

# Verify no_std compatibility:
cargo test --workspace --no-default-features

# Check linting:
cargo clippy --workspace --all-features -- -D warnings

# Check formatting:
cargo fmt --check
```

### IDE Setup

This is a standard Cargo workspace. Any IDE with `rust-analyzer` support (VS Code, IntelliJ, Neovim) will work out of the box. Open the root `motus/` folder — `rust-analyzer` will detect the workspace automatically.

---

## Project Structure

```
motus/
├── crates/
│   ├── motus-core/        ← traits + easing — start here if unsure
│   ├── motus-tween/       ← Tween<T>, KeyframeTrack<T>
│   ├── motus-timeline/    ← Timeline, Sequence, stagger
│   ├── motus-spring/      ← Spring physics
│   ├── motus-path/        ← Bezier, SVG parser, morph
│   ├── motus-physics/     ← Inertia, Drag, Gesture
│   ├── motus-color/       ← perceptual color interpolation
│   ├── motus-driver/      ← AnimationDriver, Clocks, Scroll
│   ├── motus-gpu/         ← GPU batch compute
│   ├── motus-bevy/        ← Bevy plugin
│   ├── motus-wasm/        ← WASM + DOM integrations
│   └── motus/             ← facade crate (re-exports everything)
├── examples/              ← runnable examples
├── benches/               ← criterion benchmarks
└── tests/                 ← workspace-level integration tests
```

Each crate is self-contained. If you are working on spring physics, you should only need to open `crates/motus-spring/`. You should not need to understand `motus-gpu` to fix a spring bug.

---

## Making a Change

### 1. Check for an existing issue

Search [open issues](https://github.com/AarambhDevHub/motus/issues) before starting work. If there is no issue for your change, open one first — especially for anything larger than a typo fix. This prevents duplicate work and gives maintainers a chance to give early feedback on direction.

### 2. Fork and branch

```sh
# Fork on GitHub, then:
git clone https://github.com/YOUR_USERNAME/motus.git
cd motus
git checkout -b fix/spring-settle-detection
```

Branch naming conventions:

| Change type | Prefix | Example |
|-------------|--------|---------|
| Bug fix | `fix/` | `fix/tween-delay-off-by-one` |
| New feature | `feat/` | `feat/keyframe-reverse` |
| Documentation | `docs/` | `docs/easing-guide` |
| Refactor | `refactor/` | `refactor/timeline-entry-storage` |
| Performance | `perf/` | `perf/easing-bench-baseline` |
| Tests | `test/` | `test/spring-rk4-coverage` |

### 3. Make the smallest possible change

Do not mix unrelated changes in one PR. A PR that fixes a bug in `Spring` should not also add a new easing variant. Keep the diff focused — it makes review faster and easier to revert if something goes wrong.

### 4. Keep changes backward compatible

Until `v1.0.0`, the API is not frozen, but unnecessary breakage should still be avoided. If you need to change a public API signature, open an issue first to discuss it.

### 5. Format your code

```sh
cargo fmt
```

The CI rejects unformatted code. Run `cargo fmt` before every commit.

---

## Commit Messages

Use the [Conventional Commits](https://www.conventionalcommits.org/) format:

```
<type>(<scope>): <short description>

[optional body]

[optional footer]
```

**Type** must be one of:

| Type | When to use |
|------|-------------|
| `feat` | New feature or behavior |
| `fix` | Bug fix |
| `docs` | Documentation only |
| `test` | Adding or fixing tests |
| `perf` | Performance improvement without behavior change |
| `refactor` | Code restructuring without behavior change |
| `chore` | Build system, CI, dependency updates |
| `ci` | CI configuration changes |

**Scope** is the affected crate (without the `motus-` prefix):

```
feat(spring): add snappy() preset to SpringConfig
fix(tween): clamp elapsed to duration on large dt
docs(core): add examples to Interpolate trait docs
test(timeline): add seek-after-complete test case
perf(easing): inline all bounce variants
```

**Rules:**
- Use the imperative mood in the description: "add", "fix", "update" — not "added", "fixed", "updated"
- Keep the first line under 72 characters
- Reference the issue number in the footer: `Closes #42`
- Breaking changes must include `BREAKING CHANGE:` in the footer

---

## Testing Requirements

Every pull request must include tests. There are no exceptions.

### What to test

- **New behavior:** Write a test that fails before your change and passes after.
- **Bug fixes:** Write a test that reproduces the bug, then fix it.
- **Edge cases:** Boundary values, zero-duration tweens, empty collections, `dt = 0.0`, very large `dt`.

### Where to put tests

- Small unit tests go in a `#[cfg(test)]` block at the bottom of the relevant `src/*.rs` file.
- Integration tests that span multiple modules go in `tests/` at the workspace root.
- New examples go in `examples/`.

### Running tests

```sh
# Unit + integration tests for all crates:
cargo test --workspace

# A specific crate only:
cargo test -p motus-spring

# A specific test by name:
cargo test -p motus-spring spring_settles_with_wobbly_preset

# All features (required before opening a PR):
cargo test --workspace --all-features

# no_std compile check:
cargo test --workspace --no-default-features
```

### Clippy

The CI runs:

```sh
cargo clippy --workspace --all-features -- -D warnings
```

This means any clippy warning is a CI failure. Fix all warnings before pushing.

---

## Documentation Requirements

- Every `pub` item (struct, enum, trait, function, method) must have a `///` doc comment.
- Doc comments must include at least a one-sentence description.
- Non-trivial APIs must include a `# Examples` section with a runnable code block.
- Doc examples are compiled and run by `cargo test --doc` — they must work.

```rust
/// Advances the animation by `dt` seconds.
///
/// Returns `true` while the animation is still running,
/// and `false` once it is complete.
///
/// # Examples
///
/// ```rust
/// use motus::{Tween, Easing, Update};
///
/// let mut tween = Tween::new(0.0_f32, 1.0)
///     .duration(1.0)
///     .easing(Easing::Linear)
///     .build();
///
/// assert!(!tween.is_complete());
/// tween.update(1.0);
/// assert!(tween.is_complete());
/// ```
pub fn update(&mut self, dt: f32) -> bool { ... }
```

Check that your docs render correctly:

```sh
cargo doc --workspace --all-features --open
```

---

## Pull Request Process

### Before opening

Run this checklist locally:

```sh
cargo fmt --check
cargo clippy --workspace --all-features -- -D warnings
cargo test --workspace --all-features
cargo test --workspace --no-default-features
cargo doc --workspace --all-features
```

All four must pass cleanly — zero warnings, zero failures.

### Opening the PR

- Fill in the pull request template fully.
- Link the related issue: "Closes #42" or "Related to #42".
- Keep the title in Conventional Commits format: `fix(spring): clamp settle epsilon to positive values`.
- If the PR is a work in progress, open it as a Draft.

### Review process

- At least one maintainer approval is required before merging.
- Address all review comments — if you disagree with feedback, explain your reasoning in the thread. Do not silently ignore it.
- Keep the PR up to date with `main` by rebasing, not merging.
- Once approved, the maintainer will squash-merge the PR.

### After merging

Your change will appear in the next release. If it is a fix or small feature, it will go in the next patch or minor release. Large features wait for the next planned milestone.

---

## Reporting Bugs

Open an issue using the **Bug Report** template. Include:

1. **What you expected to happen.**
2. **What actually happened** — include the full error message or unexpected output.
3. **A minimal reproduction** — the smallest possible code that demonstrates the bug. Remove everything unrelated.
4. **Environment** — Rust version (`rustc --version`), OS, Motus version, active feature flags.

A minimal reproduction is the single most important thing you can provide. Issues without one may be closed if the bug cannot be reproduced.

---

## Suggesting Features

Open an issue using the **Feature Request** template. Include:

1. **The use case** — what are you trying to accomplish? Why does the current API not solve it?
2. **Proposed API** — what would you want to write? Show code.
3. **Alternatives considered** — what workarounds exist today and why are they insufficient?

Feature requests that describe only the desired API without explaining the use case will be asked for more context before being accepted.

---

## Crate Versioning

Motus follows [Semantic Versioning](https://semver.org/).

- **Patch** (`0.1.x`) — bug fixes only, no API changes.
- **Minor** (`0.x.0`) — new features, backward-compatible API additions, new crates.
- **Major** (`x.0.0`) — breaking API changes. Will not happen before `v1.0.0`.

Until `v1.0.0`, minor versions may contain small breaking changes if unavoidable. These will always be documented clearly in `CHANGELOG.md`.

Each sub-crate (`motus-core`, `motus-tween`, etc.) is versioned independently. The facade crate (`motus`) tracks the highest version among all sub-crates.

---

## Questions?

If you are unsure about anything — whether a bug is worth reporting, whether a feature fits the project, or how to approach a change — open an issue and ask. There are no stupid questions.

You can also join the discussion on the [Aarambh Dev Hub Discord](https://discord.gg/aarambhdevhub) — look for the `#motus` channel.
