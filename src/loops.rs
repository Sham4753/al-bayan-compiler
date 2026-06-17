use crate::functions::Functions;
use crate::runtime::Value;

pub struct Loops { functions: Functions }

impl Loops {
    pub fn new() -> Self { Loops { functions: Functions::new() } }

    pub fn repeat(&mut self, text: &str) -> Result<String, String> {
        let text = text.trim().trim_start_matches("كرر").trim();
        
        let (action, num) = if let Some(pos) = text.rfind(char::is_numeric) {
            let end = text[pos..].chars().take_while(|c| c.is_numeric()).count();
            let num_str = &text[pos..pos+end];
            let action = text[..pos].trim().trim_end_matches(char::is_numeric).trim().to_string();
            (action, num_str.parse::<usize>().unwrap_or(3))
        } else if text.contains("مرات") {
            let p: Vec<&str> = text.split("مرات").collect();
            let action = p[0].trim();
            let words: Vec<&str> = action.split_whitespace().collect();
            let num = words.last().and_then(|w| w.parse().ok()).unwrap_or(3);
            let action = words[..words.len()-1].join(" ");
            (action, num)
        } else {
            return Err("❌ استخدم: كرر <فعل> <عدد> مرات".to_string());
        };

        let mut results = Vec::new();
        for i in 1..=num {
            results.push(format!("  {}. ✅ {}", i, action));
        }
        Ok(format!("🔄 كرر {} مرات:\n{}", num, results.join("\n")))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_repeat_simple() {
        let mut l = Loops::new();
        let r = l.repeat("كرر شكر الله 3 مرات").unwrap();
        assert!(r.contains("1."));
        assert!(r.contains("3."));
    }

    #[test]
    fn test_repeat_number() {
        let mut l = Loops::new();
        let r = l.repeat("كرر احفظ 5 مرات").unwrap();
        assert!(r.contains("5."));
    }
}
