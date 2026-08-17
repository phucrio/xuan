use super::{WuXing, YinYang};

/// Implemented by values that expose a Five-Phase association.
pub trait HasWuXing {
    fn wuxing(&self) -> WuXing;
}

/// Implemented by values that expose a yin/yang association.
pub trait HasYinYang {
    fn yin_yang(&self) -> YinYang;
}

/// Stable lowercase key used for serialization-adjacent and UI-neutral lookup.
pub trait ToKey {
    fn to_key(&self) -> &'static str;
}

/// Human-readable Vietnamese and Chinese labels.
pub trait Labeled {
    fn label_vn(&self) -> &'static str;
    fn label_cn(&self) -> &'static str;
}

/// Common operations for finite ordered cycles such as stems and branches.
///
/// Implementors must return `cycle()` in the same order represented by
/// `index()`. The default methods depend on that invariant for wrap-around.
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

    /// Return the element half a cycle away.
    ///
    /// This assumes an even-length cycle, which is true for the cycles that
    /// implement this trait in the crate.
    fn opposite(self) -> Self {
        self.shift(Self::CYCLE_LEN as i32 / 2)
    }
}
