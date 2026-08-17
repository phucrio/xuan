use crate::traits::{CyclicIndex, HasWuXing, HasYinYang, Labeled, ToKey};
use crate::wuxing::WuXing;
use crate::yinyang::YinYang;

// VN: Địa Chi - 12 chi trong hệ Can-Chi.
// CN: 地支 - 干支系统中的十二支
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DiZhi {
    Zi,
    Chou,
    Yin,
    Mao,
    Chen,
    Si,
    Wu,
    Wei,
    Shen,
    You,
    Xu,
    Hai,
}

pub const DIZHI_CYCLE: [DiZhi; 12] = [
    DiZhi::Zi,
    DiZhi::Chou,
    DiZhi::Yin,
    DiZhi::Mao,
    DiZhi::Chen,
    DiZhi::Si,
    DiZhi::Wu,
    DiZhi::Wei,
    DiZhi::Shen,
    DiZhi::You,
    DiZhi::Xu,
    DiZhi::Hai,
];

impl DiZhi {
    pub fn index(self) -> usize {
        self as usize
    }

    pub fn shift(self, offset: i32) -> DiZhi {
        let idx = self.index() as i32;
        let new_idx = (idx + offset).rem_euclid(12);
        DIZHI_CYCLE[new_idx as usize]
    }

    pub fn yin_yang(self) -> YinYang {
        if self.index().is_multiple_of(2) {
            YinYang::Yang
        } else {
            YinYang::Yin
        }
    }

    pub fn wuxing(self) -> WuXing {
        match self {
            DiZhi::Yin | DiZhi::Mao => WuXing::Wood,
            DiZhi::Si | DiZhi::Wu => WuXing::Fire,
            DiZhi::Shen | DiZhi::You => WuXing::Metal,
            DiZhi::Hai | DiZhi::Zi => WuXing::Water,
            DiZhi::Chen | DiZhi::Xu | DiZhi::Chou | DiZhi::Wei => WuXing::Earth,
        }
    }
}

impl HasWuXing for DiZhi {
    fn wuxing(&self) -> WuXing {
        DiZhi::wuxing(*self)
    }
}

impl HasYinYang for DiZhi {
    fn yin_yang(&self) -> YinYang {
        DiZhi::yin_yang(*self)
    }
}

impl ToKey for DiZhi {
    fn to_key(&self) -> &'static str {
        match self {
            DiZhi::Zi => "zi",
            DiZhi::Chou => "chou",
            DiZhi::Yin => "yin",
            DiZhi::Mao => "mao",
            DiZhi::Chen => "chen",
            DiZhi::Si => "si",
            DiZhi::Wu => "wu",
            DiZhi::Wei => "wei",
            DiZhi::Shen => "shen",
            DiZhi::You => "you",
            DiZhi::Xu => "xu",
            DiZhi::Hai => "hai",
        }
    }
}

impl Labeled for DiZhi {
    fn label_vn(&self) -> &'static str {
        match self {
            DiZhi::Zi => "Tý",
            DiZhi::Chou => "Sửu",
            DiZhi::Yin => "Dần",
            DiZhi::Mao => "Mão",
            DiZhi::Chen => "Thìn",
            DiZhi::Si => "Tỵ",
            DiZhi::Wu => "Ngọ",
            DiZhi::Wei => "Mùi",
            DiZhi::Shen => "Thân",
            DiZhi::You => "Dậu",
            DiZhi::Xu => "Tuất",
            DiZhi::Hai => "Hợi",
        }
    }

    fn label_cn(&self) -> &'static str {
        match self {
            DiZhi::Zi => "子",
            DiZhi::Chou => "丑",
            DiZhi::Yin => "寅",
            DiZhi::Mao => "卯",
            DiZhi::Chen => "辰",
            DiZhi::Si => "巳",
            DiZhi::Wu => "午",
            DiZhi::Wei => "未",
            DiZhi::Shen => "申",
            DiZhi::You => "酉",
            DiZhi::Xu => "戌",
            DiZhi::Hai => "亥",
        }
    }
}

impl CyclicIndex for DiZhi {
    const CYCLE_LEN: usize = 12;

    fn cycle() -> &'static [Self] {
        &DIZHI_CYCLE
    }

    fn index(self) -> usize {
        self as usize
    }
}
