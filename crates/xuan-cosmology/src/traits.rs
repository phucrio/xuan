use super::{WuXing, YinYang};

/// VN: Trait cho các đối tượng có thuộc tính Ngũ Hành.
/// CN: 具有五行属性的对象的特征
pub trait HasWuXing {
    fn wuxing(&self) -> WuXing;
}

/// VN: Trait cho các đối tượng có thuộc tính Âm Dương.
/// CN: 具有阴阳属性的对象的特征
pub trait HasYinYang {
    fn yin_yang(&self) -> YinYang;
}

/// VN: Trait chuyển đối tượng thành string key.
/// CN: 将对象转换为字符串键的特征
pub trait ToKey {
    fn to_key(&self) -> &'static str;
}

/// VN: Trait lấy nhãn hiển thị đa ngôn ngữ.
/// CN: 获取多语言显示标签的特征
pub trait Labeled {
    fn label_vn(&self) -> &'static str;
    fn label_cn(&self) -> &'static str;
}

/// VN: Trait cho các đối tượng có chu kỳ như Thiên Can, Địa Chi.
/// CN: 具有周期属性的对象的特征（如天干、地支）
pub trait CyclicIndex: Copy + Sized + 'static {
    const CYCLE_LEN: usize;
    fn cycle() -> &'static [Self];
    fn index(self) -> usize;

    fn shift(self, offset: i32) -> Self {
        let idx = self.index() as i32;
        let new_idx = (idx + offset).rem_euclid(Self::CYCLE_LEN as i32);
        Self::cycle()[new_idx as usize]
    }

    fn next(self) -> Self {
        self.shift(1)
    }

    fn prev(self) -> Self {
        self.shift(-1)
    }

    fn opposite(self) -> Self {
        self.shift(Self::CYCLE_LEN as i32 / 2)
    }
}
