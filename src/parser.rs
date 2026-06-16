use crate::musarrif::{Musarrif, Musarrafa, Zaman, Damir, Irab};

#[derive(Debug, Clone)]
pub struct ArabicSentence {
    pub verb: Option<Musarrafa>,
    pub subject: Option<String>,
    pub object: Option<String>,
    pub preposition: Option<String>,
    pub genitive: Option<String>,
    pub adverbs: Vec<String>,
    pub original: String,
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
            adverbs: vec![], original,
        };

        let prepositions = ["في", "على", "من", "إلى", "عن", "بِ", "لِ", "كَ", "حتى"];

        for (i, word) in words.iter().enumerate() {
            if i == 0 {
                if let Ok(a) = Musarrif::analyse(word) {
                    sentence.verb = Some(a);
                    continue;
                }
            }
            if prepositions.contains(word) {
                sentence.preposition = Some(word.to_string());
                continue;
            }
            if sentence.preposition.is_some() && sentence.genitive.is_none() {
                sentence.genitive = Some(word.to_string());
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
        Ok(sentence)
    }
}

impl ArabicSentence {
    pub fn execute(&self) -> String {
        let mut r = String::new();
        if let Some(ref v) = self.verb {
            r.push_str(&format!("🔍 فعل: {} (جذر:{}, وزن:{})\n", v.original, v.jidhr, v.wazn));
        }
        if let Some(ref s) = self.subject { r.push_str(&format!("👤 فاعل: {}\n", s)); }
        if let Some(ref o) = self.object { r.push_str(&format!("📦 مفعول به: {}\n", o)); }
        if let (Some(ref p), Some(ref g)) = (&self.preposition, &self.genitive) {
            r.push_str(&format!("📍 جار ومجرور: {} {}\n", p, g));
        }
        r
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple() {
        let s = SentenceParser::parse("قَرَأَ محمد الكتاب").unwrap();
        assert!(s.verb.is_some());
        assert_eq!(s.subject, Some("محمد".to_string()));
    }

    #[test]
    fn test_preposition() {
        let s = SentenceParser::parse("قَرَأَ محمد في المكتبة").unwrap();
        assert_eq!(s.preposition, Some("في".to_string()));
        assert_eq!(s.genitive, Some("المكتبة".to_string()));
    }
}
