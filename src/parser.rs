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

        let prepositions = ["في", "على", "من", "إلى", "عن", "بِ", "لِ", "كَ", "حتى", "مذ", "منذ"];
        let mut skip_next = false;

        for (i, word) in words.iter().enumerate() {
            if skip_next { skip_next = false; continue; }

            if i == 0 {
                match Musarrif::analyse(word) {
                    Ok(a) => { sentence.verb = Some(a); continue; }
                    Err(_) => {}
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

        // تشغيل المدقق النحوي
        sentence.validate();

        Ok(sentence)
    }
}

impl ArabicSentence {
    /// المدقق النحوي الكامل
    pub fn validate(&mut self) {
        self.errors.clear();

        // ١. الفاعل يجب أن يكون مرفوعاً
        if let Some(ref subj) = self.subject {
            if !Self::has_damma(subj) && !Self::is_pronoun(subj) {
                self.errors.push(format!(
                    "❌ خطأ نحوي: الفاعل '{}' يجب أن يكون مرفوعاً (بالضمة). التصحيح: {}ُ",
                    subj, subj
                ));
            }
        }

        // ٢. المفعول به يجب أن يكون منصوباً
        if let Some(ref obj) = self.object {
            if !Self::has_fatha(obj) && !Self::is_pronoun(obj) {
                self.errors.push(format!(
                    "❌ خطأ نحوي: المفعول به '{}' يجب أن يكون منصوباً (بالفتحة). التصحيح: {}َ",
                    obj, obj
                ));
            }
        }

        // ٣. حرف الجر يحتاج إلى مجرور
        if self.preposition.is_some() && self.genitive.is_none() {
            self.errors.push(
                "❌ خطأ نحوي: حرف الجر يحتاج إلى اسم مجرور بعده.".to_string()
            );
        }

        // ٤. المجرور يجب أن يكون مجروراً
        if let Some(ref gen) = self.genitive {
            if !Self::has_kasra(gen) && !Self::is_pronoun(gen) {
                self.errors.push(format!(
                    "❌ خطأ نحوي: المجرور '{}' يجب أن يكون مجروراً (بالكسرة). التصحيح: {}ِ",
                    gen, gen
                ));
            }
        }

        // ٥. الفعل المتعدي يحتاج مفعولاً به
        if let Some(ref verb) = self.verb {
            if Self::is_transitive(&verb.jidhr) && self.object.is_none() {
                self.errors.push(format!(
                    "❌ خطأ نحوي: الفعل '{}' (جذر: {}) متعدٍ ويحتاج إلى مفعول به.",
                    verb.original, verb.jidhr
                ));
            }
        }

        // ٦. التوافق في التذكير والتأنيث
        if let Some(ref verb) = self.verb {
            if let Some(ref subj) = self.subject {
                let verb_feminine = verb.original.contains('ت') && verb.original.ends_with('ت');
                let subject_feminine = subj.ends_with('ة') || subj.ends_with('ى');
                if verb_feminine && !subject_feminine && !Self::is_pronoun(subj) {
                    self.errors.push(format!(
                        "⚠️ تحذير: الفعل '{}' مؤنث لكن الفاعل '{}' مذكر. هل تقصد {}َ؟",
                        verb.original, subj,
                        verb.original.trim_end_matches('ت')
                    ));
                }
            }
        }

        // ٧. الفعل اللازم لا يأخذ مفعولاً به
        if let Some(ref verb) = self.verb {
            if Self::is_intransitive(&verb.jidhr) && self.object.is_some() {
                self.errors.push(format!(
                    "❌ خطأ نحوي: الفعل '{}' (جذر: {}) لازم ولا يحتاج إلى مفعول به.",
                    verb.original, verb.jidhr
                ));
            }
        }
    }

    /// هل الكلمة تنتهي بضمة (مرفوع)؟
    fn has_damma(word: &str) -> bool {
        word.ends_with('ُ') || word.ends_with("ٌ")
    }

    /// هل الكلمة تنتهي بفتحة (منصوب)؟
    fn has_fatha(word: &str) -> bool {
        word.ends_with('َ') || word.ends_with("ً")
    }

    /// هل الكلمة تنتهي بكسرة (مجرور)؟
    fn has_kasra(word: &str) -> bool {
        word.ends_with('ِ') || word.ends_with("ٍ")
    }

    /// هل الكلمة ضمير؟ (لا يحتاج إعراباً ظاهراً)
    fn is_pronoun(word: &str) -> bool {
        matches!(word, "هو" | "هي" | "أنا" | "أنت" | "أنتِ" | "نحن" | "هم" | "هن")
    }

    /// هل الجذر متعدٍ؟
    fn is_transitive(jidhr: &str) -> bool {
        matches!(jidhr, "قرأ" | "كتب" | "حسب" | "بعث" | "جمع" | "رسم" | "حفظ" | "فتح" | "نشر")
    }

    /// هل الجذر لازم؟
    fn is_intransitive(jidhr: &str) -> bool {
        matches!(jidhr, "نام" | "قام" | "جلس" | "مشى" | "وقف" | "فرح")
    }

    /// عرض الجملة مع الأخطاء
    pub fn display(&self) -> String {
        let mut result = String::new();

        if let Some(ref verb) = self.verb {
            result.push_str(&format!("🔍 فعل: {} (جذر:{}, وزن:{})\n", verb.original, verb.jidhr, verb.wazn));
        }
        if let Some(ref subj) = self.subject {
            result.push_str(&format!("👤 فاعل: {}\n", subj));
        }
        if let Some(ref obj) = self.object {
            result.push_str(&format!("📦 مفعول به: {}\n", obj));
        }
        if let (Some(ref prep), Some(ref gen)) = (&self.preposition, &self.genitive) {
            result.push_str(&format!("📍 جار ومجرور: {} {}\n", prep, gen));
        }

        if !self.errors.is_empty() {
            result.push_str("\n╔══════════════════════════╗\n");
            result.push_str("║   🛑 تقرير الأخطاء النحوية   ║\n");
            result.push_str("╚══════════════════════════╝\n");
            for (i, error) in self.errors.iter().enumerate() {
                result.push_str(&format!("{}. {}\n", i + 1, error));
            }
        } else {
            result.push_str("\n✅ الجملة صحيحة نحوياً\n");
        }

        result
    }

    pub fn execute(&self) -> String {
        self.display()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_sentence() {
        let s = SentenceParser::parse("قَرَأَ محمدُ الكتابَ").unwrap();
        assert!(s.errors.is_empty());
    }

    #[test]
    fn test_wrong_subject_case() {
        let s = SentenceParser::parse("قَرَأَ محمدَ الكتابَ").unwrap();
        assert!(!s.errors.is_empty());
        assert!(s.errors[0].contains("الفاعل"));
    }

    #[test]
    fn test_wrong_object_case() {
        let s = SentenceParser::parse("قَرَأَ محمدُ الكتابُ").unwrap();
        assert!(!s.errors.is_empty());
        assert!(s.errors.iter().any(|e| e.contains("المفعول به")));
    }

    #[test]
    fn test_transitive_without_object() {
        let s = SentenceParser::parse("قَرَأَ محمدُ").unwrap();
        assert!(!s.errors.is_empty());
        assert!(s.errors.iter().any(|e| e.contains("متعدٍ")));
    }

    #[test]
    fn test_preposition_without_genitive() {
        let s = SentenceParser::parse("قَرَأَ محمدُ في").unwrap();
        assert!(!s.errors.is_empty());
        assert!(s.errors.iter().any(|e| e.contains("حرف الجر")));
    }

    #[test]
    fn test_feminine_verb_masculine_subject() {
        let s = SentenceParser::parse("قَرَأَتْ محمدُ الكتابَ").unwrap();
        assert!(!s.errors.is_empty());
        assert!(s.errors.iter().any(|e| e.contains("مؤنث")));
    }
}
