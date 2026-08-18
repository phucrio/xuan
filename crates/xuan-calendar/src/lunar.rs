//! Vietnamese and Chinese lunisolar calendar conversion.
//!
//! The implementation uses astronomical new-moon and apparent-solar-longitude
//! approximations, with an explicit fixed UTC offset deciding which local civil
//! day contains an astronomical event.
//!
//! Important boundary rule: lunisolar month/day conversion always uses the
//! civil midnight boundary of the requested time zone. The optional 23:00
//! Zi-hour boundary used by GanZhi calculations is a separate convention and is
//! intentionally not applied here.

use super::solar::{CivilDate, CivilDateTime, CivilTime, GanZhiDateTime, TimeZone, ToGanZhi};

/// Lunisolar date in the requested fixed-offset calendar context.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LunarDate {
    pub year: i32,
    pub month: u8,
    pub day: u8,
    pub is_leap_month: bool,
    pub tz: TimeZone,
}

/// Lunisolar date plus civil time, used when deriving GanZhi hour information.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LunarDateTime {
    pub date: LunarDate,
    pub time: CivilTime,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LunarError {
    OutOfRange,
    InvalidInput,
}

// -----------------------------------------------------------------------------
// Integer day-number helpers
// -----------------------------------------------------------------------------
// Integer JDN arithmetic is used for local civil-day boundaries. Keeping month
// starts as integers avoids letting floating-point noise move an event across a
// local date boundary after the astronomical instant has been classified.

fn gregorian_to_jdn(date: CivilDate) -> i64 {
    let y = date.year as i64;
    let m = date.month as i64;
    let d = date.day as i64;

    // Proleptic Gregorian form of the Fliegel-Van Flandern conversion.
    let a = (14 - m) / 12;
    let y2 = y + 4800 - a;
    let m2 = m + 12 * a - 3;

    d + (153 * m2 + 2) / 5 + 365 * y2 + y2 / 4 - y2 / 100 + y2 / 400 - 32045
}

fn jdn_to_gregorian(jdn: i64) -> CivilDate {
    let a = jdn + 32044;
    let b = (4 * a + 3) / 146097;
    let c = a - (146097 * b) / 4;

    let d = (4 * c + 3) / 1461;
    let e = c - (1461 * d) / 4;
    let m = (5 * e + 2) / 153;

    let day = (e - (153 * m + 2) / 5 + 1) as u8;
    let month = (m + 3 - 12 * (m / 10)) as u8;
    let year = (100 * b + d - 4800 + (m / 10)) as i32;

    CivilDate { year, month, day }
}

// -----------------------------------------------------------------------------
// Astronomical approximations
// -----------------------------------------------------------------------------
// These routines reimplement published new-moon and solar-longitude equations
// in Rust. They are numerical ingredients, not a precomputed calendar table.
// The public docs describe provenance, limitations, and differences from common
// reference implementations in more detail.

const PI: f64 = std::f64::consts::PI;

fn norm_rad(x: f64) -> f64 {
    let mut v = x % (2.0 * PI);
    if v < 0.0 {
        v += 2.0 * PI;
    }
    v
}

fn deg_to_rad(d: f64) -> f64 {
    d * PI / 180.0
}

/// Approximate the UTC Julian Day of lunation `k`.
///
/// `k` is counted relative to the conventional epoch used by this family of
/// new-moon approximations near the beginning of 1900. The result is an
/// astronomical instant; local date assignment happens separately.
fn new_moon_jd(k: i32) -> f64 {
    let kf = k as f64;
    let t = kf / 1236.85;
    let t2 = t * t;
    let t3 = t2 * t;

    // Mean new-moon estimate followed by the principal periodic corrections.
    let mut jd = 2415020.75933 + 29.53058868 * kf + 0.0001178 * t2 - 0.000000155 * t3;
    jd += 0.00033 * deg_to_rad(166.56 + 132.87 * t - 0.009173 * t2).sin();

    // Mean anomaly of the Sun, mean anomaly of the Moon, and lunar argument of
    // latitude. They are expressed in degrees before conversion to radians.
    let m = 359.2242 + 29.10535608 * kf - 0.0000333 * t2 - 0.00000347 * t3;
    let mp = 306.0253 + 385.81691806 * kf + 0.0107306 * t2 + 0.00001236 * t3;
    let f = 21.2964 + 390.67050646 * kf - 0.0016528 * t2 - 0.00000239 * t3;

    let m = deg_to_rad(m);
    let mp = deg_to_rad(mp);
    let f = deg_to_rad(f);

    let correction = (0.1734 - 0.000393 * t) * m.sin() + 0.0021 * (2.0 * m).sin()
        - 0.4068 * mp.sin()
        + 0.0161 * (2.0 * mp).sin()
        - 0.0004 * (3.0 * mp).sin()
        + 0.0104 * (2.0 * f).sin()
        - 0.0051 * (m + mp).sin()
        - 0.0074 * (m - mp).sin()
        + 0.0004 * (2.0 * f + m).sin()
        - 0.0004 * (2.0 * f - m).sin()
        - 0.0006 * (2.0 * f + mp).sin()
        + 0.0010 * (2.0 * f - mp).sin()
        + 0.0005 * (m + 2.0 * mp).sin();

    jd + correction
}

/// Approximate apparent solar longitude at a UTC Julian Day, normalized to [0, 2π).
fn sun_longitude_rad(jd: f64) -> f64 {
    // Julian centuries from J2000.0.
    let t = (jd - 2451545.0) / 36525.0;
    let t2 = t * t;

    let m = 357.52910 + 35999.05030 * t - 0.0001559 * t2 - 0.00000048 * t * t2;
    let l0 = 280.46645 + 36000.76983 * t + 0.0003032 * t2;

    let mrad = deg_to_rad(m);
    let c = (1.914600 - 0.004817 * t - 0.000014 * t2) * mrad.sin()
        + (0.019993 - 0.000101 * t) * (2.0 * mrad).sin()
        + 0.000290 * (3.0 * mrad).sin();

    let theta = l0 + c;
    let omega = deg_to_rad(125.04 - 1934.136 * t);
    let lambda = theta - 0.00569 - 0.00478 * omega.sin();

    norm_rad(deg_to_rad(lambda))
}

// -----------------------------------------------------------------------------
// Local month boundaries and principal terms
// -----------------------------------------------------------------------------

/// Return the local civil JDN containing lunation `k`.
fn new_moon_day_local(k: i32, tz: TimeZone) -> i64 {
    let jd = new_moon_jd(k);

    // Convert the UTC astronomical instant into local civil time before taking
    // the containing day. This is why the same new moon can begin on different
    // civil dates for two time zones near midnight.
    let tz_days = tz.offset_minutes as f64 / 1440.0;
    let jd_local = jd + tz_days;
    (jd_local + 0.5).floor() as i64
}

fn major_solar_term_index_by_jd(jd_utc: f64) -> i32 {
    let lon = sun_longitude_rad(jd_utc);
    ((lon / (PI / 6.0)).floor() as i32) % 12
}

/// Test whether a local lunar-month interval contains a principal solar term.
///
/// Principal terms are 30-degree solar-longitude boundaries. The check uses
/// the complete half-open local month interval `[start, end)` instead of only
/// sampling longitude at a new-moon instant.
fn month_has_principal_term_local(start_local_jdn: i64, end_local_jdn: i64, tz: TimeZone) -> bool {
    // Convert both local-midnight boundaries back to UTC Julian Days.
    let tz_days = tz.offset_minutes as f64 / 1440.0;
    let jd_start = start_local_jdn as f64 - 0.5 - tz_days;
    let jd_end = end_local_jdn as f64 - 0.5 - tz_days;

    let lon_start = sun_longitude_rad(jd_start);
    let mut lon_end = sun_longitude_rad(jd_end);

    // Unwrap the 2π -> 0 discontinuity so a month crossing 360 degrees still
    // has a monotonically increasing longitude interval.
    if lon_end < lon_start {
        lon_end += 2.0 * PI;
    }

    let next_boundary = (lon_start / (PI / 6.0)).floor() * (PI / 6.0) + (PI / 6.0);
    lon_end >= next_boundary - 1e-9
}

/// Estimate the lunation index near a local JDN so searches start close by.
fn k_from_jdn(jdn: i64) -> i32 {
    ((jdn as f64 - 2415021.0) / 29.530588853).floor() as i32
}

/// Return the local JDN of the new moon that starts lunar month 11.
///
/// Month 11 is the month containing the winter solstice. The end-of-year seed
/// finds a nearby new moon; if it lies on the wrong side of the relevant solar
/// term region, the algorithm steps back one lunation.
fn lunar_month11_jdn(year: i32, tz: TimeZone) -> i64 {
    let jdn_31_12 = gregorian_to_jdn(CivilDate {
        year,
        month: 12,
        day: 31,
    });

    let mut k = k_from_jdn(jdn_31_12);
    let mut nm = new_moon_day_local(k, tz);
    let term = major_solar_term_index_by_jd(new_moon_jd(k));

    if term >= 9 {
        k -= 1;
        nm = new_moon_day_local(k, tz);
    }

    nm
}

/// Find the leap-month offset between two consecutive month-11 anchors.
///
/// In a 13-month span, the first lunar month that contains no principal solar
/// term is the leap month. Month starts are enumerated from local new-moon days
/// instead of estimating the count from a mean synodic-month length.
fn leap_month_offset(a11: i64, b11: i64, tz: TimeZone) -> i32 {
    let mut starts = Vec::<i64>::new();
    let mut k = k_from_jdn(a11);

    // Align the estimated lunation index exactly with the known A11 boundary.
    while new_moon_day_local(k, tz) < a11 {
        k += 1;
    }
    while new_moon_day_local(k, tz) > a11 {
        k -= 1;
    }

    loop {
        let s = new_moon_day_local(k, tz);
        if s >= b11 {
            break;
        }
        starts.push(s);
        k += 1;
    }
    starts.push(b11);

    for i in 0..starts.len() - 1 {
        if !month_has_principal_term_local(starts[i], starts[i + 1], tz) {
            return i as i32;
        }
    }
    0
}

/// Convert a Gregorian civil date to the Vietnamese/Chinese lunisolar date for
/// the supplied fixed-offset time zone.
pub fn gregorian_to_lunar(date: CivilDate, tz: TimeZone) -> Result<LunarDate, LunarError> {
    let jdn = gregorian_to_jdn(date);

    // Locate the local new moon that starts the lunar month containing `date`.
    let k = k_from_jdn(jdn);
    let mut month_start = new_moon_day_local(k + 1, tz);
    if month_start > jdn {
        month_start = new_moon_day_local(k, tz);
    }

    // Find the pair of month-11 anchors bracketing the relevant lunar year.
    let mut a11 = lunar_month11_jdn(date.year, tz);
    let mut b11 = lunar_month11_jdn(date.year + 1, tz);
    if jdn < a11 {
        b11 = a11;
        a11 = lunar_month11_jdn(date.year - 1, tz);
    }

    // A date after B11 belongs to the next anchor pair.
    if month_start >= b11 {
        a11 = lunar_month11_jdn(date.year + 1, tz);
        b11 = lunar_month11_jdn(date.year + 2, tz);
    }

    let lunar_day = (jdn - month_start + 1) as u8;

    // Count actual local new-moon boundaries from month 11 to the target month.
    let mut k_scan = k_from_jdn(a11);
    while new_moon_day_local(k_scan, tz) < a11 {
        k_scan += 1;
    }
    let mut diff = 0;
    while new_moon_day_local(k_scan + diff, tz) < month_start {
        diff += 1;
    }
    let mut lunar_month = diff + 11;
    let mut is_leap = false;

    // Determine whether A11..B11 contains 12 or 13 lunar months by explicit
    // boundary enumeration. Thirteen months require leap-month selection.
    let mut months_between = 0;
    let mut k_tmp = k_from_jdn(a11);
    while new_moon_day_local(k_tmp, tz) < a11 {
        k_tmp += 1;
    }
    while new_moon_day_local(k_tmp, tz) < b11 {
        months_between += 1;
        k_tmp += 1;
    }

    if months_between > 12 {
        let leap_off = leap_month_offset(a11, b11, tz);

        // Once the leap month is inserted, subsequent numeric month labels move
        // back by one. The leap month itself repeats the preceding month number.
        if leap_off != 0 && diff >= leap_off {
            lunar_month -= 1;
            if diff == leap_off {
                is_leap = true;
            }
        }
    }

    if lunar_month > 12 {
        lunar_month -= 12;
    }
    if lunar_month < 1 {
        lunar_month += 12;
    }

    // Lunar months 11 and 12 can fall in January/February of the following
    // civil year, so adjust the reported lunar year accordingly.
    let mut lunar_year = date.year;
    if lunar_month >= 11 && date.month <= 2 {
        lunar_year -= 1;
    }

    Ok(LunarDate {
        year: lunar_year,
        month: lunar_month as u8,
        day: lunar_day,
        is_leap_month: is_leap,
        tz,
    })
}

/// Convert a Vietnamese/Chinese lunisolar date back to Gregorian.
///
/// The reverse path intentionally favors correctness and code reuse over a
/// separate closed-form inverse. It scans a bounded civil-date range and accepts
/// the first date whose forward conversion exactly reproduces the input.
pub fn lunar_to_gregorian(lunar: LunarDate) -> Result<CivilDate, LunarError> {
    let a11_prev = lunar_month11_jdn(lunar.year - 1, lunar.tz);
    let b11 = lunar_month11_jdn(lunar.year + 1, lunar.tz);

    for jdn in a11_prev..=b11 {
        let gregorian = jdn_to_gregorian(jdn);
        if let Ok(candidate) = gregorian_to_lunar(gregorian, lunar.tz)
            && candidate == lunar
        {
            return Ok(gregorian);
        }
    }

    Err(LunarError::InvalidInput)
}

impl LunarDateTime {
    pub fn to_civil_datetime(&self) -> Result<CivilDateTime, LunarError> {
        let date = lunar_to_gregorian(self.date)?;
        Ok(CivilDateTime {
            date,
            time: self.time,
            tz: self.date.tz,
        })
    }
}

impl ToGanZhi for LunarDateTime {
    fn to_ganzhi(&self) -> GanZhiDateTime {
        // GanZhi conversion is defined on the equivalent local civil date/time;
        // keeping the conversion in one place avoids divergent pillar rules.
        let civil = self
            .to_civil_datetime()
            .expect("invalid lunar date for GanZhi conversion");
        civil.to_ganzhi()
    }
}
