# FAQ

## Does Animato render anything?

No. Animato computes values. Your application renders them.

## Is v1.0.0 stable?

Yes. The current public API is treated as stable. Breaking changes require a
future major version.

## Can I use Animato in no_std?

Yes, through focused crates such as `animato-core`, `animato-tween`, and
`animato-spring` with `default-features = false`. See [no-std.md](./no-std.md).

## Why use the facade crate?

The facade gives one dependency and stable re-exports. Use focused crates when
you need tighter dependency control.

## Does GPU batching require a GPU?

No. `GpuAnimationBatch::new_auto()` falls back to CPU when GPU setup is not
available. `new_cpu()` is deterministic and useful for tests.

## Which easing variants exist?

Animato ships 38 named variants: classic easings, CSS-compatible bezier/steps,
and advanced variants like rough ease, slow-mo, wiggle, custom bounce, and expo
scale. See [API Full](./api-full.md).

## Where are examples?

See [examples.md](./examples.md) and the repository `examples/` directory.

## Related Docs

- [Getting Started](./getting-started.md)
- [Troubleshooting](./troubleshooting.md)
