# Migration To v1.0.0

Animato v1.0.0 stabilizes the v0.9.0 API. No breaking public API changes are
required for the upgrade.

## Cargo Update

```toml
[dependencies]
animato = "1.0"
```

Focused crates:

```toml
animato-core = "1.0"
animato-tween = "1.0"
animato-spring = "1.0"
```

## What Changed

- Workspace crates are versioned as `1.0.0`.
- Public APIs are documented as stable.
- Documentation has moved into a larger `docs/` tree.
- CI and release checks include coverage and fuzzing.
- Publish workflow validates the tag and performs dry-runs before publishing.

## From v0.9.0

No code migration should be required. Replace `0.9` with `1.0` in manifests and
run your test suite.

## From Older Versions

Read [CHANGELOG.md](../CHANGELOG.md) for each release since your current
version. The biggest additions before v1.0 were paths, physics, color, Bevy,
WASM, scroll-linked animation, morphing, and GPU batches.

## Related Docs

- [API Full](./api-full.md)
- [Feature Flags](./feature-flags.md)
- [Troubleshooting](./troubleshooting.md)
