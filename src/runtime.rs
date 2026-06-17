use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::thread;

/// قيمة في الذاكرة: قد تكون نصاً، رقماً، أو مصفوفة
#[derive(Debug, Clone)]
pub enum Value {
    Text(String),
    Number(f64),
    List(Vec<Value>),
    Nothing,
}

/// حالة البرنامج أثناء التنفيذ
pub struct BayanRuntime {
    /// الذاكرة: تخزين المتغيرات (الضمائر والجذور)
    pub memory: HashMap<String, Value>,
    /// سجل العمليات المنفذة (للتتبع)
    pub log: Vec<String>,
}

impl BayanRuntime {
    pub fn new() -> Self {
        BayanRuntime {
            memory: HashMap::new(),
            log: Vec::new(),
        }
    }

    /// تنفيذ intrinsic معين مع مُدخلات
    pub fn execute_intrinsic(&mut self, intrinsic: &str, input: Option<Value>) -> Result<Value, String> {
        self.log.push(format!("تنفيذ: {} | المُدخل: {:?}", intrinsic, input));

        match intrinsic {
            _ => Err(format!("intrinsic غير معروف: {}", intrinsic)),
        }
    }

    /// تشغيل دورة حياة البرنامج
    pub fn run(&mut self, instructions: Vec<(String, Option<Value>)>) -> Result<Value, String> {
        let mut last_result = Value::Nothing;

        for (intrinsic, input) in instructions {
            last_result = self.execute_intrinsic(&intrinsic, input)?;
            println!("  ↳ النتيجة: {:?}", last_result);
        }

        println!("\n📋 سجل التنفيذ:");
        for entry in &self.log {
            println!("  • {}", entry);
        }

        Ok(last_result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_sync() {
        let mut rt = BayanRuntime::new();
        let result = rt.execute_intrinsic("bayan.io.read_sync", None).unwrap();
        match result {
            Value::Text(t) => assert!(!t.is_empty()),
            _ => panic!("يجب أن يعيد نصاً"),
        }
    }

    #[test]
    fn test_compute_sync() {
        let mut rt = BayanRuntime::new();
        let input = Value::List(vec![Value::Number(10.0), Value::Number(20.0)]);
        let result = rt.execute_intrinsic("bayan.compute.sync", Some(input)).unwrap();
        match result {
            Value::Number(n) => assert_eq!(n, 30.0),
            _ => panic!("يجب أن يعيد رقماً"),
        }
    }

    #[test]
    fn test_parallel_map() {
        let mut rt = BayanRuntime::new();
        let input = Value::List(vec![Value::Number(1.0), Value::Number(2.0), Value::Number(3.0)]);
        let result = rt.execute_intrinsic("bayan.compute.parallel_map", Some(input)).unwrap();
        match result {
            Value::List(list) => {
                assert_eq!(list.len(), 3);
                // كل عنصر يجب أن يكون مضاعفاً
            }
            _ => panic!("يجب أن يعيد قائمة"),
        }
    }

    #[test]
    fn test_encrypt() {
        let mut rt = BayanRuntime::new();
        let input = Value::Text("بيانات سرية".to_string());
        let result = rt.execute_intrinsic("bayan.security.encrypt", Some(input)).unwrap();
        match result {
            Value::Text(t) => assert!(t.contains("مُشفر")),
            _ => panic!("يجب أن يعيد نصاً مشفراً"),
        }
    }

    #[test]
    fn test_listen() {
        let mut rt = BayanRuntime::new();
        let result = rt.execute_intrinsic("bayan.net.listen", None).unwrap();
        match result {
            Value::Text(t) => assert!(t.contains("8080")),
            _ => panic!("يجب أن يعيد نصاً"),
        }
    }

    #[test]
    fn test_profile() {
        let mut rt = BayanRuntime::new();
        let result = rt.execute_intrinsic("bayan.system.profile", None).unwrap();
        match result {
            Value::Text(t) => assert!(t.contains("النظام")),
            _ => panic!("يجب أن يعيد نصاً"),
        }
    }
}
