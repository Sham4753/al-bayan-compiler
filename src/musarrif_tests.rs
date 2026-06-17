#[cfg(test)]
mod musarrif_new_tests {
    use crate::musarrif::Musarrif;

    // ========== الأفعال الماضية اللي تبدأ بـ (أ، ت، ي، ن) ==========

    #[test]
    fn test_nasakha_madi() {
        // نَسَخَ: النون أصلية (جذر: ن-س-خ)
        let r = Musarrif::analyse("نَسَخَ").unwrap();
        assert_eq!(r.jidhr, "نسخ", "نَسَخَ: النون أصلية، الجذر نسخ");
        assert_eq!(r.wazn, "فَعَلَ");
    }

    #[test]
    fn test_tarjama_madi() {
        // تَرجَمَ: التاء أصلية (جذر: ت-ر-ج-م)
        let r = Musarrif::analyse("تَرجَمَ").unwrap();
        assert_eq!(r.jidhr, "ترجم", "تَرجَمَ: التاء أصلية، الجذر ترجم");
        assert_eq!(r.wazn, "فَعَلَ");
    }

    #[test]
    fn test_ammana_madi() {
        // أَمَّنَ: الألف أصلية (جذر: أ-م-ن)
        let r = Musarrif::analyse("أَمَّنَ").unwrap();
        assert_eq!(r.jidhr, "أمن", "أَمَّنَ: الألف أصلية، الجذر أمن");
    }

    #[test]
    fn test_yasara_madi() {
        // يَسَرَ: الياء أصلية (جذر: ي-س-ر)
        let r = Musarrif::analyse("يَسَرَ").unwrap();
        assert_eq!(r.jidhr, "يسر", "يَسَرَ: الياء أصلية، الجذر يسر");
    }

    // ========== الأفعال المضارعة (نتأكد ما تنكسرش) ==========

    #[test]
    fn test_yaktubu_mudari3() {
        // يَكْتُبُ: الياء مضارعة (جذر: ك-ت-ب)
        let r = Musarrif::analyse("يَكْتُبُ").unwrap();
        assert_eq!(r.jidhr, "كتب", "يَكْتُبُ: الياء مضارعة، الجذر كتب");
    }

    #[test]
    fn test_taktubu_mudari3() {
        // تَكْتُبُ: التاء مضارعة (جذر: ك-ت-ب)
        let r = Musarrif::analyse("تَكْتُبُ").unwrap();
        assert_eq!(r.jidhr, "كتب", "تَكْتُبُ: التاء مضارعة، الجذر كتب");
    }
}
