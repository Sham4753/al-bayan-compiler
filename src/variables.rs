use crate::runtime::{BayanRuntime, Value};
use std::collections::HashMap;

/// نظام المتغيرات للبيان
pub struct Variables {
    runtime: BayanRuntime,
}

impl Variables {
    pub fn new() -> Self {
        Variables { runtime: BayanRuntime::new() }
    }

    /// تنفيذ أمر "اجعل س = قيمة"
    pub fn set(&mut self, text: &str) -> Result<Value, String> {
        let text = text.trim().trim_start_matches("اجعل").trim();
        
        // "س في قيمة"
        if let Some(pos) = text.find("في") {
            let var = text[..pos].trim().to_string();
            let val = text[pos+2..].trim().to_string();
            
            self.runtime.memory.insert(var.clone(), Value::Text(val.clone()));
            return Ok(Value::Text(format!("✅ {} = {}", var, val)));
        }
        
        // "س = قيمة"
        if let Some(pos) = text.find('=') {
            let var = text[..pos].trim().to_string();
            let val = text[pos+1..].trim().to_string();
            
            self.runtime.memory.insert(var.clone(), Value::Text(val.clone()));
            return Ok(Value::Text(format!("✅ {} = {}", var, val)));
        }

        Err("❌ استخدم: اجعل <اسم> في <قيمة> أو اجعل <اسم> = <قيمة>".to_string())
    }

    /// استرجاع قيمة متغير
    pub fn get(&self, name: &str) -> Option<String> {
        self.runtime.memory.get(name).and_then(|v| match v {
            Value::Text(t) => Some(t.clone()),
            _ => None,
        })
    }

    /// عرض كل المتغيرات
    pub fn list(&self) -> String {
        let mut r = String::from("📦 المتغيرات:\n");
        for (k, v) in &self.runtime.memory {
            r.push_str(&format!("   {} = {:?}\n", k, v));
        }
        r
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_set_fi() {
        let mut vars = Variables::new();
        let r = vars.set("اجعل نتيجة في شكر الله").unwrap();
        assert!(vars.get("نتيجة").is_some());
    }

    #[test]
    fn test_set_equals() {
        let mut vars = Variables::new();
        vars.set("اجعل س = ١٠").unwrap();
        assert_eq!(vars.get("س"), Some("١٠".to_string()));
    }

    #[test]
    fn test_list() {
        let mut vars = Variables::new();
        vars.set("اجعل أ في ١").unwrap();
        vars.set("اجعل ب في ٢").unwrap();
        let list = vars.list();
        assert!(list.contains("أ"));
        assert!(list.contains("ب"));
    }
}
