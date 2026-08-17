use super::{Labeled, ToKey};

/// The Five Phases (Wu Xing) plus an explicit unknown value.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum WuXing {
    Wood,
    Fire,
    Earth,
    Metal,
    Water,
    Unknown,
}

/// Directional relationship from one Five-Phase value to another.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WuxingRelation {
    /// Generating / nourishing relation (sheng).
    Sheng,
    /// Overcoming / controlling relation (ke).
    Ke,
    /// Same phase.
    TongXing,
    /// No direct generating, overcoming, or same-phase relation.
    Neutral,
}

impl WuXing {
    /// Return whether `self` generates `other` in the canonical sheng cycle:
    /// Wood -> Fire -> Earth -> Metal -> Water -> Wood.
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

    /// Return whether `self` overcomes `other` in the canonical ke cycle:
    /// Wood -> Earth -> Water -> Fire -> Metal -> Wood.
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

    /// Classify the directed relationship from `self` to `other`.
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

/// Five-Phase Bureau values represented by their traditional numeric labels.
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
