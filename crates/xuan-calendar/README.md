# xuan-calendar

Deterministic Rust calendar calculations for Vietnamese and Chinese lunisolar calendars,
Gregorian civil dates, Julian Day arithmetic, solar terms, and GanZhi date-time conversion.

## Features

- civil date/time types with explicit fixed UTC offsets;
- `TimeZone::VN`, `TimeZone::CN`, and `TimeZone::UTC`;
- proleptic Gregorian ↔ Julian Day / Julian Day Number helpers;
- Gregorian ↔ Vietnamese/Chinese lunisolar conversion;
- principal-term and solar-term calculations;
- GanZhi year, month, day, and hour calculation;
- configurable day-boundary policy, including the 23:00 Zi-hour convention;
- deterministic behavior with no dependency on the system clock, environment, filesystem,
  network, locale, or host time-zone database.

`xuan-calendar` depends on `xuan-cosmology` for Heavenly Stems, Earthly Branches, and
GanZhi primitives.

## Quick start

```rust
use xuan_calendar::{gregorian_to_lunar, CivilDate, TimeZone};

let lunar = gregorian_to_lunar(CivilDate::new(2026, 2, 17), TimeZone::VN)
    .expect("valid calendar date");

assert_eq!(lunar.year, 2026);
assert_eq!(lunar.month, 1);
assert_eq!(lunar.day, 1);
assert!(!lunar.is_leap_month);
```

GanZhi calculations are exposed through `CivilDateTime` and `ToGanZhi`:

```rust
use xuan_calendar::{CivilDate, CivilDateTime, CivilTime, TimeZone, ToGanZhi};

let dt = CivilDateTime::new(
    CivilDate::new(2026, 2, 17),
    CivilTime::new(12, 0),
    TimeZone::VN,
);

let ganzhi = dt.to_ganzhi();
println!("{:?}", ganzhi);
```

## Algorithm documentation

The lunisolar model, reimplemented components, differences from common reference
approaches, and accuracy notes are documented in two maintained versions:

- **English (default):** [`docs/lunisolar-calendar.md`](docs/lunisolar-calendar.md)
- **Tiếng Việt:** [`docs/lunisolar-calendar.vi.md`](docs/lunisolar-calendar.vi.md)

## Implementation model

The lunisolar layer uses astronomical new-moon and apparent-solar-longitude approximations.
A new moon is assigned to a local civil day using an explicit fixed UTC offset. Lunar
month intervals are then evaluated for principal-term crossings to determine month
numbering and leap months.

The civil calendar is **proleptic Gregorian**. `Julian` in the API refers to Julian Day /
Julian Day Number arithmetic, not automatic switching to the historical Julian calendar.

## Accuracy

The astronomical layer uses compact approximations rather than a high-precision ephemeris.
The regression suite includes selected cases from 1800 through 2620, but that span is test
coverage rather than a guarantee for every date in the interval. Boundary-sensitive,
historical, or far-future results should be cross-checked against an appropriate ephemeris
or trusted calendar oracle when high confidence is required.

See [the algorithm documentation](docs/lunisolar-calendar.md#accuracy-and-validation) for
details.

## Development

From the workspace root:

```bash
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace --all-features
```

## License

Licensed under either Apache-2.0 or MIT, at your option.
