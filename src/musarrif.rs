// ============================================================
// ملف: src/musarrif.rs
// جزء من: مترجم لغة البيان (Al-Bayan Compiler)
// الوظيفة: المُصَرِّف - تحليل الكلمة العربية إلى جذر ووزن وضمير وزمن
// ============================================================

/// بنية الكلمة بعد التصريف
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Musarrafa {
    /// الجذر الثلاثي أو الرباعي (مثل: حسب، قرأ)
    pub jidhr: String,
    /// الوزن الصرفي (مثل: فَعَلَ، فَاعَلَ، اِستَفعَلَ)
    pub wazn: String,
    /// الزمن: ماض، مضارع، أمر، مستقبل
    pub zaman: Zaman,
    /// الضمائر المتصلة (فاعل ومفعول)
    pub damair: Vec<Damir>,
    /// علامة الإعراب الظاهرة على الكلمة
    pub irab: Option<Irab>,
    /// الكلمة الأصلية قبل التحليل
    pub original: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Zaman {
    Madin,      // ماض (كَتَبَ)
    Mudari3,    // مضارع (يَكْتُبُ)
    Amr,        // أمر (اُكتُبْ)
    Mustaqbal,  // مستقبل (سَيَكْتُبُ)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Damir {
    // ضمائر الرفع (الفاعل)
    Ana,         // أنا (أفعلُ) - همزة المضارع
    Anta,        // أنتَ (تفعلُ) - تاء المخاطب
    Anti,        // أنتِ (تفعلين)
    Huwa,        // هو (يفعلُ) - ياء الغائب
    Hiya,        // هي (تفعلُ)
    Nahnu,       // نحن (نفعلُ) - نون المتكلمين
    Antum,       // أنتم (تفعلون)
    Hum,         // هم (يفعلون)

    // ضمائر النصب (المفعول به) - متصلة
    Ni,          // ني (ياء المتكلم) - Owned
    Ka,          // كَ (كاف المخاطب) - Moved
    Ki,          // كِ (كاف المخاطبة)
    Hu,          // هُ (هاء الغائب) - Shared
    Ha,          // ها (هاء الغائبة)
    Na,          // نا (نا المتكلمين)
    Kum,         // كم (كاف الجمع)
    HumPronoun,  // هم (هاء الجمع)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Irab {
    Raf3,    // مرفوع (ضمة)
    Nasb,    // منصوب (فتحة)
    Jarr,    // مجرور (كسرة)
    Jazm,    // مجزوم (سكون)
}

/// المُصَرِّف الرئيسي
pub struct Musarrif;

impl Musarrif {
    /// تحليل كلمة عربية واحدة إلى مكوناتها الصرفية
    pub fn analyse(word: &str) -> Result<Musarrafa, String> {
        let original = word.to_string();
        let mut w = word.to_string();
        let mut damair = Vec::new();
        let mut zaman = Zaman::Madin; // افتراضي
        let mut irab = None;

        // ==========================================
        // المرحلة ١: نزع الضمائر المتصلة (من النهاية)
        // ==========================================
        let pronoun_endings = vec![
            ("نِي", Damir::Ni), ("نِى", Damir::Ni),
            ("كُم", Damir::Kum), ("كُمْ", Damir::Kum),
            ("هُم", Damir::HumPronoun), ("هُمْ", Damir::HumPronoun),
            ("كَ", Damir::Ka), ("كِ", Damir::Ki),
            ("هُ", Damir::Hu), ("هَا", Damir::Ha),
            ("نَا", Damir::Na),
        ];

        for (suffix, damir) in &pronoun_endings {
            if w.ends_with(suffix) {
                damair.push(damir.clone());
                w = w[..w.len() - suffix.len()].to_string();
                break; // ننزع ضميراً واحداً متصلاً
            }
        }

        // ==========================================
        // المرحلة ٢: تحديد الزمن واستخراج أحرف المضارعة
        // ==========================================

        // المستقبل (سَـ)
        if w.starts_with("سَ") || w.starts_with("سَيُ") || w.starts_with("سَتَ") || w.starts_with("سَنَ") {
            zaman = Zaman::Mustaqbal;
            w = w[2..].to_string(); // نزع "سَ"
        } else if w.starts_with("سَوْفَ") {
            zaman = Zaman::Mustaqbal;
            w = w[5..].to_string(); // نزع "سَوْفَ"
        }

        // حروف المضارعة (أ، ت، ي، ن)
        if w.starts_with("أَ") || w.starts_with("أُ") {
            zaman = if zaman == Zaman::Mustaqbal { zaman } else { Zaman::Mudari3 };
            damair.push(Damir::Ana);
            w = w[2..].to_string(); // نزع "أَ"
        } else if w.starts_with("تَ") {
            zaman = if zaman == Zaman::Mustaqbal { zaman } else { Zaman::Mudari3 };
            damair.push(Damir::Anta); // أو Hiya، لكن هذا يكفي للبدء
            w = w[2..].to_string();
        } else if w.starts_with("يَ") || w.starts_with("يُ") {
            zaman = if zaman == Zaman::Mustaqbal { zaman } else { Zaman::Mudari3 };
            damair.push(Damir::Huwa);
            w = w[2..].to_string();
        } else if w.starts_with("نَ") || w.starts_with("نُ") {
            zaman = if zaman == Zaman::Mustaqbal { zaman } else { Zaman::Mudari3 };
            damair.push(Damir::Nahnu);
            w = w[2..].to_string();
        }

        // ==========================================
        // المرحلة ٣: تحديد الوزن من بداية الكلمة (الزوائد)
        // ==========================================
        let (wazn, core) = if w.starts_with("اِستَ") {
            ("اِستَفعَلَ".to_string(), w[5..].to_string())
        } else if w.starts_with("اِفتَ") {
            ("اِفتَعَلَ".to_string(), w[5..].to_string())
        } else if w.starts_with("اِنفَ") {
            ("اِنفَعَلَ".to_string(), w[5..].to_string())
        } else if w.starts_with("تَفَا") {
            ("تَفَاعَلَ".to_string(), w[5..].to_string())
        } else if w.starts_with("اِ") {
            ("اِفعَل".to_string(), w[2..].to_string()) // أمر
        } else if w.contains("َاعَ") || w.contains("َاعُ") || w.contains("َاعِ") {
            // فَاعَلَ - نبحث عن الألف بين الحرفين الأولين
            ("فَاعَلَ".to_string(), w.to_string())
        } else if w.contains("َعَّ") || w.contains("َعِّ") || w.contains("َعُّ") {
            // فَعَّلَ - نبحث عن التشديد على العين
            ("فَعَّلَ".to_string(), w.to_string())
        } else {
            ("فَعَلَ".to_string(), w.to_string())
        };

        // ==========================================
        // المرحلة ٤: استخراج الجذر من القلب
        // ==========================================
        let jidhr = extract_root(&core, &wazn);

        // ==========================================
        // المرحلة ٥: علامة الإعراب (آخر حرف)
        // ==========================================
        let last_char = w.chars().last().unwrap_or(' ');
        irab = match last_char {
            'ُ' => Some(Irab::Raf3),
            'َ' => Some(Irab::Nasb),
            'ِ' => Some(Irab::Jarr),
            'ْ' => Some(Irab::Jazm),
            _ => None,
        };

        Ok(Musarrafa {
            jidhr,
            wazn,
            zaman,
            damair,
            irab,
            original,
        })
    }
}

/// استخراج الجذر الثلاثي من قلب الكلمة
fn extract_root(core: &str, wazn: &str) -> String {
    // ننزع الحركات ونبقي الحروف فقط
    let letters: Vec<char> = core
        .chars()
        .filter(|c| !is_haraka(*c) && *c != 'ْ' && *c != 'ّ')
        .collect();

    // نأخذ أول ٣ حروف (افتراضي ثلاثي)
    let root: String = letters.iter().take(3).collect();

    if root.is_empty() {
        "??".to_string()
    } else {
        root
    }
}

/// هل هذا الحرف حركة؟
fn is_haraka(c: char) -> bool {
    matches!(c, 'َ' | 'ُ' | 'ِ' | 'ً' | 'ٌ' | 'ٍ' | 'ْ' | 'ّ')
}

// ============================================================
// اختبارات المُصَرِّف
// ============================================================
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_faala_madi() {
        let result = Musarrif::analyse("قَرَأَ").unwrap();
        assert_eq!(result.jidhr, "قرأ");
        assert_eq!(result.wazn, "فَعَلَ");
        assert_eq!(result.zaman, Zaman::Madin);
    }

    #[test]
    fn test_fa3ala_mustaqbal() {
        let result = Musarrif::analyse("سَيُحَسِّبُ").unwrap();
        assert_eq!(result.jidhr, "حسب");
        assert_eq!(result.wazn, "فَعَّلَ");
        assert_eq!(result.zaman, Zaman::Mustaqbal);
    }

    #[test]
    fn test_faa3ala_mudari3() {
        let result = Musarrif::analyse("يُحَاسِبُ").unwrap();
        assert_eq!(result.jidhr, "حسب");
        assert_eq!(result.wazn, "فَاعَلَ");
        assert_eq!(result.zaman, Zaman::Mudari3);
    }

    #[test]
    fn test_istaf3ala() {
        let result = Musarrif::analyse("اِستَقرَأَ").unwrap();
        assert_eq!(result.jidhr, "قرأ");
        assert_eq!(result.wazn, "اِستَفعَلَ");
    }

    #[test]
    fn test_with_damir_muttasil() {
        let result = Musarrif::analyse("يَحْفَظُهُ").unwrap();
        assert_eq!(result.jidhr, "حفظ");
        assert_eq!(result.wazn, "فَعَلَ");
        assert_eq!(result.zaman, Zaman::Mudari3);
        // يجب أن يحتوي على ضمير هاء الغائب
        assert!(result.damair.iter().any(|d| matches!(d, Damir::Hu)));
    }

    #[test]
    fn test_ift3ala() {
        let result = Musarrif::analyse("اِحتَسَبَ").unwrap();
        assert_eq!(result.jidhr, "حسب");
        assert_eq!(result.wazn, "اِفتَعَلَ");
    }

    #[test]
    fn test_infa3ala() {
        let result = Musarrif::analyse("اِنبَعَثَ").unwrap();
        assert_eq!(result.jidhr, "بعث");
        assert_eq!(result.wazn, "اِنفَعَلَ");
    }
}
