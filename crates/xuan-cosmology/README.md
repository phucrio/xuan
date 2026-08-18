# xuan-cosmology

Deterministic Rust primitives for concepts shared by traditional calendrical and correlative-cosmology systems.

## Public API

- `YinYang`
- `WuXing` and `WuxingRelation`
- `TianGan` and the 10-stem cycle
- `DiZhi` and the 12-branch cycle
- `GanZhi` and the sexagenary cycle
- reusable traits for labels, element/yin-yang properties, keys, and cyclic indexing

Vietnamese and Chinese labels are provided by the `Labeled` trait.

## Design

The crate is pure and deterministic. It performs no filesystem, network, clock, environment, or random I/O and is the lowest-level crate in the `xuan` workspace.

## License

Licensed under either Apache-2.0 or MIT, at your option.
