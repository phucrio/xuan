use super::{HasYinYang, Labeled, ToKey};

/// Yin/Yang polarity with an explicit unknown state for incomplete data.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum YinYang {
    Yang,
    Yin,
    Unknown,
}

impl ToKey for YinYang {
    fn to_key(&self) -> &'static str {
        match self {
            YinYang::Yang => "yang",
            YinYang::Yin => "yin",
            YinYang::Unknown => "unknown",
        }
    }
}

impl Labeled for YinYang {
    fn label_vn(&self) -> &'static str {
        match self {
            YinYang::Yang => "Dương",
            YinYang::Yin => "Âm",
            YinYang::Unknown => "Không rõ",
        }
    }

    fn label_cn(&self) -> &'static str {
        match self {
            YinYang::Yang => "阳",
            YinYang::Yin => "阴",
            YinYang::Unknown => "未知",
        }
    }
}

impl HasYinYang for YinYang {
    fn yin_yang(&self) -> YinYang {
        *self
    }
}

/// Strategy trait for values whose polarity is derived rather than stored.
pub trait YinYangRule {
    fn yin_yang(&self) -> YinYang;
}
