use crate::gan::{TIANGAN_CYCLE, TianGan};
use crate::zhi::{DIZHI_CYCLE, DiZhi};

/// A Heavenly Stem / Earthly Branch pair.
///
/// Only pairs whose stem and branch have matching yin/yang parity occur in the
/// canonical 60-position sexagenary cycle.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GanZhi {
    pub gan: TianGan,
    pub zhi: DiZhi,
}

impl GanZhi {
    pub fn new(gan: TianGan, zhi: DiZhi) -> Self {
        GanZhi { gan, zhi }
    }

    /// Return the zero-based position in the canonical 60-pair cycle.
    ///
    /// A valid cycle pair must have the same parity on both component cycles:
    /// even stem with even branch, or odd stem with odd branch. Pairs such as
    /// Jia-Chou therefore return `None` rather than being projected onto the
    /// sexagenary cycle.
    pub fn sexagenary_index(self) -> Option<usize> {
        let gan_idx = self.gan.index();
        let zhi_idx = self.zhi.index();
        if gan_idx % 2 != zhi_idx % 2 {
            return None;
        }

        // Solves the simultaneous congruences index % 10 = gan_idx and
        // index % 12 = zhi_idx for the parity-compatible pair.
        let idx = (6 * gan_idx as i32 - 5 * zhi_idx as i32).rem_euclid(60);
        Some(idx as usize)
    }

    /// Build a pair from a zero-based sexagenary-cycle position.
    ///
    /// The modulo operations intentionally wrap indices larger than 59.
    pub fn from_index(idx: usize) -> Self {
        let gan = TIANGAN_CYCLE[idx % 10];
        let zhi = DIZHI_CYCLE[idx % 12];
        GanZhi { gan, zhi }
    }

    /// Build the conventional year pair for a Gregorian-style year number.
    ///
    /// The offsets are equivalent to anchoring 1984 as Jia-Zi (cycle index 0).
    /// `rem_euclid` keeps the mapping well-defined for years before the anchor.
    pub fn from_year(year: i32) -> Self {
        let gan_idx = (year + 6).rem_euclid(10) as usize;
        let zhi_idx = (year + 8).rem_euclid(12) as usize;
        GanZhi {
            gan: TIANGAN_CYCLE[gan_idx],
            zhi: DIZHI_CYCLE[zhi_idx],
        }
    }
}

/// Derive the stem paired with a target branch using the Five Tigers rule.
///
/// The year stem determines the stem assigned to Yin, the first lunar-month
/// branch. The target branch is then reached by advancing both cycles together
/// from Yin while preserving the supplied branch unchanged.
pub fn palace_ganzhi(year_gan: TianGan, dizhi: DiZhi) -> (TianGan, DiZhi) {
    let year_gan_idx = year_gan.index() as i32;

    // Year stems repeat the same first-month stem every five positions.
    let start_gan_idx = ((year_gan_idx % 5) * 2 + 2).rem_euclid(10);

    // Yin is branch index 2 and represents the first lunar month in this rule.
    let month_index = (dizhi.index() as i32 - DiZhi::Yin.index() as i32).rem_euclid(12);
    let gan_idx = (start_gan_idx + month_index).rem_euclid(10);
    let gan = TIANGAN_CYCLE[gan_idx as usize];
    (gan, dizhi)
}
