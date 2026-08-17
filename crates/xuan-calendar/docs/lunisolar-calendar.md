# Lunisolar calendar model and implementation

This document describes the lunisolar calendar algorithm implemented by `xuan-calendar`.
It is original project documentation written from the behavior of this crate. It is not a
copy or translation of third-party calendar documentation or source code.

Vietnamese version: [lunisolar-calendar.vi.md](lunisolar-calendar.vi.md)

## Scope

`xuan-calendar` provides deterministic astronomical calendar calculations from explicit
inputs. The lunisolar layer currently supports:

- Gregorian civil dates using the proleptic Gregorian calendar;
- Julian Day and Julian Day Number arithmetic;
- Vietnamese and Chinese lunisolar conversion with explicit fixed UTC offsets;
- `TimeZone::VN` (`UTC+07:00`) and `TimeZone::CN` (`UTC+08:00`), plus arbitrary fixed offsets;
- principal-term detection for leap-month assignment;
- round-trip Gregorian to lunisolar and lunisolar to Gregorian conversion.

This is an astronomical/proleptic implementation. It is not a reconstruction of an
official historical calendar and should not be treated as a legal or historical-calendar
authority.

## Calendar rules modeled by the crate

The implementation follows the standard lunisolar structure used by modern Vietnamese
and Chinese calendars:

1. A lunar month begins on the local civil day containing a new moon.
2. A common lunar year has 12 lunar months; a leap year has 13.
3. Lunar month 11 is anchored around the winter-solstice principal term.
4. When there are 13 lunar months between successive month-11 anchors, the first month
   without a principal term is treated as the leap month.
5. The local civil day of an astronomical event depends on the supplied UTC offset, so
   the same new moon can begin a month on different civil dates in different time zones.

The crate models these rules using explicit inputs rather than the host system clock,
time-zone database, locale, or operating-system state.

## Implementation pipeline

### 1. Gregorian date to JDN

`gregorian_to_jdn` maps a `CivilDate` to an integer Julian Day Number using proleptic
Gregorian arithmetic. The inverse helper reconstructs a Gregorian date from a JDN.

The crate deliberately does not switch to the Julian calendar before the Gregorian
reform date. Historical dates are therefore interpreted consistently as proleptic
Gregorian dates.

### 2. Approximate new-moon instant

`new_moon_jd(k)` evaluates a compact astronomical series for lunation index `k`.
The series uses the mean new-moon epoch, solar anomaly, lunar anomaly, argument of
latitude, and periodic corrections commonly found in Meeus-style calendar algorithms.

The result is a Julian Day in UTC-like astronomical time used by this implementation.
`new_moon_day_local` then applies the explicit fixed UTC offset and converts the instant
to the local JDN containing that new moon.

### 3. Apparent solar longitude

`sun_longitude_rad` computes an approximate apparent solar longitude from Julian Day.
It evaluates the mean solar anomaly and longitude, applies the equation of center, then
a small apparent-longitude correction. The result is normalized to `[0, 2π)`.

Principal terms are represented by 30-degree solar-longitude boundaries.

### 4. Month-11 anchors

`lunar_month11_jdn` finds a new-moon start near the end of the Gregorian year and uses
its principal-term sector to select the month-11 anchor. Consecutive anchors define the
lunisolar year span used for month numbering and leap-month detection.

### 5. Principal-term detection over local month intervals

A lunar month is represented as a half-open local interval:

```text
[start_of_month, start_of_next_month)
```

`month_has_principal_term_local` converts the local boundaries to UTC Julian Day and
checks whether solar longitude crosses the next 30-degree principal-term boundary inside
that interval.

This is important because leap-month assignment is made against the local civil month
interval, not merely by sampling one longitude value at a nominal month start.

### 6. Month counting and leap-month assignment

The implementation enumerates local new-moon starts between the two month-11 anchors.
If more than 12 month starts occur, `leap_month_offset` scans each local month interval
and selects the first interval without a principal term.

Month numbers are then assigned from month 11 forward, with the leap month repeating the
number of the preceding regular month.

### 7. Reverse conversion

`lunar_to_gregorian` intentionally favors a simple verification strategy: it scans a
bounded JDN range around the requested lunar year, converts each candidate Gregorian
date forward, and returns the date whose computed `LunarDate` exactly matches the input.

This is less asymptotically efficient than a direct inverse formula, but it keeps reverse
conversion aligned with the forward algorithm and gives the regression suite a strong
round-trip invariant.

## Reimplementation and provenance boundary

The public Rust implementation is maintained as a reimplementation of published
calendrical rules and astronomical equations. The repository does **not** vendor or
redistribute third-party JavaScript, PHP, Java, or other calendar source files, and it
does not include precomputed third-party lunar-calendar tables.

The following parts are implemented directly in Rust in this crate:

- proleptic Gregorian/JDN conversion and date arithmetic;
- approximate new-moon evaluation;
- approximate apparent solar longitude;
- fixed-offset local-day conversion;
- month-11 anchoring;
- principal-term detection across local lunar-month intervals;
- local new-moon enumeration and leap-month assignment;
- reverse conversion by forward verification;
- crate-specific types, APIs, error model, and regression tests.

Published mathematical rules, astronomical equations, and numeric coefficients are used
as technical references for the method being implemented. Third-party prose, program
structure, source-code expression, and precomputed datasets are not relicensed as part of
`xuan-calendar`.

This distinction is deliberate: the repository license (`MIT OR Apache-2.0`) applies to
the original material in this repository and does not make any claim about the license
of external reference implementations or publications.

Contributions that introduce third-party code, tables, or datasets should document their
source and license explicitly before they are merged.

## Differences from common Hồ Ngọc Đức reference implementations

Hồ Ngọc Đức's calendar articles and programs are useful references for the Vietnamese
lunisolar rules, but `xuan-calendar` is not a source-compatible port. Important
implementation differences include:

| Area | `xuan-calendar` | Common Hồ Ngọc Đức reference approach |
| --- | --- | --- |
| Civil calendar | Proleptic Gregorian throughout | Example conversion code may switch between Julian and Gregorian calendars around 1582 |
| Time zone | Explicit fixed offset in minutes | Common examples use a time-zone value in hours, especially Hanoi `+7` |
| New moon | Compact approximation used directly by the crate | Some reference routines include an additional ΔT correction branch |
| Principal term | Detects a 30-degree crossing across the complete local month interval | Compact routines commonly compare solar-term sectors at new-moon starts |
| Month count | Enumerates local new-moon starts | Compact routines can infer counts from day-span arithmetic |
| Reverse conversion | Bounded search plus exact forward round-trip verification | Usually calculated directly from month offsets and new-moon indices |
| Calendar data | No precomputed lunar-calendar tables | Some published JavaScript versions use precomputed tables for a fixed year range |
| Historical calendar | Not implemented | Hồ Ngọc Đức's site separately provides reconstructed official/historical calendars |
| GanZhi support | Integrated elsewhere in this crate | Outside the scope of the basic lunisolar conversion articles |

These differences are design choices, not claims of greater astronomical precision.

## Accuracy and validation

The astronomical functions are compact approximations rather than a high-precision
planetary/lunar ephemeris. Boundary-sensitive cases can be affected by small timing
errors, especially when a new moon or principal term falls close to local midnight.

The regression suite includes selected Vietnamese and Chinese cases covering ordinary
months, leap months, New Year dates, GanZhi behavior, and round trips. Current test
vectors include selected dates from 1800 through 2620. That range describes regression
coverage only; it is **not** a blanket accuracy guarantee for every date in that interval.

For historical research, dates far outside the modern era, or applications requiring
high-confidence boundary timing, validate results against an authoritative ephemeris or
a trusted calendar oracle.

Future precision work can replace the approximate astronomical layer while preserving
the public calendar rules and local-month model.

## Related crate behavior

The lunisolar algorithm is only one part of `xuan-calendar`. The crate also provides:

- Julian Day helpers;
- solar-term indexing;
- GanZhi year/month/day/hour calculations;
- explicit 23:00 Zi-hour versus midnight day-boundary policies.

Those APIs share the same deterministic input model but should not be confused with the
lunisolar month-numbering rules documented above.

## References

The following references are used for background, validation, and comparison. Their
inclusion here does not imply that their source code or prose is distributed under this
repository's license.

- Hồ Ngọc Đức, *Thuật toán tính âm lịch*: https://www.xemamlich.uhm.vn/calrules.html
- Hồ Ngọc Đức, *Computing the Vietnamese lunar calendar*: https://www.xemamlich.uhm.vn/calrules_en.html
- Jean Meeus, *Astronomical Algorithms*, for standard astronomical approximation methods.
- Edward M. Reingold and Nachum Dershowitz, *Calendrical Calculations*, for calendrical algorithms and terminology.

For the implementation itself, the authoritative source is the Rust code in
`src/lunar.rs`, `src/julian.rs`, and `src/solar.rs`, together with `src/tests.rs`.
