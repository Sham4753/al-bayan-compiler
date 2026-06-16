use crate::musarrif::Musarrif;
use crate::generator::Generator;
use crate::runtime::{BayanRuntime, Value};

/// محرك البيان المستقل - يقرأ كود البيان وينفذه مباشرة
pub struct BayanEngine {
    generator: Generator,
    runtime: BayanRuntime,
}

impl BayanEngine {
    pub fn new() -> Self {
        BayanEngine {
            generator: Generator::new(),
            runtime: BayanRuntime::new(),
        }
    }

    /// تنفيذ ملف .بيان كامل
    pub fn execute_file(&mut self, path: &str) -> Result<(), String> {
        let content = std::fs::read_to_string(path)
            .map_err(|e| format!("فشل قراءة الملف '{}': {}", path, e))?;

        println!("🕌 تنفيذ ملف البيان: {}\n", path);

        for (line_num, line) in content.lines().enumerate() {
            let line = line.trim();

            // تخطي الأسطر الفارغة والتعليقات
            if line.is_empty() || line.starts_with("//") || line.starts_with('#') {
                continue;
            }

            println!("📜 سطر {}: '{}'", line_num + 1, line);

            match self.execute_line(line) {
                Ok(result) => {
                    if !matches!(result, Value::Nothing) {
                        println!("   ↳ {:?}\n", result);
                    }
                }
                Err(e) => {
                    eprintln!("❌ خطأ في السطر {}: {}\n", line_num + 1, e);
                    return Err(e);
                }
            }
        }

        println!("✅ تم التنفيذ بنجاح");
        Ok(())
    }

    /// تنفيذ سطر واحد من كود البيان
    pub // تجاهل البسملة والتعليقات
    fn execute_line(&mut self, line: &str) -> Result<Value, String> {
        // نمط: متغير = فعل (مُدخل)
        if line.contains('=') {
            let parts: Vec<&str> = line.splitn(2, '=').collect();
            let var_name = parts[0].trim().to_string();
            let expression = parts[1].trim();

            // تحليل الفعل
            let verb = self.extract_verb(expression)?;
            let input = self.extract_input(expression);

            // تحليل الكلمة العربية
            let analysis = Musarrif::analyse(&verb)?;
            println!("   🔍 فعل: {} (جذر={}, وزن={})", verb, analysis.jidhr, analysis.wazn);

            // توليد الكود
            let code = self.generator.generate(&analysis)?;
            println!("   ⚙️  {}", code.ir.trim());

            // تنفيذ
            let result = self.runtime.execute_intrinsic(&code.intrinsic, input)?;

            // تخزين في الذاكرة
            self.runtime.memory.insert(var_name, result.clone());

            Ok(result)
        } else {
            // فعل بدون متغير
            let verb = self.extract_verb(line)?;
            let input = self.extract_input(line);

            let analysis = Musarrif::analyse(&verb)?;
            println!("   🔍 فعل: {} (جذر={}, وزن={})", verb, analysis.jidhr, analysis.wazn);

            let code = self.generator.generate(&analysis)?;
            println!("   ⚙️  {}", code.ir.trim());

            self.runtime.execute_intrinsic(&code.intrinsic, input)
        }
    }

    /// استخراج الفعل من التعبير (الكلمة الأولى)
    fn extract_verb(&self, expr: &str) -> Result<String, String> {
        let expr = expr.trim();
        // نأخذ الكلمة الأولى (قبل المسافة أو القوس)
        let verb = expr
            .split(|c: char| c.is_whitespace() || c == '(' || c == '"')
            .next()
            .unwrap_or(expr)
            .to_string();

        if verb.is_empty() {
            Err("لم يتم العثور على فعل".to_string())
        } else {
            Ok(verb)
        }
    }

    /// استخراج المُدخل من التعبير (ما بين القوسين أو المقتبسين)
    fn extract_input(&self, expr: &str) -> Option<Value> {
        let expr = expr.trim();

        // نص بين علامتي تنصيص
        if let Some(start) = expr.find('"') {
            if let Some(end) = expr[start + 1..].find('"') {
                let text = &expr[start + 1..start + 1 + end];
                return Some(Value::Text(text.to_string()));
            }
        }

        // رقم
        if let Ok(num) = expr.split_whitespace().last().unwrap_or("").parse::<f64>() {
            return Some(Value::Number(num));
        }

        None
    }

    /// عرض حالة الذاكرة
    pub fn show_memory(&self) {
        println!("\n📦 الذاكرة:");
        for (key, value) in &self.runtime.memory {
            println!("   {} = {:?}", key, value);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_execute_line_simple() {
        let mut engine = BayanEngine::new();
        let result = engine.execute_line("قَرَأَ").unwrap();
        match result {
            Value::Text(t) => assert!(!t.is_empty()),
            _ => panic!("يجب أن يعيد نصاً"),
        }
    }

    #[test]
    fn test_execute_line_with_assignment() {
        let mut engine = BayanEngine::new();
        let result = engine.execute_line("ملفي = قَرَأَ").unwrap();
        match result {
            Value::Text(t) => assert!(!t.is_empty()),
            _ => panic!("يجب أن يعيد نصاً"),
        }
        // تأكد أنه خُزّن في الذاكرة
        assert!(engine.runtime.memory.contains_key("ملفي"));
    }

    #[test]
    fn test_extract_verb() {
        let engine = BayanEngine::new();
        assert_eq!(engine.extract_verb("قَرَأَ \"ملف.txt\"").unwrap(), "قَرَأَ");
        assert_eq!(engine.extract_verb("حَسَبَ 42").unwrap(), "حَسَبَ");
    }

    #[test]
    fn test_extract_input_text() {
        let engine = BayanEngine::new();
        let input = engine.extract_input("قَرَأَ \"كتاب.txt\"");
        match input {
            Some(Value::Text(t)) => assert_eq!(t, "كتاب.txt"),
            _ => panic!("يجب أن يستخرج النص"),
        }
    }
}
