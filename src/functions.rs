use crate::variables::Variables;
use crate::runtime::Value;
use std::collections::HashMap;

/// نظام الدوال للبيان
pub struct Functions {
    variables: Variables,
    /// الدوال المخزنة: اسم الدالة → جسم الدالة
    functions: HashMap<String, String>,
}

impl Functions {
    pub fn new() -> Self {
        Functions { variables: Variables::new(), functions: HashMap::new() }
    }

    /// تعريف دالة: دالة <اسم>(<معامل>) = <جسم>
    pub fn define(&mut self, text: &str) -> Result<String, String> {
        let text = text.trim().trim_start_matches("دالة").trim();
        
        // استخرج الاسم
        let name_end = text.find('(').ok_or("❌ استخدم: دالة <اسم>(<معامل>) = <جسم>")?;
        let name = text[..name_end].trim().to_string();
        
        // استخرج المعامل
        let params_end = text.find(')').ok_or("❌ تحتاج قوس إغلاق")?;
        let params = text[name_end+1..params_end].trim().to_string();
        
        // استخرج الجسم
        let body_start = text.find('=').ok_or("❌ تحتاج = بعد المعاملات")?;
        let body = text[body_start+1..].trim().to_string();
        
        self.functions.insert(name.clone(), body.clone());
        Ok(format!("✅ دالة {} ({}) = {}", name, params, body))
    }

    /// استدعاء دالة
    pub fn call(&mut self, text: &str) -> Result<Value, String> {
        let text = text.trim();
        
        let paren_start = text.find('(').ok_or("❌ استخدم: <اسم>(<قيمة>)")?;
        let name = text[..paren_start].trim().to_string();
        
        let paren_end = text.find(')').ok_or("❌ تحتاج قوس إغلاق")?;
        let arg = text[paren_start+1..paren_end].trim().to_string();
        
        let body = self.functions.get(&name)
            .ok_or(format!("❌ دالة غير معروفة: {}", name))?;
        
        // استبدل المعامل في الجسم
        let executed = body.replace("معامل", &arg);
        
        // خزّن النتيجة
        self.variables.set(&format!("اجعل نتيجة_{} في {}", name, executed))?;
        
        Ok(Value::Text(format!("✅ {} → {}", name, executed)))
    }

    /// عرض كل الدوال
    pub fn list(&self) -> String {
        let mut r = String::from("📦 الدوال:\n");
        for (name, body) in &self.functions {
            r.push_str(&format!("   {} = {}\n", name, body));
        }
        r
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_define() {
        let mut f = Functions::new();
        let r = f.define("دالة شكر(شخص) = احفظ شكر شخص في السجل").unwrap();
        assert!(r.contains("شكر"));
    }

    #[test]
    fn test_call() {
        let mut f = Functions::new();
        f.define("دالة شكر(شخص) = احفظ شكر معامل في السجل").unwrap();
        let r = f.call("شكر(الله)").unwrap();
        match r {
            Value::Text(t) => assert!(t.contains("الله")),
            _ => panic!(),
        }
    }
}
