use crate::gan::TianGan;
use crate::traits::{HasWuXing, HasYinYang, Labeled, ToKey};
use crate::wuxing::{WuXing, WuxingRelation};
use crate::yinyang::YinYang;
use crate::zhi::DiZhi;

#[test]
fn wuxing_generates_cycle() {
    assert!(WuXing::Wood.generates(WuXing::Fire));
    assert!(WuXing::Fire.generates(WuXing::Earth));
    assert!(WuXing::Earth.generates(WuXing::Metal));
    assert!(WuXing::Metal.generates(WuXing::Water));
    assert!(WuXing::Water.generates(WuXing::Wood));
}

#[test]
fn wuxing_generates_negative() {
    assert!(!WuXing::Fire.generates(WuXing::Wood));
    assert!(!WuXing::Wood.generates(WuXing::Metal));
    assert!(!WuXing::Wood.generates(WuXing::Wood));
}

#[test]
fn wuxing_overcomes_cycle() {
    assert!(WuXing::Wood.overcomes(WuXing::Earth));
    assert!(WuXing::Earth.overcomes(WuXing::Water));
    assert!(WuXing::Water.overcomes(WuXing::Fire));
    assert!(WuXing::Fire.overcomes(WuXing::Metal));
    assert!(WuXing::Metal.overcomes(WuXing::Wood));
}

#[test]
fn wuxing_overcomes_negative() {
    assert!(!WuXing::Earth.overcomes(WuXing::Wood));
    assert!(!WuXing::Wood.overcomes(WuXing::Fire));
    assert!(!WuXing::Fire.overcomes(WuXing::Fire));
}

#[test]
fn wuxing_relation_to() {
    assert_eq!(
        WuXing::Wood.relation_to(WuXing::Wood),
        WuxingRelation::TongXing
    );
    assert_eq!(
        WuXing::Wood.relation_to(WuXing::Fire),
        WuxingRelation::Sheng
    );
    assert_eq!(WuXing::Wood.relation_to(WuXing::Earth), WuxingRelation::Ke);
    assert_eq!(
        WuXing::Wood.relation_to(WuXing::Metal),
        WuxingRelation::Neutral
    );
    assert_eq!(
        WuXing::Wood.relation_to(WuXing::Water),
        WuxingRelation::Neutral
    );
}

#[test]
fn tiangan_wuxing() {
    assert_eq!(TianGan::Jia.wuxing(), WuXing::Wood);
    assert_eq!(TianGan::Yi.wuxing(), WuXing::Wood);
    assert_eq!(TianGan::Bing.wuxing(), WuXing::Fire);
    assert_eq!(TianGan::Ding.wuxing(), WuXing::Fire);
    assert_eq!(TianGan::Wu.wuxing(), WuXing::Earth);
    assert_eq!(TianGan::Ji.wuxing(), WuXing::Earth);
    assert_eq!(TianGan::Geng.wuxing(), WuXing::Metal);
    assert_eq!(TianGan::Xin.wuxing(), WuXing::Metal);
    assert_eq!(TianGan::Ren.wuxing(), WuXing::Water);
    assert_eq!(TianGan::Gui.wuxing(), WuXing::Water);
}

#[test]
fn dizhi_wuxing() {
    assert_eq!(DiZhi::Yin.wuxing(), WuXing::Wood);
    assert_eq!(DiZhi::Mao.wuxing(), WuXing::Wood);
    assert_eq!(DiZhi::Si.wuxing(), WuXing::Fire);
    assert_eq!(DiZhi::Wu.wuxing(), WuXing::Fire);
    assert_eq!(DiZhi::Shen.wuxing(), WuXing::Metal);
    assert_eq!(DiZhi::You.wuxing(), WuXing::Metal);
    assert_eq!(DiZhi::Hai.wuxing(), WuXing::Water);
    assert_eq!(DiZhi::Zi.wuxing(), WuXing::Water);
    assert_eq!(DiZhi::Chen.wuxing(), WuXing::Earth);
    assert_eq!(DiZhi::Xu.wuxing(), WuXing::Earth);
    assert_eq!(DiZhi::Chou.wuxing(), WuXing::Earth);
    assert_eq!(DiZhi::Wei.wuxing(), WuXing::Earth);
}

#[test]
fn dizhi_yin_yang() {
    assert_eq!(DiZhi::Zi.yin_yang(), YinYang::Yang);
    assert_eq!(DiZhi::Yin.yin_yang(), YinYang::Yang);
    assert_eq!(DiZhi::Chen.yin_yang(), YinYang::Yang);
    assert_eq!(DiZhi::Wu.yin_yang(), YinYang::Yang);
    assert_eq!(DiZhi::Chou.yin_yang(), YinYang::Yin);
    assert_eq!(DiZhi::Mao.yin_yang(), YinYang::Yin);
    assert_eq!(DiZhi::Si.yin_yang(), YinYang::Yin);
    assert_eq!(DiZhi::Hai.yin_yang(), YinYang::Yin);
}

#[test]
fn test_traits_wuxing() {
    fn check_wuxing<T: HasWuXing>(obj: T, expected: WuXing) {
        assert_eq!(obj.wuxing(), expected);
    }
    check_wuxing(TianGan::Jia, WuXing::Wood);
    check_wuxing(DiZhi::Zi, WuXing::Water);
}

#[test]
fn test_traits_yinyang() {
    fn check_yinyang<T: HasYinYang>(obj: T, expected: YinYang) {
        assert_eq!(obj.yin_yang(), expected);
    }
    check_yinyang(TianGan::Jia, YinYang::Yang);
    check_yinyang(DiZhi::Chou, YinYang::Yin);
}

#[test]
fn test_traits_tokey() {
    fn check_key<T: ToKey>(obj: T, expected: &str) {
        assert_eq!(obj.to_key(), expected);
    }
    check_key(WuXing::Wood, "wood");
    check_key(TianGan::Jia, "jia");
}

#[test]
fn test_traits_labeled() {
    fn check_labels<T: Labeled>(obj: T, vn: &str, cn: &str) {
        assert_eq!(obj.label_vn(), vn);
        assert_eq!(obj.label_cn(), cn);
    }
    check_labels(WuXing::Wood, "Mộc", "木");
    check_labels(TianGan::Jia, "Giáp", "甲");
    check_labels(DiZhi::Zi, "Tý", "子");
}
