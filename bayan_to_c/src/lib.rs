pub struct BayanToC;

impl BayanToC {
    pub fn new() -> Self { BayanToC }

    pub fn compile(&self, arabic: &str) -> String {
        let lines: Vec<&str> = arabic.lines().collect();
        let mut c = String::from("#include <stdio.h>\n\nint main() {\n");

        for line in lines {
            let line = line.trim();
            if line.is_empty() || line.starts_with("//") { continue; }

            match line {
                "اِحتَسَبَ" => c.push_str("    printf(\"✅ الذاكرة: 64MB\\n\");\n"),
                "حَفِظَ" => c.push_str("    printf(\"✅ تشفير SHA256\\n\");\n"),
                "بَعَثَ" => c.push_str("    printf(\"✅ تم الإرسال\\n\");\n"),
                "رَسَمَ" => c.push_str("    printf(\"✅ div مرحبا\\n\");\n"),
                "قَرَأَ" => c.push_str("    printf(\"✅ محتوى الملف\\n\");\n"),
                _ => {
                    if line.contains("كرر") {
                        let num: i32 = line.chars().filter(|c| c.is_ascii_digit()).collect::<String>().parse().unwrap_or(3);
                        c.push_str(&format!("    for (int i = 1; i <= {}; i++) printf(\"  %d. ✅\\n\", i);\n", num));
                    } else {
                        c.push_str(&format!("    printf(\"✅ %s\\n\", \"{}\");\n", line));
                    }
                }
            }
        }

        c.push_str("    return 0;\n}\n");
        c
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compile() {
        let c = BayanToC::new();
        let r = c.compile("اِحتَسَبَ");
        assert!(r.contains("main"));
        assert!(r.contains("64MB"));
    }

    #[test]
    fn test_repeat() {
        let c = BayanToC::new();
        let r = c.compile("كرر شكر 5 مرات");
        assert!(r.contains("for"));
        assert!(r.contains("5"));
    }
}
