use super::solar::{CivilDate, CivilTime};

/// Convert a Gregorian date to a Julian Day Number (JDN) at 00:00.
///
/// Uses the proleptic Gregorian calendar and is independent of time zone and
/// operating-system state.
pub fn gregorian_to_jdn(date: CivilDate) -> i64 {
    let y = date.year as i64;
    let m = date.month as i64;
    let d = date.day as i64;

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

/// Add days in the proleptic Gregorian calendar via JDN arithmetic.
pub fn add_days_gregorian(date: CivilDate, days: i32) -> CivilDate {
    let jdn = gregorian_to_jdn(date);
    let jdn2 = jdn + days as i64;
    jdn_to_gregorian(jdn2)
}

/// Convert a Gregorian local date/time to Julian Day.
pub fn gregorian_to_jd(date: CivilDate, time: CivilTime) -> f64 {
    let mut y = date.year as i64;
    let mut m = date.month as i64;
    let d = date.day as f64;

    if m <= 2 {
        y -= 1;
        m += 12;
    }

    let a = y / 100;
    let b = 2 - a + a / 4;
    let day_fraction = time.hour as f64 / 24.0 + time.minute as f64 / 1440.0;

    (365.25 * (y as f64 + 4716.0)).floor()
        + (30.6001 * ((m + 1) as f64)).floor()
        + d
        + day_fraction
        + b as f64
        - 1524.5
}

/// Convert Julian Day to a Gregorian date and minute-resolution time.
pub fn jd_to_gregorian(jd: f64) -> (CivilDate, CivilTime) {
    let z = (jd + 0.5).floor();
    let f = (jd + 0.5) - z;

    let mut a = z;
    let alpha = ((z - 1867216.25) / 36524.25).floor();
    a += 1.0 + alpha - (alpha / 4.0).floor();

    let b = a + 1524.0;
    let c = ((b - 122.1) / 365.25).floor();
    let d = (365.25 * c).floor();
    let e = ((b - d) / 30.6001).floor();

    let day = (b - d - (30.6001 * e).floor()) as u8;
    let month = if e < 14.0 {
        (e - 1.0) as u8
    } else {
        (e - 13.0) as u8
    };
    let year = if month > 2 {
        (c - 4716.0) as i32
    } else {
        (c - 4715.0) as i32
    };

    let mut date = CivilDate { year, month, day };
    let day_fraction = f * 24.0;
    let mut hour = day_fraction.floor() as i32;
    let mut minute = ((day_fraction - hour as f64) * 60.0).round() as i32;

    if minute == 60 {
        minute = 0;
        hour += 1;
    }
    if hour == 24 {
        hour = 0;
        date = add_days_gregorian(date, 1);
    }

    (
        date,
        CivilTime {
            hour: hour as u8,
            minute: minute as u8,
        },
    )
}
