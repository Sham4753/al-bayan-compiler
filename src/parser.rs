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
