# xuan

Reusable Rust building blocks for traditional calendrical and correlative-cosmology systems.

`xuan` provides small, deterministic crates with explicit domain boundaries and composable APIs.

## Crates

| Crate | Purpose |
| --- | --- |
| `xuan-cosmology` | Yin-Yang, Wu Xing, Heavenly Stems, Earthly Branches, GanZhi, cycles, and generic relationships |
| `xuan-calendar` | Civil/Gregorian dates, Julian Day, lunisolar conversion, time zones, and GanZhi date-time calculations |

Dependency direction is one-way:

```text
xuan-calendar -> xuan-cosmology
```

## Development

```bash
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace --all-features
```

## License

Licensed under either of the following, at your option:

- Apache License, Version 2.0 (`LICENSE-APACHE`)
- MIT License (`LICENSE-MIT`)
