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
            // ========== القراءة ==========
            "bayan.io.read_sync" => {
                // محاكاة: نعيد نصاً وهمياً
                Ok(Value::Text("محتوى الملف التجريبي".to_string()))
            }
            "bayan.io.read_async" => {
                // محاكاة غير متزامنة: نعيد نصاً
                Ok(Value::Text("محتوى من Stream".to_string()))
            }
            "bayan.net.http_get" => {
                Ok(Value::Text("{\"result\": \"استجابة API\"}".to_string()))
            }

            // ========== الكتابة ==========
            "bayan.io.write_sync" => {
                if let Some(Value::Text(t)) = input {
                    self.memory.insert("آخر_كتابة".to_string(), Value::Text(t));
                }
                Ok(Value::Nothing)
            }

            // ========== الحساب ==========
            "bayan.compute.sync" => {
                if let Some(Value::List(list)) = input {
                    let sum: f64 = list.iter().filter_map(|v| {
                        if let Value::Number(n) = v { Some(*n) } else { None }
                    }).sum();
                    Ok(Value::Number(sum))
                } else {
                    Ok(Value::Number(42.0)) // قيمة افتراضية
                }
            }
            "bayan.compute.parallel_map" => {
                // تنفيذ متوازي حقيقي باستخدام threads
                if let Some(Value::List(list)) = input {
                    let results = Arc::new(Mutex::new(Vec::new()));
                    let mut handles = vec![];

                    for item in list {
                        let results = Arc::clone(&results);
                        handles.push(thread::spawn(move || {
                            // معالجة متوازية: نضاعف الرقم
                            let processed = match item {
                                Value::Number(n) => Value::Number(n * 2.0),
                                other => other,
                            };
                            results.lock().unwrap().push(processed);
                        }));
                    }

                    for handle in handles {
                        handle.join().unwrap();
                    }

                    let final_results = results.lock().unwrap().clone();
                    Ok(Value::List(final_results))
                } else {
                    Err("parallel_map يحتاج إلى قائمة".to_string())
                }
            }
            "bayan.compute.async" => {
                // محاكاة غير متزامنة
                Ok(Value::Number(100.0))
            }

            // ========== الشبكة ==========
            "bayan.net.send_sync" => {
                self.memory.insert("حالة_الإرسال".to_string(), Value::Text("تم".to_string()));
                Ok(Value::Text("تم الإرسال".to_string()))
            }
            "bayan.net.send_async" => {
                Ok(Value::Text("إرسال غير متزامن مجدول".to_string()))
            }
            "bayan.net.listen" => {
                // يفتح مستمعاً (محاكاة)
                self.memory.insert("مستمع_نشط".to_string(), Value::Text("يعمل".to_string()));
                Ok(Value::Text("المستمع يعمل على المنفذ 8080".to_string()))
            }

            // ========== التخزين ==========
            "bayan.memory.store" => {
                if let Some(val) = input {
                    self.memory.insert("مخزن".to_string(), val);
                }
                Ok(Value::Nothing)
            }

            // ========== الأمان ==========
            "bayan.security.encrypt" => {
                if let Some(Value::Text(t)) = input {
                    Ok(Value::Text(format!("مُشفر[{}]", t)))
                } else {
                    Ok(Value::Text("مُشفر[??]".to_string()))
                }
            }

            // ========== النظام ==========
            "bayan.system.profile" => {
                Ok(Value::Text("حالة النظام: الذاكرة 64MB، المعالج 2.1GHz".to_string()))
            }

            // ========== الاستعلام ==========
            "bayan.data.query" => {
                Ok(Value::List(vec![
                    Value::Text("نتيجة1".to_string()),
                    Value::Text("نتيجة2".to_string()),
                ]))
            }

            // ========== الواجهة ==========
            "bayan.data.search" => {
                Ok(Value::Text("نتائج البحث".to_string()))
            }
            "bayan.io.open" => {
                Ok(Value::Text("تم فتح الاتصال".to_string()))
            }
            "bayan.ui.render" => {
                Ok(Value::Text("<div>مرحباً بالعالم</div>".to_string()))
            }

            // ========== التجميع ==========
            "bayan.collection.create" => {
                if let Some(val) = input {
                    Ok(Value::List(vec![val]))
                } else {
                    Ok(Value::List(vec![]))
                }
            }

            // ========== المنطق ==========
            "bayan.control.if_else" => {
                Ok(Value::Nothing)
            }
            "bayan.io.print" => Ok(Value::Text("تمت الطباعة".to_string())),
            "bayan.data.search" => Ok(Value::Text("نتائج البحث".to_string())),
            "bayan.data.delete" => Ok(Value::Text("تم الحذف".to_string())),
            "bayan.io.copy" => Ok(Value::Text("تم النسخ".to_string())),
            "bayan.io.paste" => Ok(Value::Text("تم اللصق".to_string())),
            "bayan.ai.translate" => Ok(Value::Text("تمت الترجمة".to_string())),
            "bayan.exec.run" => Ok(Value::Text("تم التشغيل".to_string())),
            "bayan.security.lock" => Ok(Value::Text("تم التأمين".to_string())),
            "bayan.data.count" => Ok(Value::Number(42.0)),
            "bayan.data.sort" => Ok(Value::Text("تم الترتيب".to_string())),

            "bayan.arabic.sabr" => Ok(Value::Text("صَبَرَ: صبر".to_string())),
            "bayan.arabic.غفر" => Ok(Value::Text("غفر: فعل".to_string())),
            "bayan.arabic.رحم" => Ok(Value::Text("رحم: فعل".to_string())),
            "bayan.arabic.دخل" => Ok(Value::Text("دخل: فعل".to_string())),
            "bayan.arabic.خرج" => Ok(Value::Text("خرج: فعل".to_string())),
            "bayan.arabic.سأل" => Ok(Value::Text("سأل: فعل".to_string())),
            "bayan.arabic.جلس" => Ok(Value::Text("جلس: فعل".to_string())),
            "bayan.arabic.قام" => Ok(Value::Text("قام: فعل".to_string())),
            "bayan.arabic.نام" => Ok(Value::Text("نام: فعل".to_string())),
            "bayan.arabic.ذهب" => Ok(Value::Text("ذهب: فعل".to_string())),
            "bayan.arabic.رجع" => Ok(Value::Text("رجع: فعل".to_string())),
            "bayan.arabic.سكن" => Ok(Value::Text("سكن: فعل".to_string())),
            "bayan.arabic.حمل" => Ok(Value::Text("حمل: فعل".to_string())),
            "bayan.arabic.عمل" => Ok(Value::Text("عمل: فعل".to_string())),
            "bayan.arabic.درس" => Ok(Value::Text("درس: فعل".to_string())),
            "bayan.arabic.فهم" => Ok(Value::Text("فهم: فعل".to_string())),
            "bayan.arabic.حكم" => Ok(Value::Text("حكم: فعل".to_string())),
            "bayan.arabic.ملك" => Ok(Value::Text("ملك: فعل".to_string())),
            "bayan.arabic.سلم" => Ok(Value::Text("سلم: فعل".to_string())),
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
