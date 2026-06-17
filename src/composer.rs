use crate::morphology::Morphology;

#[derive(Debug)]
pub struct ComposedSentence {
    pub original: String,
    pub subject: Option<String>,
    pub verb: Option<String>,
    pub object: Option<String>,
    pub subject_form: Option<String>,
    pub verb_form: Option<String>,
}

pub struct Composer;

impl Composer {
    pub fn compose(text: &str) -> ComposedSentence {
        let words: Vec<&str> = text.split_whitespace().collect();
        let mut subject = None;
        let mut verb = None;
        let mut object = None;
        let mut subject_form = None;
        let mut verb_form = None;

        for (i, word) in words.iter().enumerate() {
            let letters: Vec<char> = word.chars()
                .filter(|c| !matches!(*c as u32, 0x064B..=0x0652 | 0x0670))
                .collect();
            let jidhr: String = letters.iter().collect();
            let forms = Morphology::derive(&jidhr);

            match i {
                0 => {
                    subject = Some(word.to_string());
                    if forms.len() >= 4 { subject_form = Some(forms[3].clone()); }
                }
                1 => {
                    verb = Some(word.to_string());
                    if forms.len() >= 2 { verb_form = Some(forms[1].clone()); }
                }
                2 => { object = Some(word.to_string()); }
                _ => {}
            }
        }

        ComposedSentence { original: text.to_string(), subject, verb, object, subject_form, verb_form }
    }
}

impl ComposedSentence {
    pub fn execute(&self) -> String {
        let mut r = format!("📝 الجملة: {}\n", self.original);
        if let Some(ref s) = self.subject {
            r.push_str(&format!("   👤 الفاعل: {}", s));
            if let Some(ref sf) = self.subject_form { r.push_str(&format!(" (اسم فاعل: {})", sf)); }
            r.push_str("\n");
        }
        if let Some(ref v) = self.verb {
            r.push_str(&format!("   🔍 الفعل: {}", v));
            if let Some(ref vf) = self.verb_form { r.push_str(&format!(" (مضارع: {})", vf)); }
            r.push_str("\n");
        }
        if let Some(ref o) = self.object { r.push_str(&format!("   📦 المفعول به: {}\n", o)); }
        r
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shakara_sentence() {
        let s = Composer::compose("شاكر يشكر الله");
        assert_eq!(s.subject, Some("شاكر".to_string()));
        assert_eq!(s.verb, Some("يشكر".to_string()));
        assert_eq!(s.object, Some("الله".to_string()));
    }

    #[test]
    fn test_darasa_sentence() {
        let s = Composer::compose("دارس يدرس العلم");
        assert!(s.subject_form.is_some());
        assert!(s.verb_form.is_some());
    }
}
