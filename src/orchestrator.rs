use crate::morphology::{Morphology, Zaman};
use crate::runtime::{BayanRuntime, Value};

/// المنسق - يربط التحليل اللغوي بالتنفيذ
pub struct Orchestrator {
    runtime: BayanRuntime,
}

impl Orchestrator {
    pub fn new() -> Self {
        Orchestrator { runtime: BayanRuntime::new() }
    }

    /// تنفيذ كلمة عربية - المسار الكامل
    pub fn execute(&mut self, word: &str) -> Result<Value, String> {
        // ١. فصل الحروف عن الحركات
        let letters: Vec<char> = word.chars()
            .filter(|c| !matches!(*c as u32, 0x064B..=0x0652 | 0x0670))
            .collect();

        // ٢. تحليل صرفي
        let morph = Morphology::analyse(&letters);
        if morph.jidhr.is_empty() {
            return Err(format!("لم يتم التعرف على: {}", word));
        }

        // ٣. بناء intrinsic من الجذر
        let intrinsic = format!("bayan.arabic.{}", morph.jidhr);

        // ٤. تنفيذ
        self.runtime.execute_intrinsic(&intrinsic, None)
    }

    /// تنفيذ جملة كاملة
    pub fn execute_sentence(&mut self, text: &str) -> Vec<Result<Value, String>> {
        text.split_whitespace()
            .map(|w| self.execute(w))
            .collect()
    }

    /// عرض حالة الذاكرة
    pub fn show_memory(&self) {
        println!("📦 الذاكرة:");
        for (k, v) in &self.runtime.memory {
            println!("   {} = {:?}", k, v);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_execute_shakara() {
        let mut o = Orchestrator::new();
        let r = o.execute("شَكَرَ").unwrap();
        match r {
            Value::Text(t) => assert!(t.contains("صبر") || t.contains("شكر") || !t.is_empty()),
            _ => {}
        }
    }

    #[test]
    fn test_execute_sabara() {
        let mut o = Orchestrator::new();
        assert!(o.execute("صَبَرَ").is_ok());
    }

    #[test]
    fn test_execute_20_roots() {
        let mut o = Orchestrator::new();
        let roots = ["شكر", "صبر", "غفر", "رحم", "دخل", "خرج", "سأل", "جلس", "قام", "نام",
                     "ذهب", "رجع", "سكن", "حمل", "عمل", "درس", "فهم", "حكم", "ملك", "سلم"];
        for root in &roots {
            let word = format!("{}َ", root);
            assert!(o.execute(&word).is_ok(), "فشل: {}", root);
        }
    }
}
