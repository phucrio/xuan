# Lunisolar calendar model and implementation

This document describes the lunisolar calendar model implemented by `xuan-calendar`,
including its astronomical approximations, local-time rules, and implementation choices.

Vietnamese version: [lunisolar-calendar.vi.md](lunisolar-calendar.vi.md)

## Scope

`xuan-calendar` provides deterministic calendar calculations from explicit inputs. The
lunisolar layer currently supports:

- proleptic Gregorian civil dates;
- Julian Day and Julian Day Number arithmetic;
- Vietnamese and Chinese lunisolar conversion with explicit fixed UTC offsets;
- `TimeZone::VN` (`UTC+07:00`) and `TimeZone::CN` (`UTC+08:00`), plus arbitrary fixed offsets;
- principal-term detection for leap-month assignment;
- round-trip Gregorian to lunisolar and lunisolar to Gregorian conversion.

The implementation models an astronomical lunisolar calendar. Historical calendar
reconstruction is outside its current scope.

## Calendar rules

The lunisolar model follows these rules:

1. A lunar month begins on the local civil day containing a new moon.
2. A common lunar year has 12 lunar months; a leap year has 13.
3. Lunar month 11 is anchored around the winter-solstice principal term.
4. When 13 lunar months occur between successive month-11 anchors, the first month
   without a principal term is treated as the leap month.
5. The local civil day of an astronomical event depends on the supplied UTC offset, so
   the same new moon can begin a month on different dates in different time zones.

The crate evaluates these rules from explicit values rather than host clock or locale
state.

## Implementation pipeline

### 1. Gregorian date and JDN

`gregorian_to_jdn` maps a `CivilDate` to an integer Julian Day Number using proleptic
Gregorian arithmetic. The inverse helper reconstructs a Gregorian date from a JDN.

The implementation uses proleptic Gregorian dates throughout instead of switching to the
Julian calendar around the historical Gregorian reform.

### 2. New-moon approximation

`new_moon_jd(k)` evaluates a compact astronomical series for lunation index `k`. The
series uses a mean new-moon epoch, solar anomaly, lunar anomaly, argument of latitude,
and periodic corrections commonly used in Meeus-style calendar calculations.

`new_moon_day_local` then applies the fixed UTC offset and maps the instant to the local
civil day containing that new moon.

### 3. Apparent solar longitude

`sun_longitude_rad` approximates apparent solar longitude from Julian Day. It evaluates
mean solar anomaly and longitude, applies the equation of center and a small apparent-
longitude correction, then normalizes the result to `[0, 2π)`.

Principal terms are represented by 30-degree solar-longitude boundaries.

### 4. Month-11 anchors

`lunar_month11_jdn` finds a new-moon start near the end of the Gregorian year and uses
the principal-term sector to select the month-11 anchor. Consecutive anchors define the
span used for month numbering and leap-month detection.

### 5. Principal terms across local month intervals

A lunar month is represented as a half-open local interval:

```text
[start_of_month, start_of_next_month)
```

`month_has_principal_term_local` converts those local boundaries to UTC Julian Day and
checks whether solar longitude crosses the next 30-degree principal-term boundary inside
the interval.

This interval-based check matters near time-zone and civil-day boundaries, where sampling
only one longitude value at the month start can be ambiguous.

### 6. Month counting and leap-month assignment

The implementation enumerates local new-moon starts between the two month-11 anchors.
If there are 13 lunar months, `leap_month_offset` scans each local month interval and
selects the first interval without a principal term.

Month numbers are assigned from month 11 forward, with the leap month repeating the
number of the preceding regular month.

### 7. Reverse conversion

`lunar_to_gregorian` uses a bounded verification strategy: it scans a finite JDN range
around the requested lunar year, converts each Gregorian candidate forward, and returns
the date whose computed `LunarDate` exactly matches the input.

This favors consistency with the forward algorithm over a separate direct inverse
formula and gives the regression suite a strong round-trip invariant.

## Reimplemented components

The crate implements the following pieces directly in Rust:

- proleptic Gregorian and JDN conversion;
- Gregorian date arithmetic through JDN;
- approximate new-moon evaluation;
- approximate apparent solar longitude;
- fixed-offset local-day assignment;
- month-11 anchoring;
- principal-term detection across local lunar-month intervals;
- local new-moon enumeration and leap-month assignment;
- reverse conversion by forward verification;
- crate-specific types, APIs, error handling, and regression tests.

Published calendrical rules and astronomical formulas are used as technical references
for these calculations. The implementation is organized around the crate's own data
model, local-month interval logic, and Rust APIs.

## Differences from common Hồ Ngọc Đức reference implementations

Hồ Ngọc Đức's calendar articles and programs are useful references for Vietnamese
lunisolar rules. `xuan-calendar` differs in several implementation details:

| Area | `xuan-calendar` | Common reference approach |
| --- | --- | --- |
| Civil calendar | Proleptic Gregorian throughout | Example code may switch between Julian and Gregorian calendars around 1582 |
| Time zone | Explicit fixed offset in minutes | Common examples use a time-zone value in hours |
| New moon | Compact approximation used directly by the crate | Some routines include an additional ΔT correction branch |
| Principal term | Detects a 30-degree crossing across the complete local month interval | Compact routines often compare solar-term sectors at new-moon starts |
| Month count | Enumerates local new-moon starts | Compact routines may infer counts from day-span arithmetic |
| Reverse conversion | Bounded search plus exact forward round-trip verification | Often calculated directly from month offsets and new-moon indices |
| Calendar data | Computed from formulas at runtime | Some implementations also provide precomputed tables for fixed ranges |
| Historical calendar | Outside current scope | Historical reconstructions may be provided separately |
| GanZhi support | Integrated elsewhere in this crate | Separate from basic lunisolar conversion in many references |

These are implementation choices rather than claims of higher astronomical precision.

## Accuracy and validation

The astronomical functions are compact approximations rather than a high-precision
planetary or lunar ephemeris. Boundary-sensitive cases can be affected by small timing
errors, especially when a new moon or principal term falls close to local midnight.

The regression suite includes selected Vietnamese and Chinese cases for ordinary months,
leap months, New Year dates, GanZhi behavior, and round trips. Current test vectors include
selected dates from 1800 through 2620. This describes regression coverage, not guaranteed
precision for every date in that interval.

For historical research, far-future dates, or applications that depend on exact boundary
timing, results should be cross-checked against an appropriate ephemeris or trusted
calendar oracle.

The astronomical layer can be replaced with a higher-precision implementation later
without changing the public local-month model.

## Related crate behavior

The lunisolar algorithm is one part of `xuan-calendar`. The crate also provides:

- Julian Day helpers;
- solar-term indexing;
- GanZhi year/month/day/hour calculations;
- explicit 23:00 Zi-hour versus midnight day-boundary policies.

## References

References used for background, terminology, validation, and comparison:

- Hồ Ngọc Đức, *Thuật toán tính âm lịch*: https://www.xemamlich.uhm.vn/calrules.html
- Hồ Ngọc Đức, *Computing the Vietnamese lunar calendar*: https://www.xemamlich.uhm.vn/calrules_en.html
- Jean Meeus, *Astronomical Algorithms*.
- Edward M. Reingold and Nachum Dershowitz, *Calendrical Calculations*.

For implementation details, see `src/lunar.rs`, `src/julian.rs`, `src/solar.rs`, and
`src/tests.rs`.
