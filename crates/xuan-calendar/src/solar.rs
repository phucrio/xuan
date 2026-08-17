use xuan_cosmology::gan::{TIANGAN_CYCLE, TianGan};
use xuan_cosmology::ganzhi::GanZhi;
use xuan_cosmology::zhi::{DIZHI_CYCLE, DiZhi};

/// Fixed-offset time zone used by deterministic calendar calculations.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TimeZone {
    pub offset_minutes: i32,
}

impl TimeZone {
    pub const UTC: TimeZone = TimeZone { offset_minutes: 0 };
    pub const VN: TimeZone = TimeZone {
        offset_minutes: 7 * 60,
    };
    pub const CN: TimeZone = TimeZone {
        offset_minutes: 8 * 60,
    };
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CivilDate {
    pub year: i32,
    pub month: u8,
    pub day: u8,
}

impl CivilDate {
    pub const fn new(year: i32, month: u8, day: u8) -> Self {
        CivilDate { year, month, day }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CivilTime {
    pub hour: u8,
    pub minute: u8,
}

impl CivilTime {
    pub const fn new(hour: u8, minute: u8) -> Self {
        CivilTime { hour, minute }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CivilDateTime {
    pub date: CivilDate,
    pub time: CivilTime,
    pub tz: TimeZone,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GanZhiDateTime {
    pub year: GanZhi,
    pub month: GanZhi,
    pub day: GanZhi,
    pub hour: GanZhi,
}

pub trait ToGanZhi {
    fn to_ganzhi(&self) -> GanZhiDateTime;
}

/// Policy for determining the effective day at the Zi-hour boundary.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum DayBoundaryPolicy {
    /// Treat 23:00 as the beginning of the next effective day.
    #[default]
    ZiAt23,
    /// Use the civil 00:00 day boundary.
    Midnight,
}

impl CivilDateTime {
    pub const fn new(date: CivilDate, time: CivilTime, tz: TimeZone) -> Self {
        CivilDateTime { date, time, tz }
    }

    pub fn effective_date(&self, policy: DayBoundaryPolicy) -> CivilDate {
        match policy {
            DayBoundaryPolicy::Midnight => self.date,
            DayBoundaryPolicy::ZiAt23 => {
                if self.time.hour == 23 {
                    add_days(self.date, 1)
                } else {
                    self.date
                }
            }
        }
    }

    /// Earthly Branch of the hour using the 23:00 Zi-hour convention.
    pub fn hour_branch_wenmo(&self) -> DiZhi {
        match self.time.hour {
            23 | 0 => DiZhi::Zi,
            1 | 2 => DiZhi::Chou,
            3 | 4 => DiZhi::Yin,
            5 | 6 => DiZhi::Mao,
            7 | 8 => DiZhi::Chen,
            9 | 10 => DiZhi::Si,
            11 | 12 => DiZhi::Wu,
            13 | 14 => DiZhi::Wei,
            15 | 16 => DiZhi::Shen,
            17 | 18 => DiZhi::You,
            19 | 20 => DiZhi::Xu,
            21 | 22 => DiZhi::Hai,
            _ => DiZhi::Zi,
        }
    }
}

fn add_days(date: CivilDate, days: i32) -> CivilDate {
    crate::julian::add_days_gregorian(date, days)
}

#[inline]
fn gan_from_index(idx: i32) -> TianGan {
    TIANGAN_CYCLE[idx.rem_euclid(10) as usize]
}

#[inline]
fn zhi_from_index(idx: i32) -> DiZhi {
    DIZHI_CYCLE[idx.rem_euclid(12) as usize]
}

#[inline]
fn ganzhi_from_indices(gan_idx: i32, zhi_idx: i32) -> GanZhi {
    GanZhi {
        gan: gan_from_index(gan_idx),
        zhi: zhi_from_index(zhi_idx),
    }
}

fn gregorian_to_jdn(date: CivilDate) -> i64 {
    let y = date.year as i64;
    let m = date.month as i64;
    let d = date.day as i64;

    let a = (14 - m) / 12;
    let y2 = y + 4800 - a;
    let m2 = m + 12 * a - 3;

    d + (153 * m2 + 2) / 5 + 365 * y2 + y2 / 4 - y2 / 100 + y2 / 400 - 32045
}

fn jd_utc_from_civil(dt: CivilDateTime) -> f64 {
    let jd_local = crate::julian::gregorian_to_jd(dt.date, dt.time);
    let tz_days = dt.tz.offset_minutes as f64 / 1440.0;
    jd_local - tz_days
}

fn sun_longitude_deg_utc(jd_utc: f64) -> f64 {
    let t = (jd_utc - 2451545.0) / 36525.0;
    let t2 = t * t;

    let m = 357.52910 + 35999.05030 * t - 0.0001559 * t2 - 0.00000048 * t * t2;
    let l0 = 280.46645 + 36000.76983 * t + 0.0003032 * t2;

    let mrad = m.to_radians();
    let c = (1.914600 - 0.004817 * t - 0.000014 * t2) * mrad.sin()
        + (0.019993 - 0.000101 * t) * (2.0 * mrad).sin()
        + 0.000290 * (3.0 * mrad).sin();

    let theta = l0 + c;
    let omega = (125.04 - 1934.136 * t).to_radians();
    let lambda = theta - 0.00569 - 0.00478 * omega.sin();

    let mut deg = lambda % 360.0;
    if deg < 0.0 {
        deg += 360.0;
    }
    if deg >= 360.0 {
        deg -= 360.0;
    }
    deg
}

fn lichun_jd_utc(year: i32, tz: TimeZone) -> f64 {
    let mut lo = jd_utc_from_civil(CivilDateTime {
        date: CivilDate {
            year,
            month: 2,
            day: 1,
        },
        time: CivilTime { hour: 0, minute: 0 },
        tz,
    });
    let mut hi = jd_utc_from_civil(CivilDateTime {
        date: CivilDate {
            year,
            month: 2,
            day: 10,
        },
        time: CivilTime { hour: 0, minute: 0 },
        tz,
    });

    while sun_longitude_deg_utc(lo) > 315.0 {
        lo -= 1.0;
    }
    while sun_longitude_deg_utc(hi) < 315.0 {
        hi += 1.0;
    }

    for _ in 0..40 {
        let mid = (lo + hi) * 0.5;
        if sun_longitude_deg_utc(mid) < 315.0 {
            lo = mid;
        } else {
            hi = mid;
        }
    }
    hi
}

/// Return the 24-term solar-term index for a civil date and time zone.
pub fn solar_term_index(date: CivilDate, tz: TimeZone) -> u8 {
    let jd_utc = jd_utc_from_civil(CivilDateTime {
        date,
        time: CivilTime {
            hour: 12,
            minute: 0,
        },
        tz,
    });
    (sun_longitude_deg_utc(jd_utc) / 15.0).floor() as u8
}

impl ToGanZhi for CivilDateTime {
    fn to_ganzhi(&self) -> GanZhiDateTime {
        let eff_date = self.effective_date(DayBoundaryPolicy::ZiAt23);
        let jdn = gregorian_to_jdn(eff_date);

        let day_gan_idx = (jdn + 9).rem_euclid(10) as i32;
        let day_zhi_idx = (jdn + 1).rem_euclid(12) as i32;
        let day = ganzhi_from_indices(day_gan_idx, day_zhi_idx);

        let hour_zhi = self.hour_branch_wenmo();
        let hour_zhi_idx = hour_zhi as i32;
        let hour_gan_idx = (day_gan_idx * 2 + hour_zhi_idx).rem_euclid(10);
        let hour = ganzhi_from_indices(hour_gan_idx, hour_zhi_idx);

        let jd_utc = jd_utc_from_civil(*self);
        let lichun_jd = lichun_jd_utc(self.date.year, self.tz);
        let year_for_gz = if jd_utc < lichun_jd {
            self.date.year - 1
        } else {
            self.date.year
        };

        let year_diff = year_for_gz - 1984;
        let year_gan_idx = year_diff.rem_euclid(10);
        let year_zhi_idx = year_diff.rem_euclid(12);
        let year = ganzhi_from_indices(year_gan_idx, year_zhi_idx);

        let lunar = crate::lunar::gregorian_to_lunar(eff_date, self.tz)
            .expect("invalid civil date for lunar conversion");
        let month_index = lunar.month as i32 - 1;
        let month_zhi_idx = (DiZhi::Yin as i32 + month_index).rem_euclid(12);
        let month_gan_idx = (year_gan_idx * 2 + 2 + month_index).rem_euclid(10);
        let month = ganzhi_from_indices(month_gan_idx, month_zhi_idx);

        GanZhiDateTime {
            year,
            month,
            day,
            hour,
        }
    }
}
