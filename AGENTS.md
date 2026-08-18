# AGENTS.md

## Scope

This repository contains reusable public Rust crates. Keep the workspace deterministic, focused, and side-effect free at the core.

## Architecture

- `xuan-cosmology` is the lowest-level domain primitive crate.
- `xuan-calendar` may depend on `xuan-cosmology`.
- `xuan-cosmology` must not depend on `xuan-calendar`.
- Keep public APIs data-oriented and reusable.
- Avoid ambient I/O, hidden runtime state, randomness, or implicit system-time dependencies in core calculations.

## Release tags

- Each crate is versioned and released independently.
- Release tags must follow `<crate-name>-v<semver>`.
- Examples: `xuan-cosmology-v0.1.0`, `xuan-calendar-v0.1.0`.
- Do not create workspace-wide version tags such as `0.1.0` or `v0.1.0`.

## Quality gate

Before committing, run:

```bash
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace --all-features
```

Do not commit with unresolved Clippy warnings or errors. If a finding remains, review its root cause and fix the implementation rather than suppressing it without justification.
