use crate::traits::{CyclicIndex, HasWuXing, HasYinYang, Labeled, ToKey};
use crate::wuxing::WuXing;
use crate::yinyang::YinYang;

/// The ten Heavenly Stems in canonical cycle order.
///
/// The discriminant is the zero-based cycle index and is relied on by cyclic
/// arithmetic throughout the crate.
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TianGan {
    Jia,  // 甲 / Giáp
    Yi,   // 乙 / Ất
    Bing, // 丙 / Bính
    Ding, // 丁 / Đinh
    Wu,   // 戊 / Mậu
    Ji,   // 己 / Kỷ
    Geng, // 庚 / Canh
    Xin,  // 辛 / Tân
    Ren,  // 壬 / Nhâm
    Gui,  // 癸 / Quý
}

pub const TIANGAN_CYCLE: [TianGan; 10] = [
    TianGan::Jia,
    TianGan::Yi,
    TianGan::Bing,
    TianGan::Ding,
    TianGan::Wu,
    TianGan::Ji,
    TianGan::Geng,
    TianGan::Xin,
    TianGan::Ren,
    TianGan::Gui,
];

impl TianGan {
    pub fn index(self) -> usize {
        self as usize
    }

    /// Shift by a signed number of positions with wrap-around.
    pub fn shift(self, offset: i32) -> TianGan {
        let idx = self.index() as i32;
        let new_idx = (idx + offset).rem_euclid(10);
        TIANGAN_CYCLE[new_idx as usize]
    }

    /// Yin/yang alternates across the stem cycle, beginning with Yang at Jia.
    pub fn yin_yang(self) -> YinYang {
        if self.index().is_multiple_of(2) {
            YinYang::Yang
        } else {
            YinYang::Yin
        }
    }

    /// Five-Phase association of the Heavenly Stems.
    ///
    /// Each phase owns two adjacent stems: Wood, Fire, Earth, Metal, Water.
    pub fn wuxing(self) -> WuXing {
        match self {
            TianGan::Jia | TianGan::Yi => WuXing::Wood,
            TianGan::Bing | TianGan::Ding => WuXing::Fire,
            TianGan::Wu | TianGan::Ji => WuXing::Earth,
            TianGan::Geng | TianGan::Xin => WuXing::Metal,
            TianGan::Ren | TianGan::Gui => WuXing::Water,
        }
    }
}

impl HasWuXing for TianGan {
    fn wuxing(&self) -> WuXing {
        TianGan::wuxing(*self)
    }
}

impl HasYinYang for TianGan {
    fn yin_yang(&self) -> YinYang {
        TianGan::yin_yang(*self)
    }
}

impl ToKey for TianGan {
    fn to_key(&self) -> &'static str {
        match self {
            TianGan::Jia => "jia",
            TianGan::Yi => "yi",
            TianGan::Bing => "bing",
            TianGan::Ding => "ding",
            TianGan::Wu => "wu",
            TianGan::Ji => "ji",
            TianGan::Geng => "geng",
            TianGan::Xin => "xin",
            TianGan::Ren => "ren",
            TianGan::Gui => "gui",
        }
    }
}

impl Labeled for TianGan {
    fn label_vn(&self) -> &'static str {
        match self {
            TianGan::Jia => "Giáp",
            TianGan::Yi => "Ất",
            TianGan::Bing => "Bính",
            TianGan::Ding => "Đinh",
            TianGan::Wu => "Mậu",
            TianGan::Ji => "Kỷ",
            TianGan::Geng => "Canh",
            TianGan::Xin => "Tân",
            TianGan::Ren => "Nhâm",
            TianGan::Gui => "Quý",
        }
    }

    fn label_cn(&self) -> &'static str {
        match self {
            TianGan::Jia => "甲",
            TianGan::Yi => "乙",
            TianGan::Bing => "丙",
            TianGan::Ding => "丁",
            TianGan::Wu => "戊",
            TianGan::Ji => "己",
            TianGan::Geng => "庚",
            TianGan::Xin => "辛",
            TianGan::Ren => "壬",
            TianGan::Gui => "癸",
        }
    }
}

impl CyclicIndex for TianGan {
    const CYCLE_LEN: usize = 10;

    fn cycle() -> &'static [Self] {
        &TIANGAN_CYCLE
    }

    fn index(self) -> usize {
        self as usize
    }
}
