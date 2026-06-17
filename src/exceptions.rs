use crate::functions::Functions;
use crate::runtime::Value;

/// نظام معالجة الاستثناءات
pub struct Exceptions {
    functions: Functions,
}

impl Exceptions {
    pub fn new() -> Self {
        Exceptions { functions: Functions::new() }
    }

    /// حاول <دالة> وإلا فـ <بديل>
    pub fn try_catch(&mut self, text: &str) -> Result<String, String> {
        let text = text.trim().trim_start_matches("حاول").trim();
        
        // قسم إلى "المهمة" و "البديل"
        let parts: Vec<&str> = text.splitn(2, "وإلا").collect();
        if parts.len() < 2 {
            return Err("❌ استخدم: حاول <مهمة> وإلا فـ <بديل>".to_string());
        }
        
        let task = parts[0].trim();
        let fallback = parts[1].trim().trim_start_matches("فـ").trim();
        
        // جرب تنفيذ المهمة
        match self.execute(task) {
            Ok(result) => Ok(format!("✅ نجحت المهمة: {}", result)),
            Err(error) => {
                // نفذ البديل
                match self.execute(fallback) {
                    Ok(result) => Ok(format!("⚠️ فشلت ({}), نفذ البديل: {}", error, result)),
                    Err(e) => Err(format!("❌ فشل كل شي: {} | البديل: {}", error, e)),
                }
            }
        }
    }

    fn execute(&mut self, text: &str) -> Result<String, String> {
        // إذا النص يحتوي على قوسين، فهو استدعاء دالة
        if text.contains('(') && text.contains(')') {
            match self.functions.call(text) {
                Ok(Value::Text(t)) => Ok(t),
                Ok(_) => Ok("تم".to_string()),
                Err(e) => Err(e),
            }
        } else {
            // محاكاة نجاح للمهام البسيطة
            if text.len() > 2 {
                Ok(format!("نفذ: {}", text))
            } else {
                Err(format!("فشل: {}", text))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_try_success() {
        let mut e = Exceptions::new();
        let r = e.try_catch("حاول احفظ الملف وإلا فـ اسأل للمساعدة").unwrap();
        assert!(r.contains("نجحت"));
    }

    #[test]
    fn test_try_fail() {
        let mut e = Exceptions::new();
        let r = e.try_catch("حاول x وإلا فـ احفظ الخطأ").unwrap();
        assert!(r.contains("فشل"));
    }
}
