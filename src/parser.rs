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

        let mut s = ArabicSentence {
            verb: None, subject: None, object: None,
            preposition: None, genitive: None,
            adverbs: vec![], original, errors: vec![],
        };

        let prepositions = ["في", "على", "من", "إلى", "عن", "بِ", "لِ", "كَ", "حتى"];
        let mut skip = false;

        for (i, w) in words.iter().enumerate() {
            if skip { skip = false; continue; }
            if i == 0 { if let Ok(a) = Musarrif::analyse(w) { s.verb = Some(a); continue; } }
            if prepositions.contains(w) {
                s.preposition = Some(w.to_string());
                if i + 1 < words.len() { s.genitive = Some(words[i+1].to_string()); skip = true; }
                continue;
            }
            if s.subject.is_none() && s.verb.is_some() { s.subject = Some(w.to_string()); continue; }
            if s.object.is_none() { s.object = Some(w.to_string()); continue; }
            s.adverbs.push(w.to_string());
        }
        s.validate();
        Ok(s)
    }
}

impl ArabicSentence {
    pub fn validate(&mut self) {
        self.errors.clear();

        if let Some(ref subj) = self.subject {
            if !Self::has_damma(subj) && !Self::is_pronoun(subj) && !Self::is_taqdeeri_marfu3(subj) {
                self.errors.push(format!("❌ الفاعل '{}' يجب أن يكون مرفوعاً", subj));
            }
        }
        if let Some(ref obj) = self.object {
            if !Self::has_fatha(obj) && !Self::is_pronoun(obj) && !Self::is_taqdeeri_mansub(obj) {
                self.errors.push(format!("❌ المفعول به '{}' يجب أن يكون منصوباً", obj));
            }
        }
        if self.preposition.is_some() && self.genitive.is_none() {
            self.errors.push("❌ حرف الجر يحتاج إلى اسم مجرور".to_string());
        }
        if let Some(ref gen) = self.genitive {
            if !Self::has_kasra(gen) && !Self::is_pronoun(gen) && !Self::is_taqdeeri_majrur(gen) {
                self.errors.push(format!("❌ المجرور '{}' يجب أن يكون مجروراً", gen));
            }
        }
        if let Some(ref verb) = self.verb {
            if Self::is_transitive(&verb.jidhr) && self.object.is_none() {
                self.errors.push(format!("❌ الفعل '{}' متعدٍ ويحتاج مفعولاً به", verb.original));
            }
        }
        if let Some(ref verb) = self.verb {
            if let Some(ref subj) = self.subject {
                let vf = verb.original.contains('ت');
                let sf = subj.ends_with('ة') || subj.ends_with('ى');
                if vf && !sf && !Self::is_pronoun(subj) {
                    self.errors.push(format!("⚠️ الفعل '{}' مؤنث لكن الفاعل '{}' مذكر", verb.original, subj));
                }
            }
        }
    }

    /// الأسماء التي ترفع تقديرياً (لا تظهر عليها الضمة)
    fn is_taqdeeri_marfu3(word: &str) -> bool {
        // مقصور: ينتهي بألف
        if word.ends_with('ى') || word.ends_with("َى") { return true; }
        // منقوص: ينتهي بياء (الضمة مقدرة)
        if word.ends_with("ِي") { return true; }
        // مبني: هذا، الذي، أنا...
        let mabni = ["هذا", "هذه", "الذي", "التي", "أنا", "أنت", "هو", "هي", "نحن", "هم", "ما", "مَن"];
        if mabni.contains(&word) { return true; }
        // الأسماء الخمسة
        let five = ["أبوك", "أخوك", "حموك", "فوك", "ذو"];
        for f in &five { if word.starts_with(f) { return true; } }
        false
    }

    fn is_taqdeeri_mansub(word: &str) -> bool {
        if word.ends_with('ى') || word.ends_with("َى") { return true; }
        if word.ends_with("ِيَ") || word.ends_with("ِي") { return true; }
        let mabni = ["هذا", "هذه", "الذي", "التي", "أنا", "أنت", "هو", "هي", "نحن", "هم"];
        if mabni.contains(&word) { return true; }
        let five = ["أباك", "أخاك", "حماك", "فاك", "ذا"];
        for f in &five { if word.starts_with(f) { return true; } }
        false
    }

    fn is_taqdeeri_majrur(word: &str) -> bool {
        if word.ends_with('ى') || word.ends_with("َى") { return true; }
        if word.ends_with("ِي") { return true; }
        let mabni = ["هذا", "هذه", "الذي", "التي", "أنا", "أنت", "هو", "هي", "نحن", "هم"];
        if mabni.contains(&word) { return true; }
        let five = ["أبيك", "أخيك", "حميك", "فيك", "ذي"];
        for f in &five { if word.starts_with(f) { return true; } }
        false
    }

    pub fn detect_taqdeeri(&self, word: &str) -> String {
        if word.ends_with('ى') || word.ends_with("َى") { return format!("📌 '{}': اسم مقصور - إعرابه تقديري", word); }
        if word.ends_with("ِي") { return format!("📌 '{}': اسم منقوص - إعرابه تقديري", word); }
        let mabni = ["هذا", "هذه", "الذي", "التي", "أنا", "أنت", "هو", "هي", "نحن", "هم"];
        if mabni.contains(&word) { return format!("📌 '{}': اسم مبني", word); }
        "".to_string()
    }

    pub fn full_irab(&self) -> String {
        let mut r = String::from("📋 التحليل الإعرابي:\n");
        if let Some(ref v) = self.verb { r.push_str(&format!("   🔍 '{}': فعل\n", v.original)); }
        if let Some(ref s) = self.subject {
            r.push_str(&format!("   👤 '{}': فاعل مرفوع", s));
            let t = self.detect_taqdeeri(s);
            if !t.is_empty() { r.push_str(&format!(" ({})", t.split(": ").last().unwrap_or(""))); }
            r.push_str("\n");
        }
        if let Some(ref o) = self.object {
            r.push_str(&format!("   📦 '{}': مفعول به منصوب", o));
            let t = self.detect_taqdeeri(o);
            if !t.is_empty() { r.push_str(&format!(" ({})", t.split(": ").last().unwrap_or(""))); }
            r.push_str("\n");
        }
        if let (Some(ref p), Some(ref g)) = (&self.preposition, &self.genitive) {
            r.push_str(&format!("   📍 '{}': مجرور بـ '{}'", g, p));
            let t = self.detect_taqdeeri(g);
            if !t.is_empty() { r.push_str(&format!(" ({})", t.split(": ").last().unwrap_or(""))); }
            r.push_str("\n");
        }
        if !self.errors.is_empty() {
            r.push_str("\n🛑 الأخطاء:\n");
            for e in &self.errors { r.push_str(&format!("   {}\n", e)); }
        } else { r.push_str("\n✅ صحيحة نحوياً\n"); }
        r
    }

    fn has_damma(w: &str) -> bool { w.ends_with('ُ') || w.ends_with("ٌ") }
    fn has_fatha(w: &str) -> bool { w.ends_with('َ') || w.ends_with("ً") }
    fn has_kasra(w: &str) -> bool { w.ends_with('ِ') || w.ends_with("ٍ") }
    fn is_pronoun(w: &str) -> bool { matches!(w, "هو"|"هي"|"أنا"|"أنت"|"أنتِ"|"نحن"|"هم") }
    fn is_transitive(j: &str) -> bool { matches!(j, "قرأ"|"كتب"|"حسب"|"بعث"|"جمع"|"رسم"|"حفظ"|"فتح"|"نشر") }

    pub fn display(&self) -> String { self.full_irab() }
    pub fn execute(&self) -> String { self.full_irab() }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn test_valid() { assert!(SentenceParser::parse("قَرَأَ محمدُ الكتابَ").unwrap().errors.is_empty()); }
    #[test] fn test_subject() { assert!(!SentenceParser::parse("قَرَأَ محمدَ الكتابَ").unwrap().errors.is_empty()); }
    #[test] fn test_object() { assert!(!SentenceParser::parse("قَرَأَ محمدُ الكتابُ").unwrap().errors.is_empty()); }
    #[test] fn test_transitive() { assert!(!SentenceParser::parse("قَرَأَ محمدُ").unwrap().errors.is_empty()); }
    #[test] fn test_maqsour_subject() { assert!(SentenceParser::parse("قَرَأَ الفتى الكتابَ").unwrap().errors.is_empty()); }
    #[test] fn test_mabni_subject() { assert!(SentenceParser::parse("قَرَأَ هذا الكتابَ").unwrap().errors.is_empty()); }
    #[test] fn test_manqous_subject() { assert!(SentenceParser::parse("جاءَ القاضي").unwrap().errors.is_empty()); }
}
