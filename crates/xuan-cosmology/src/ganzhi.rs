use crate::gan::{TIANGAN_CYCLE, TianGan};
use crate::zhi::{DIZHI_CYCLE, DiZhi};

// VN: Cặp Can-Chi (Giáp Tý, Ất Sửu, ...)
// CN: 干支组合（甲子、乙丑、...）
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GanZhi {
    pub gan: TianGan,
    pub zhi: DiZhi,
}

impl GanZhi {
    pub fn new(gan: TianGan, zhi: DiZhi) -> Self {
        GanZhi { gan, zhi }
    }

    // VN: Lấy index trong vòng 60 Giáp Tý (0-59)
    // CN: 获取在60甲子中的索引（0-59）
    pub fn sexagenary_index(self) -> Option<usize> {
        // VN: Can và Chi phải cùng tính chẵn/lẻ mới hợp lệ
        // CN: 天干地支必须同阴阳才是有效的组合
        let gan_idx = self.gan.index();
        let zhi_idx = self.zhi.index();
        if gan_idx % 2 != zhi_idx % 2 {
            return None;
        }
        let idx = (6 * gan_idx as i32 - 5 * zhi_idx as i32).rem_euclid(60);
        Some(idx as usize)
    }

    // VN: Tạo GanZhi từ index trong vòng 60 (0-59)
    // CN: 从60甲子索引创建干支组合（0-59）
    pub fn from_index(idx: usize) -> Self {
        let gan = TIANGAN_CYCLE[idx % 10];
        let zhi = DIZHI_CYCLE[idx % 12];
        GanZhi { gan, zhi }
    }

    /// VN: Tạo GanZhi từ năm theo chu kỳ.
    pub fn from_year(year: i32) -> Self {
        let gan_idx = (year + 6).rem_euclid(10) as usize;
        let zhi_idx = (year + 8).rem_euclid(12) as usize;
        GanZhi {
            gan: TIANGAN_CYCLE[gan_idx],
            zhi: DIZHI_CYCLE[zhi_idx],
        }
    }
}

// VN: Tính Can-Chi của cung từ năm Can và Địa Chi của cung.
// CN: 根据年干和宫位地支计算宫位干支
// Dựa trên Ngũ Hổ Độn để xác định Can bắt đầu tại Dần.
pub fn palace_ganzhi(year_gan: TianGan, dizhi: DiZhi) -> (TianGan, DiZhi) {
    let year_gan_idx = year_gan.index() as i32;
    let start_gan_idx = ((year_gan_idx % 5) * 2 + 2).rem_euclid(10);
    let month_index = (dizhi.index() as i32 - DiZhi::Yin.index() as i32).rem_euclid(12);
    let gan_idx = (start_gan_idx + month_index).rem_euclid(10);
    let gan = TIANGAN_CYCLE[gan_idx as usize];
    (gan, dizhi)
}
