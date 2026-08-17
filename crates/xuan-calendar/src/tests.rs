use crate::lunar::{LunarDate, gregorian_to_lunar, lunar_to_gregorian};
use crate::solar::{CivilDate, CivilDateTime, CivilTime, DayBoundaryPolicy, TimeZone, ToGanZhi};
use xuan_cosmology::gan::TianGan;
use xuan_cosmology::zhi::DiZhi;

#[test]
fn wenmo_zi_boundary() {
    let tz = TimeZone::VN;

    let before = CivilDateTime {
        date: CivilDate {
            year: 2026,
            month: 1,
            day: 26,
        },
        time: CivilTime {
            hour: 22,
            minute: 59,
        },
        tz,
    };
    assert_eq!(before.effective_date(DayBoundaryPolicy::ZiAt23).day, 26);
    assert_eq!(before.hour_branch_wenmo(), DiZhi::Hai);

    let boundary = CivilDateTime {
        date: CivilDate {
            year: 2026,
            month: 1,
            day: 26,
        },
        time: CivilTime {
            hour: 23,
            minute: 0,
        },
        tz,
    };
    assert_eq!(boundary.effective_date(DayBoundaryPolicy::ZiAt23).day, 27);
    assert_eq!(boundary.hour_branch_wenmo(), DiZhi::Zi);
}

#[test]
fn ganzhi_day_boundary_alignment() {
    let tz = TimeZone::VN;

    let dt_before = CivilDateTime {
        date: CivilDate {
            year: 2026,
            month: 1,
            day: 26,
        },
        time: CivilTime {
            hour: 22,
            minute: 59,
        },
        tz,
    };
    let dt_boundary = CivilDateTime {
        date: CivilDate {
            year: 2026,
            month: 1,
            day: 26,
        },
        time: CivilTime {
            hour: 23,
            minute: 0,
        },
        tz,
    };
    let dt_next = CivilDateTime {
        date: CivilDate {
            year: 2026,
            month: 1,
            day: 27,
        },
        time: CivilTime { hour: 0, minute: 0 },
        tz,
    };

    let gz_before = dt_before.to_ganzhi();
    let gz_boundary = dt_boundary.to_ganzhi();
    let gz_next = dt_next.to_ganzhi();

    assert_eq!(gz_boundary.day, gz_next.day);
    assert_ne!(gz_before.day, gz_boundary.day);
    assert_eq!(gz_boundary.hour.zhi, dt_boundary.hour_branch_wenmo());
}

#[test]
fn ganzhi_wenmo_case_2026_01_30_vn() {
    let dt = CivilDateTime {
        date: CivilDate {
            year: 2026,
            month: 1,
            day: 30,
        },
        time: CivilTime { hour: 0, minute: 0 },
        tz: TimeZone::VN,
    };

    let lunar = gregorian_to_lunar(dt.date, dt.tz).unwrap();
    assert_eq!(lunar.year, 2025);
    assert_eq!(lunar.month, 12);
    assert_eq!(lunar.day, 12);
    assert!(!lunar.is_leap_month);

    let gz = dt.to_ganzhi();
    assert_eq!(gz.year.gan, TianGan::Yi);
    assert_eq!(gz.year.zhi, DiZhi::Si);
    assert_eq!(gz.month.gan, TianGan::Ji);
    assert_eq!(gz.month.zhi, DiZhi::Chou);
    assert_eq!(gz.day.gan, TianGan::Jia);
    assert_eq!(gz.day.zhi, DiZhi::Chen);
    assert_eq!(gz.hour.gan, TianGan::Jia);
    assert_eq!(gz.hour.zhi, DiZhi::Zi);
}

#[test]
fn ganzhi_wenmo_case_1988_04_09_1130_vn() {
    let dt = CivilDateTime {
        date: CivilDate {
            year: 1988,
            month: 4,
            day: 9,
        },
        time: CivilTime {
            hour: 11,
            minute: 30,
        },
        tz: TimeZone::VN,
    };

    let lunar = gregorian_to_lunar(dt.date, dt.tz).unwrap();
    assert_eq!(lunar.year, 1988);
    assert_eq!(lunar.month, 2);
    assert_eq!(lunar.day, 23);
    assert!(!lunar.is_leap_month);

    let gz = dt.to_ganzhi();
    assert_eq!(gz.year.gan, TianGan::Wu);
    assert_eq!(gz.year.zhi, DiZhi::Chen);
    assert_eq!(gz.month.gan, TianGan::Yi);
    assert_eq!(gz.month.zhi, DiZhi::Mao);
    assert_eq!(gz.day.gan, TianGan::Jia);
    assert_eq!(gz.day.zhi, DiZhi::Wu);
    assert_eq!(gz.hour.gan, TianGan::Geng);
    assert_eq!(gz.hour.zhi, DiZhi::Wu);
}

#[test]
fn ganzhi_wenmo_case_2002_10_17_1630_vn() {
    let dt = CivilDateTime {
        date: CivilDate {
            year: 2002,
            month: 10,
            day: 17,
        },
        time: CivilTime {
            hour: 16,
            minute: 30,
        },
        tz: TimeZone::VN,
    };

    let lunar = gregorian_to_lunar(dt.date, dt.tz).unwrap();
    assert_eq!(lunar.year, 2002);
    assert_eq!(lunar.month, 9);
    assert_eq!(lunar.day, 12);
    assert!(!lunar.is_leap_month);

    let gz = dt.to_ganzhi();
    assert_eq!(gz.year.gan, TianGan::Ren);
    assert_eq!(gz.year.zhi, DiZhi::Wu);
    assert_eq!(gz.month.gan, TianGan::Geng);
    assert_eq!(gz.month.zhi, DiZhi::Xu);
    assert_eq!(gz.day.gan, TianGan::Wu);
    assert_eq!(gz.day.zhi, DiZhi::Wu);
    assert_eq!(gz.hour.gan, TianGan::Geng);
    assert_eq!(gz.hour.zhi, DiZhi::Shen);
}

struct LunarCase {
    label: &'static str,
    civil: CivilDate,
    expected: LunarDate,
}

#[test]
fn lunar_cases_table() {
    let cases: &[LunarCase] = &[
        LunarCase {
            label: "VN Tet 2026",
            civil: CivilDate::new(2026, 2, 17),
            expected: LunarDate {
                year: 2026,
                month: 1,
                day: 1,
                is_leap_month: false,
                tz: TimeZone::VN,
            },
        },
        LunarCase {
            label: "VN leap month 2025",
            civil: CivilDate::new(2025, 7, 26),
            expected: LunarDate {
                year: 2025,
                month: 6,
                day: 2,
                is_leap_month: true,
                tz: TimeZone::VN,
            },
        },
        LunarCase {
            label: "VN sample 2002-10-17",
            civil: CivilDate::new(2002, 10, 17),
            expected: LunarDate {
                year: 2002,
                month: 9,
                day: 12,
                is_leap_month: false,
                tz: TimeZone::VN,
            },
        },
        LunarCase {
            label: "VN sample 2006-06-17",
            civil: CivilDate::new(2006, 6, 17),
            expected: LunarDate {
                year: 2006,
                month: 5,
                day: 22,
                is_leap_month: false,
                tz: TimeZone::VN,
            },
        },
        LunarCase {
            label: "VN 2000-01-17",
            civil: CivilDate::new(2000, 1, 17),
            expected: LunarDate {
                year: 1999,
                month: 12,
                day: 11,
                is_leap_month: false,
                tz: TimeZone::VN,
            },
        },
        LunarCase {
            label: "VN 2001-01-18",
            civil: CivilDate::new(2001, 1, 18),
            expected: LunarDate {
                year: 2000,
                month: 12,
                day: 24,
                is_leap_month: false,
                tz: TimeZone::VN,
            },
        },
        LunarCase {
            label: "VN 2026-01-26",
            civil: CivilDate::new(2026, 1, 26),
            expected: LunarDate {
                year: 2025,
                month: 12,
                day: 8,
                is_leap_month: false,
                tz: TimeZone::VN,
            },
        },
        LunarCase {
            label: "CN New Year 2025",
            civil: CivilDate::new(2025, 1, 29),
            expected: LunarDate {
                year: 2025,
                month: 1,
                day: 1,
                is_leap_month: false,
                tz: TimeZone::CN,
            },
        },
        LunarCase {
            label: "CN leap month 2023",
            civil: CivilDate::new(2023, 3, 22),
            expected: LunarDate {
                year: 2023,
                month: 2,
                day: 1,
                is_leap_month: true,
                tz: TimeZone::CN,
            },
        },
        LunarCase {
            label: "VN leap month 1952",
            civil: CivilDate::new(1952, 6, 22),
            expected: LunarDate {
                year: 1952,
                month: 5,
                day: 1,
                is_leap_month: true,
                tz: TimeZone::VN,
            },
        },
        LunarCase {
            label: "VN leap month 2028",
            civil: CivilDate::new(2028, 6, 23),
            expected: LunarDate {
                year: 2028,
                month: 5,
                day: 1,
                is_leap_month: true,
                tz: TimeZone::VN,
            },
        },
        LunarCase {
            label: "VN leap month 2052",
            civil: CivilDate::new(2052, 9, 23),
            expected: LunarDate {
                year: 2052,
                month: 8,
                day: 1,
                is_leap_month: true,
                tz: TimeZone::VN,
            },
        },
        LunarCase {
            label: "VN leap month 2080",
            civil: CivilDate::new(2080, 4, 20),
            expected: LunarDate {
                year: 2080,
                month: 3,
                day: 1,
                is_leap_month: true,
                tz: TimeZone::VN,
            },
        },
        LunarCase {
            label: "VN Tet 2172",
            civil: CivilDate::new(2172, 1, 25),
            expected: LunarDate {
                year: 2172,
                month: 1,
                day: 1,
                is_leap_month: false,
                tz: TimeZone::VN,
            },
        },
        LunarCase {
            label: "VN Tet 2199",
            civil: CivilDate::new(2199, 1, 27),
            expected: LunarDate {
                year: 2199,
                month: 1,
                day: 1,
                is_leap_month: false,
                tz: TimeZone::VN,
            },
        },
        LunarCase {
            label: "VN leap month 2199",
            civil: CivilDate::new(2199, 7, 23),
            expected: LunarDate {
                year: 2199,
                month: 6,
                day: 1,
                is_leap_month: true,
                tz: TimeZone::VN,
            },
        },
        LunarCase {
            label: "VN 1800-01-01",
            civil: CivilDate::new(1800, 1, 1),
            expected: LunarDate {
                year: 1799,
                month: 12,
                day: 7,
                is_leap_month: false,
                tz: TimeZone::VN,
            },
        },
        LunarCase {
            label: "VN 1890-05-19",
            civil: CivilDate::new(1890, 5, 19),
            expected: LunarDate {
                year: 1890,
                month: 4,
                day: 1,
                is_leap_month: false,
                tz: TimeZone::VN,
            },
        },
        LunarCase {
            label: "VN leap month 1890",
            civil: CivilDate::new(1890, 3, 21),
            expected: LunarDate {
                year: 1890,
                month: 2,
                day: 1,
                is_leap_month: true,
                tz: TimeZone::VN,
            },
        },
        LunarCase {
            label: "VN Tet 2289",
            civil: CivilDate::new(2289, 1, 22),
            expected: LunarDate {
                year: 2289,
                month: 1,
                day: 1,
                is_leap_month: false,
                tz: TimeZone::VN,
            },
        },
        LunarCase {
            label: "VN leap month 2389",
            civil: CivilDate::new(2389, 7, 24),
            expected: LunarDate {
                year: 2389,
                month: 6,
                day: 1,
                is_leap_month: true,
                tz: TimeZone::VN,
            },
        },
        LunarCase {
            label: "VN leap month 2620",
            civil: CivilDate::new(2620, 6, 22),
            expected: LunarDate {
                year: 2620,
                month: 5,
                day: 1,
                is_leap_month: true,
                tz: TimeZone::VN,
            },
        },
    ];

    for case in cases {
        let got = gregorian_to_lunar(case.civil, case.expected.tz).unwrap();
        assert_eq!(got, case.expected, "lunar mismatch for {}", case.label);
        let back_civil = lunar_to_gregorian(case.expected).unwrap();
        assert_eq!(
            back_civil, case.civil,
            "round-trip failed for {}",
            case.label
        );
        let back_again = gregorian_to_lunar(back_civil, case.expected.tz).unwrap();
        assert_eq!(
            back_again, case.expected,
            "reverse round-trip failed for {}",
            case.label
        );
    }
}
