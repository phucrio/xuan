use super::{Labeled, ToKey};

// VN: Ngũ Hành.
// CN: 五行
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum WuXing {
    Wood,
    Fire,
    Earth,
    Metal,
    Water,
    Unknown,
}

// VN: Quan hệ giữa các Ngũ Hành.
// CN: 五行之间的关系
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WuxingRelation {
    Sheng,
    Ke,
    TongXing,
    Neutral,
}

impl WuXing {
    pub fn generates(self, other: WuXing) -> bool {
        matches!(
            (self, other),
            (WuXing::Wood, WuXing::Fire)
                | (WuXing::Fire, WuXing::Earth)
                | (WuXing::Earth, WuXing::Metal)
                | (WuXing::Metal, WuXing::Water)
                | (WuXing::Water, WuXing::Wood)
        )
    }

    pub fn overcomes(self, other: WuXing) -> bool {
        matches!(
            (self, other),
            (WuXing::Wood, WuXing::Earth)
                | (WuXing::Earth, WuXing::Water)
                | (WuXing::Water, WuXing::Fire)
                | (WuXing::Fire, WuXing::Metal)
                | (WuXing::Metal, WuXing::Wood)
        )
    }

    pub fn relation_to(self, other: WuXing) -> WuxingRelation {
        if self == other {
            WuxingRelation::TongXing
        } else if self.generates(other) {
            WuxingRelation::Sheng
        } else if self.overcomes(other) {
            WuxingRelation::Ke
        } else {
            WuxingRelation::Neutral
        }
    }
}

impl ToKey for WuXing {
    fn to_key(&self) -> &'static str {
        match self {
            WuXing::Wood => "wood",
            WuXing::Fire => "fire",
            WuXing::Earth => "earth",
            WuXing::Metal => "metal",
            WuXing::Water => "water",
            WuXing::Unknown => "unknown",
        }
    }
}

impl Labeled for WuXing {
    fn label_vn(&self) -> &'static str {
        match self {
            WuXing::Wood => "Mộc",
            WuXing::Fire => "Hỏa",
            WuXing::Earth => "Thổ",
            WuXing::Metal => "Kim",
            WuXing::Water => "Thủy",
            WuXing::Unknown => "Không rõ",
        }
    }

    fn label_cn(&self) -> &'static str {
        match self {
            WuXing::Wood => "木",
            WuXing::Fire => "火",
            WuXing::Earth => "土",
            WuXing::Metal => "金",
            WuXing::Water => "水",
            WuXing::Unknown => "未知",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WuxingJu {
    Water2,
    Wood3,
    Metal4,
    Earth5,
    Fire6,
}

impl WuxingJu {
    pub fn number(self) -> u8 {
        match self {
            WuxingJu::Water2 => 2,
            WuxingJu::Wood3 => 3,
            WuxingJu::Metal4 => 4,
            WuxingJu::Earth5 => 5,
            WuxingJu::Fire6 => 6,
        }
    }

    pub fn element(self) -> WuXing {
        match self {
            WuxingJu::Water2 => WuXing::Water,
            WuxingJu::Wood3 => WuXing::Wood,
            WuxingJu::Metal4 => WuXing::Metal,
            WuxingJu::Earth5 => WuXing::Earth,
            WuxingJu::Fire6 => WuXing::Fire,
        }
    }
}
