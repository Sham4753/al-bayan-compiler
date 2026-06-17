use crate::composer::{Composer, ComposedSentence};
use crate::runtime::{BayanRuntime, Value};

/// منفذ الجمل - يحول الجملة العربية إلى عملية
pub struct Executor {
    runtime: BayanRuntime,
}

impl Executor {
    pub fn new() -> Self {
        Executor { runtime: BayanRuntime::new() }
    }

    /// تنفيذ جملة عربية
    pub fn execute_sentence(&mut self, text: &str) -> Result<Value, String> {
        let sentence = Composer::compose(text);
        
        // استخرج الجذور
        let verb_root = sentence.verb.as_ref()
            .map(|v| self.extract_root(v))
            .unwrap_or_default();
        
        let subject_root = sentence.subject.as_ref()
            .map(|s| self.extract_root(s))
            .unwrap_or_default();
        
        let object_root = sentence.object.as_ref()
            .map(|o| self.extract_root(o))
            .unwrap_or_default();

        // ابنِ الأمر من الجذور
        let command = format!("bayan.{}.{}", verb_root, object_root);
        
        // نفذ
        let value = format!("{} ← {} ← {}", verb_root, subject_root, object_root);
        self.runtime.memory.insert(verb_root.clone(), Value::Text(value.clone()));
        
        Ok(Value::Text(format!("✅ {} | {} ← {} ← {}", 
            sentence.original, verb_root, subject_root, object_root)))
    }

    fn extract_root(&self, word: &str) -> String {
        let letters: Vec<char> = word.chars()
            .filter(|c| !matches!(*c as u32, 0x064B..=0x0652 | 0x0670))
            .collect();
        if letters.len() >= 4 {
            letters[1..].iter().take(3).collect()
        } else {
            letters.iter().take(3).collect()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_execute_shakara() {
        let mut e = Executor::new();
        let r = e.execute_sentence("شاكر يشكر الله").unwrap();
        match r {
            Value::Text(t) => assert!(t.contains("شكر")),
            _ => panic!(),
        }
    }

    #[test]
    fn test_execute_darasa() {
        let mut e = Executor::new();
        let r = e.execute_sentence("دارس يدرس العلم").unwrap();
        assert!(e.runtime.memory.contains_key("درس"));
    }
}
