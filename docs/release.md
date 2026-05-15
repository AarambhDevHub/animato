# Release Process

This checklist is for v1.0.0 and later stable releases.

## Preflight

1. Confirm `CHANGELOG.md` has the new version entry.
2. Confirm workspace version and internal dependency pins match the tag.
3. Run the full test plan in [testing.md](./testing.md).
4. Run `cargo publish --dry-run` for each crate immediately before publishing
   it. In a coordinated workspace release, downstream crates cannot verify
   against crates.io until their new internal dependencies have been published.
5. Capture benchmark baseline data from [benchmarks.md](./benchmarks.md).

## Publish Order

```text
animato-core
animato-color
animato-tween
animato-spring
animato-path
animato-physics
animato-driver
animato-timeline
animato-gpu
animato-wasm
animato-bevy
animato
```

## Tag

```sh
git tag v1.0.0
git push origin v1.0.0
```

The publish workflow validates that `v1.0.0` matches the workspace package
version `1.0.0`.

## GitHub Release

The release notes must include:

- crate table with versions,
- install snippet,
- docs links,
- benchmark summary,
- coverage and fuzz gate status,
- notable compatibility notes.

## Related Docs

- [Testing](./testing.md)
- [Benchmarks](./benchmarks.md)
- [Migration](./migration.md)
