use crate::composer::{Composer, ComposedSentence};
use crate::executor::Executor;
use crate::runtime::Value;

/// نتيجة تنفيذ المنطق
#[derive(Debug)]
pub enum FlowResult {
    Success(String),
    Failure(String),
    Skipped(String),
}

/// محرك المنطق - إذا... فـ... وإلا
pub struct ControlFlow {
    executor: Executor,
}

impl ControlFlow {
    pub fn new() -> Self {
        ControlFlow { executor: Executor::new() }
    }

    /// تنفيذ جملة شرطية
    pub fn execute_if(&mut self, text: &str) -> FlowResult {
        let parts: Vec<&str> = text.splitn(2, "فـ").collect();
        if parts.len() < 2 {
            return FlowResult::Failure("❌ استخدم: إذا <شرط> فـ <نتيجة>".to_string());
        }

        let condition = parts[0].trim().trim_start_matches("إذا").trim();
        let action = parts[1].trim();

        // نفذ الشرط
        if self.check_condition(condition) {
            match self.executor.execute_sentence(action) {
                Ok(_) => FlowResult::Success("✅ تحقق الشرط".to_string()),
                Err(e) => FlowResult::Failure(format!("❌ {}", e)),
            }
        } else {
            FlowResult::Skipped(format!("⏭️ لم يتحقق: {}", condition))
        }
    }

    /// تحقق من شرط بسيط
    fn check_condition(&self, text: &str) -> bool {
        let sentence = Composer::compose(text);
        // إذا الجملة فيها فعل وفاعل ومفعول، الشرط تحقق
        sentence.subject.is_some() && sentence.verb.is_some() && sentence.object.is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_if_then() {
        let mut ctrl = ControlFlow::new();
        let r = ctrl.execute_if("إذا شكر شاكر الله فـ احفظ النتيجة");
        match r {
            FlowResult::Success(s) => assert!(s.contains("✅")),
            _ => panic!("يجب أن ينجح"),
        }
    }

    #[test]
    fn test_if_false() {
        let mut ctrl = ControlFlow::new();
        let r = ctrl.execute_if("إذا فـ احفظ"); // جملة ناقصة
        match r {
            FlowResult::Skipped(_) => {}, // نجاح
            _ => panic!("يجب أن يتخطى"),
        }
    }
}
