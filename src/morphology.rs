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
}

impl Morphology {
    fn lookup_root(letters: &[char]) -> Option<String> {
        let roots = ["ترجم", "نسخ", "أمن", "يسر", "طبع", "بحث", "حذف", "لصق", "شغل", "رتب"];
        for size in [4, 3] {
            if letters.len() >= size {
                let c: String = letters[..size].iter().collect();
                if roots.contains(&c.as_str()) { return Some(c); }
            }
        }
        None
    }
}
