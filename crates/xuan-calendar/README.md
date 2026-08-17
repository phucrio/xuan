# xuan-calendar

Deterministic Rust calendar calculations for Vietnamese and Chinese lunisolar calendars, Gregorian and Julian dates, and GanZhi date-time conversion.

## Public API

- civil date/time types and explicit fixed-offset time zones
- `TimeZone::VN`, `TimeZone::CN`, and `TimeZone::UTC`
- Gregorian ↔ Julian Day helpers
- Gregorian ↔ Vietnamese/Chinese lunisolar conversion
- solar-term indexing
- GanZhi year/month/day/hour calculation
- configurable day-boundary policy, including the 23:00 Zi-hour convention

## Design

The crate is pure and deterministic. It does not read the system clock, environment, filesystem, or network. Calendar behavior is driven entirely by explicit input values and time-zone offsets.

`xuan-calendar` depends on `xuan-cosmology` for Heavenly Stems, Earthly Branches, and GanZhi primitives.

## License

Licensed under either Apache-2.0 or MIT, at your option.
