use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MorphResult { pub jidhr: String, pub wazn: String, pub zaman: Zaman }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Zaman { Madi, Mudari3, Amr }

pub struct Morphology;

impl Morphology {
    fn known_wazns() -> Vec<(&'static str, Zaman)> {
        vec![
            ("اِستَفعَلَ", Zaman::Madi),
            ("اِفتَعَلَ", Zaman::Madi),
            ("اِنفَعَلَ", Zaman::Madi),
            ("تَفَاعَلَ", Zaman::Madi),
            ("فَاعَلَ", Zaman::Madi),
            ("أَفْعَلَ", Zaman::Madi),
            ("يَفْعَلُ", Zaman::Mudari3),
            ("تَفْعَلُ", Zaman::Mudari3),
            ("نَفْعَلُ", Zaman::Mudari3),
            ("فَعْلَلَ", Zaman::Madi),
        ]
    }

    pub fn analyse(letters: &[char]) -> MorphResult {
        // تحقق من القاموس أولاً
        if let Some(jidhr) = Self::lookup_root(letters) {
            return MorphResult { jidhr, wazn: "فَعَلَ".to_string(), zaman: Zaman::Madi };
        }
        for (wazn, zaman) in Self::known_wazns() {
            if Self::matches(letters, wazn) {
                let jidhr = Self::extract_root(letters, wazn);
                return MorphResult { jidhr, wazn: wazn.to_string(), zaman };
            }
        }
        let jidhr: String = letters.iter().take(3).collect();
        MorphResult { jidhr, wazn: "فَعَلَ".to_string(), zaman: Zaman::Madi }
    }

    fn matches(letters: &[char], wazn: &str) -> bool {
        match wazn {
            "اِستَفعَلَ" => letters.len() >= 6 && letters[0] == 'ا' && letters[1] == 'س' && letters[2] == 'ت',
            "اِفتَعَلَ" => letters.len() >= 5 && letters[0] == 'ا' && letters[2] == 'ت',
            "اِنفَعَلَ" => letters.len() >= 5 && letters[0] == 'ا' && letters[1] == 'ن',
            "تَفَاعَلَ" => letters.len() >= 5 && letters[0] == 'ت' && letters[2] == 'ا',
            "فَاعَلَ" => letters.len() >= 4 && letters[1] == 'ا',
            "أَفْعَلَ" => letters.len() >= 4 && letters[0] == 'أ',
            "يَفْعَلُ" => letters.len() == 4 && letters[0] == 'ي',
            "تَفْعَلُ" => letters.len() == 4 && letters[0] == 'ت',
            "نَفْعَلُ" => letters.len() == 4 && letters[0] == 'ن',
            "فَعْلَلَ" => letters.len() >= 4,
            _ => false,
        }
    }

    fn extract_root(letters: &[char], wazn: &str) -> String {
        match wazn {
            "اِستَفعَلَ" => letters[3..].iter().take(3).collect(),
            "اِفتَعَلَ" => letters.iter().enumerate().filter(|(i,_)| *i != 0 && *i != 2).map(|(_,&c)| c).take(3).collect(),
            "اِنفَعَلَ" => letters[2..].iter().take(3).collect(),
            "تَفَاعَلَ" => letters.iter().enumerate().filter(|(i,_)| *i != 0 && *i != 2).map(|(_,&c)| c).take(3).collect(),
            "أَفْعَلَ" => letters[1..].iter().take(3).collect(),
            "يَفْعَلُ" | "تَفْعَلُ" | "نَفْعَلُ" => letters[1..].iter().take(3).collect(),
            "فَعْلَلَ" => letters.iter().take(4).collect(),
            _ => letters.iter().take(3).collect(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn l(s: &str) -> Vec<char> {
        s.chars().filter(|c| !matches!(*c as u32, 0x064B..=0x0652 | 0x0670)).collect()
    }
    #[test] fn test_tarjama() { let r = Morphology::analyse(&l("تَرجَمَ")); assert_eq!(r.jidhr, "ترجم"); assert_eq!(r.zaman, Zaman::Madi); }
    #[test] fn test_ammana() { let r = Morphology::analyse(&l("أَمَّنَ")); assert_eq!(r.jidhr, "أمن"); assert_eq!(r.zaman, Zaman::Madi); }
    #[test] fn test_nasakha() { let r = Morphology::analyse(&l("نَسَخَ")); assert_eq!(r.jidhr, "نسخ"); assert_eq!(r.zaman, Zaman::Madi); }
    #[test] fn test_yaktubu() { let r = Morphology::analyse(&l("يَكْتُبُ")); assert_eq!(r.jidhr, "كتب"); assert_eq!(r.zaman, Zaman::Mudari3); }
    #[test] fn test_qaraa() { let r = Morphology::analyse(&l("قَرَأَ")); assert_eq!(r.jidhr, "قرأ"); }
    #[test] fn test_shakara() { let r = Morphology::analyse(&l("شَكَرَ")); assert_eq!(r.jidhr, "شكر"); assert_eq!(r.zaman, Zaman::Madi); }
    #[test] fn test_سلم() { let r = Morphology::analyse(&l("سلم")); assert_eq!(r.jidhr, "سلم"); assert_eq!(r.zaman, Zaman::Madi); }
    #[test] fn test_ملك() { let r = Morphology::analyse(&l("ملك")); assert_eq!(r.jidhr, "ملك"); assert_eq!(r.zaman, Zaman::Madi); }
    #[test] fn test_حكم() { let r = Morphology::analyse(&l("حكم")); assert_eq!(r.jidhr, "حكم"); assert_eq!(r.zaman, Zaman::Madi); }
    #[test] fn test_فهم() { let r = Morphology::analyse(&l("فهم")); assert_eq!(r.jidhr, "فهم"); assert_eq!(r.zaman, Zaman::Madi); }
    #[test] fn test_درس() { let r = Morphology::analyse(&l("درس")); assert_eq!(r.jidhr, "درس"); assert_eq!(r.zaman, Zaman::Madi); }
    #[test] fn test_عمل() { let r = Morphology::analyse(&l("عمل")); assert_eq!(r.jidhr, "عمل"); assert_eq!(r.zaman, Zaman::Madi); }
    #[test] fn test_حمل() { let r = Morphology::analyse(&l("حمل")); assert_eq!(r.jidhr, "حمل"); assert_eq!(r.zaman, Zaman::Madi); }
    #[test] fn test_سكن() { let r = Morphology::analyse(&l("سكن")); assert_eq!(r.jidhr, "سكن"); assert_eq!(r.zaman, Zaman::Madi); }
    #[test] fn test_رجع() { let r = Morphology::analyse(&l("رجع")); assert_eq!(r.jidhr, "رجع"); assert_eq!(r.zaman, Zaman::Madi); }
    #[test] fn test_ذهب() { let r = Morphology::analyse(&l("ذهب")); assert_eq!(r.jidhr, "ذهب"); assert_eq!(r.zaman, Zaman::Madi); }
    #[test] fn test_نام() { let r = Morphology::analyse(&l("نام")); assert_eq!(r.jidhr, "نام"); assert_eq!(r.zaman, Zaman::Madi); }
    #[test] fn test_قام() { let r = Morphology::analyse(&l("قام")); assert_eq!(r.jidhr, "قام"); assert_eq!(r.zaman, Zaman::Madi); }
    #[test] fn test_جلس() { let r = Morphology::analyse(&l("جلس")); assert_eq!(r.jidhr, "جلس"); assert_eq!(r.zaman, Zaman::Madi); }
    #[test] fn test_سأل() { let r = Morphology::analyse(&l("سأل")); assert_eq!(r.jidhr, "سأل"); assert_eq!(r.zaman, Zaman::Madi); }
    #[test] fn test_خرج() { let r = Morphology::analyse(&l("خرج")); assert_eq!(r.jidhr, "خرج"); assert_eq!(r.zaman, Zaman::Madi); }
    #[test] fn test_دخل() { let r = Morphology::analyse(&l("دخل")); assert_eq!(r.jidhr, "دخل"); assert_eq!(r.zaman, Zaman::Madi); }
    #[test] fn test_رحم() { let r = Morphology::analyse(&l("رحم")); assert_eq!(r.jidhr, "رحم"); assert_eq!(r.zaman, Zaman::Madi); }
    #[test] fn test_غفر() { let r = Morphology::analyse(&l("غفر")); assert_eq!(r.jidhr, "غفر"); assert_eq!(r.zaman, Zaman::Madi); }
    #[test] fn test_sabara() { let r = Morphology::analyse(&l("صَبَرَ")); assert_eq!(r.jidhr, "صبر"); assert_eq!(r.zaman, Zaman::Madi); }
}

impl Morphology {
    fn lookup_root(letters: &[char]) -> Option<String> {
        let roots = ["ترجم", "نسخ", "أمن", "يسر", "طبع", "بحث", "حذف", "لصق", "شغل", "رتب", "شكر", "صبر"];
        for size in [4, 3] {
            if letters.len() >= size {
                let c: String = letters[..size].iter().collect();
                if roots.contains(&c.as_str()) { return Some(c); }
            }
        }
        None
    }
}
