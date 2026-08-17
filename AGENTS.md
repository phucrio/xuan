# AGENTS.md

## Scope

This repository contains reusable public Rust crates. Keep the workspace deterministic, side-effect free at the core, and independent from downstream applications.

## Architecture

- `xuan-cosmology` is the lowest-level domain primitive crate.
- `xuan-calendar` may depend on `xuan-cosmology`.
- `xuan-cosmology` must not depend on `xuan-calendar`.
- Do not add Zi Wei-specific chart/rule logic, I Ching trading logic, UI code, network access, filesystem access, hidden system-time dependencies, or other product-specific behavior to the shared foundation.

## Quality gate

Before committing, run:

```bash
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace --all-features
```

Do not commit with unresolved Clippy warnings or errors. If a finding remains, review its root cause and fix the implementation rather than suppressing it without justification.
