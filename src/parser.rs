use crate::musarrif::{Musarrif, Musarrafa};

#[derive(Debug, Clone)]
pub struct ArabicSentence {
    pub verb: Option<Musarrafa>,
    pub subject: Option<String>,
    pub object: Option<String>,
    pub preposition: Option<String>,
    pub genitive: Option<String>,
    pub adverbs: Vec<String>,
    pub original: String,
    pub errors: Vec<String>,
}

pub struct SentenceParser;

impl SentenceParser {
    pub fn parse(text: &str) -> Result<ArabicSentence, String> {
        let original = text.to_string();
        let words: Vec<&str> = text.split_whitespace().collect();
        if words.is_empty() { return Err("جملة فارغة".to_string()); }

        let mut sentence = ArabicSentence {
            verb: None, subject: None, object: None,
            preposition: None, genitive: None,
            adverbs: vec![], original, errors: vec![],
        };

        let prepositions = ["في", "على", "من", "إلى", "عن", "بِ", "لِ", "كَ", "حتى"];
        let mut skip_next = false;

        for (i, word) in words.iter().enumerate() {
            if skip_next { skip_next = false; continue; }
            if i == 0 {
                if let Ok(a) = Musarrif::analyse(word) {
                    sentence.verb = Some(a);
                    continue;
                }
            }
            if prepositions.contains(word) {
                sentence.preposition = Some(word.to_string());
                if i + 1 < words.len() {
                    sentence.genitive = Some(words[i + 1].to_string());
                    skip_next = true;
                }
                continue;
            }
            if sentence.subject.is_none() && sentence.verb.is_some() {
                sentence.subject = Some(word.to_string());
                continue;
            }
            if sentence.object.is_none() {
                sentence.object = Some(word.to_string());
                continue;
            }
            sentence.adverbs.push(word.to_string());
        }

        sentence.validate();
        Ok(sentence)
    }
}

impl ArabicSentence {
    pub fn validate(&mut self) {
        self.errors.clear();

        if let Some(ref subj) = self.subject {
            if !Self::has_damma(subj) && !Self::is_pronoun(subj) {
                self.errors.push(format!("❌ خطأ نحوي: الفاعل '{}' يجب أن يكون مرفوعاً", subj));
            }
        }

        if let Some(ref obj) = self.object {
            if !Self::has_fatha(obj) && !Self::is_pronoun(obj) {
                self.errors.push(format!("❌ خطأ نحوي: المفعول به '{}' يجب أن يكون منصوباً", obj));
            }
        }

        if self.preposition.is_some() && self.genitive.is_none() {
            self.errors.push("❌ خطأ نحوي: حرف الجر يحتاج إلى اسم مجرور".to_string());
        }

        if let Some(ref gen) = self.genitive {
            if !Self::has_kasra(gen) && !Self::is_pronoun(gen) {
                self.errors.push(format!("❌ خطأ نحوي: المجرور '{}' يجب أن يكون مجروراً", gen));
            }
        }

        if let Some(ref verb) = self.verb {
            if Self::is_transitive(&verb.jidhr) && self.object.is_none() {
                self.errors.push(format!("❌ خطأ نحوي: الفعل '{}' متعدٍ ويحتاج مفعولاً به", verb.original));
            }
        }

        // تأنيث الفعل
        if let Some(ref verb) = self.verb {
            if let Some(ref subj) = self.subject {
                let vf = verb.original.contains('ت');
                let sf = subj.ends_with('ة') || subj.ends_with('ى');
                if vf && !sf && !Self::is_pronoun(subj) {
                    self.errors.push(format!("⚠️ تحذير: الفعل '{}' مؤنث لكن الفاعل '{}' مذكر", verb.original, subj));
                }
            }
        }
    }

    fn has_damma(word: &str) -> bool { word.ends_with('ُ') || word.ends_with("ٌ") }
    fn has_fatha(word: &str) -> bool { word.ends_with('َ') || word.ends_with("ً") }
    fn has_kasra(word: &str) -> bool { word.ends_with('ِ') || word.ends_with("ٍ") }
    fn is_pronoun(word: &str) -> bool { matches!(word, "هو" | "هي" | "أنا" | "أنت" | "أنتِ" | "نحن" | "هم") }
    fn is_transitive(jidhr: &str) -> bool { matches!(jidhr, "قرأ" | "كتب" | "حسب" | "بعث" | "جمع" | "رسم" | "حفظ" | "فتح" | "نشر") }

    pub fn display(&self) -> String {
        let mut r = String::new();
        if let Some(ref v) = self.verb { r.push_str(&format!("🔍 فعل: {} (جذر:{}, وزن:{})\n", v.original, v.jidhr, v.wazn)); }
        if let Some(ref s) = self.subject { r.push_str(&format!("👤 فاعل: {}\n", s)); }
        if let Some(ref o) = self.object { r.push_str(&format!("📦 مفعول به: {}\n", o)); }
        if let (Some(ref p), Some(ref g)) = (&self.preposition, &self.genitive) { r.push_str(&format!("📍 جار ومجرور: {} {}\n", p, g)); }
        if !self.errors.is_empty() {
            r.push_str("\n╔══════════════════════════╗\n║   🛑 الأخطاء النحوية   ║\n╚══════════════════════════╝\n");
            for (i, e) in self.errors.iter().enumerate() { r.push_str(&format!("{}. {}\n", i + 1, e)); }
        } else { r.push_str("\n✅ الجملة صحيحة نحوياً\n"); }
        r
    }

    pub fn execute(&self) -> String { self.display() }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid() { assert!(SentenceParser::parse("قَرَأَ محمدُ الكتابَ").unwrap().errors.is_empty()); }
    #[test]
    fn test_subject() { assert!(!SentenceParser::parse("قَرَأَ محمدَ الكتابَ").unwrap().errors.is_empty()); }
    #[test]
    fn test_object() { assert!(!SentenceParser::parse("قَرَأَ محمدُ الكتابُ").unwrap().errors.is_empty()); }
    #[test]
    fn test_transitive() { assert!(!SentenceParser::parse("قَرَأَ محمدُ").unwrap().errors.is_empty()); }
    #[test]
    fn test_preposition() { assert!(!SentenceParser::parse("قَرَأَ محمدُ في").unwrap().errors.is_empty()); }
}

// ============================================
// الإعراب التقديري - المرحلة الثانية
// ============================================

impl ArabicSentence {
    /// كشف الإعراب التقديري (للأسماء المقصورة والمنقوصة والمبنية)
    pub fn detect_taqdeeri(&self, word: &str) -> String {
        // الأسماء المقصورة (تنتهي بألف لازمة)
        if word.ends_with('ى') || word.ends_with("َى") || word.ends_with("َىٰ") {
            return format!("📌 '{}': اسم مقصور - إعرابه تقديري (الحركات مقدرة على الألف)", word);
        }

        // الأسماء المنقوصة (تنتهي بياء لازمة)
        if word.ends_with("ِي") || word.ends_with("ِيْ") {
            return format!("📌 '{}': اسم منقوص - إعرابه تقديري (الضمة والكسرة مقدرتان على الياء)", word);
        }

        // الأسماء المبنية (لا تتغير حركتها)
        let mabni = ["هذا", "هذه", "الذي", "التي", "أنا", "أنت", "هو", "هي", "نحن", "هم", "ما", "مَن"];
        if mabni.contains(&word) {
            return format!("📌 '{}': اسم مبني - لا محل له من الإعراب", word);
        }

        // الأسماء الخمسة (أب، أخ، حم، فو، ذو)
        let asma_khamsa = ["أب", "أخ", "حم", "فو", "ذو"];
        for name in &asma_khamsa {
            if word.starts_with(name) {
                return format!("📌 '{}': من الأسماء الخمسة - ترفع بالواو وتنصب بالألف وتجر بالياء", word);
            }
        }

        // الممنوع من الصرف
        let mamnu_min_sarf = ["أحمدُ", "عمرُ", "مصرُ", "دمشقُ", "بغدادُ"];
        if mamnu_min_sarf.contains(&word) {
            return format!("📌 '{}': ممنوع من الصرف - لا ينون", word);
        }

        "".to_string()
    }

    /// تحليل إعرابي كامل مع التقديري
    pub fn full_irab(&self) -> String {
        let mut result = String::from("📋 التحليل الإعرابي الكامل:\n");

        if let Some(ref verb) = self.verb {
            result.push_str(&format!("   🔍 '{}': فعل (جذر: {}, وزن: {})\n", verb.original, verb.jidhr, verb.wazn));
        }

        if let Some(ref subj) = self.subject {
            result.push_str(&format!("   👤 '{}': فاعل مرفوع", subj));
            let taqdeeri = self.detect_taqdeeri(subj);
            if !taqdeeri.is_empty() {
                result.push_str(&format!(" ({})", taqdeeri.split(": ").last().unwrap_or("")));
            }
            result.push_str("\n");
        }

        if let Some(ref obj) = self.object {
            result.push_str(&format!("   📦 '{}': مفعول به منصوب", obj));
            let taqdeeri = self.detect_taqdeeri(obj);
            if !taqdeeri.is_empty() {
                result.push_str(&format!(" ({})", taqdeeri.split(": ").last().unwrap_or("")));
            }
            result.push_str("\n");
        }

        if let (Some(ref prep), Some(ref gen)) = (&self.preposition, &self.genitive) {
            result.push_str(&format!("   📍 '{}': مجرور بـ '{}'", gen, prep));
            let taqdeeri = self.detect_taqdeeri(gen);
            if !taqdeeri.is_empty() {
                result.push_str(&format!(" ({})", taqdeeri.split(": ").last().unwrap_or("")));
            }
            result.push_str("\n");
        }

        result
    }
}

#[cfg(test)]
mod taqdeeri_tests {
    use super::*;

    #[test]
    fn test_maqsour() {
        let s = SentenceParser::parse("قَرَأَ الفتى الكتابَ").unwrap();
        let analysis = s.detect_taqdeeri("الفتى");
        assert!(analysis.contains("مقصور"));
    }

    #[test]
    fn test_manqous() {
        let s = SentenceParser::parse("جاءَ القاضي").unwrap();
        let analysis = s.detect_taqdeeri("القاضي");
        assert!(analysis.contains("منقوص"));
    }

    #[test]
    fn test_mabni() {
        let s = SentenceParser::parse("قَرَأَ هذا الكتابَ").unwrap();
        let analysis = s.detect_taqdeeri("هذا");
        assert!(analysis.contains("مبني"));
    }

    #[test]
    fn test_full_irab() {
        let s = SentenceParser::parse("قَرَأَ الفتى الكتابَ").unwrap();
        let result = s.full_irab();
        assert!(result.contains("مقصور"));
        assert!(result.contains("الفتى"));
    }
}
