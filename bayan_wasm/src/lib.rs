use wasm_bindgen::prelude::*;
use bayan_compiler::musarrif::Musarrif;
use bayan_compiler::parser::SentenceParser;
use bayan_compiler::generator::Generator;
use bayan_compiler::balagha::{BalaghaAnalyzer, BalaghaReport};
use bayan_compiler::optimizer::CodeOptimizer;

/// تحليل كلمة عربية
#[wasm_bindgen]
pub fn analyse(word: &str) -> String {
    match Musarrif::analyse(word) {
        Ok(analysis) => serde_json::to_string(&serde_json::json!({
            "original": analysis.original,
            "jidhr": analysis.jidhr,
            "wazn": analysis.wazn,
            "zaman": format!("{:?}", analysis.zaman),
            "damair": format!("{:?}", analysis.damair),
            "irab": format!("{:?}", analysis.irab),
            "success": true
        })).unwrap_or_else(|_| r#"{"error": "فشل"}"#.to_string()),
        Err(e) => serde_json::to_string(&serde_json::json!({
            "error": e,
            "success": false
        })).unwrap_or_else(|_| r#"{"error": "فشل"}"#.to_string()),
    }
}

/// تحليل جملة عربية
#[wasm_bindgen]
pub fn analyse_sentence(text: &str) -> String {
    match SentenceParser::parse(text) {
        Ok(s) => serde_json::to_string(&serde_json::json!({
            "verb": s.verb.map(|v| v.original),
            "subject": s.subject,
            "object": s.object,
            "preposition": s.preposition,
            "genitive": s.genitive,
            "errors": s.errors,
            "full_irab": s.full_irab(),
            "success": true
        })).unwrap_or_else(|_| r#"{"error": "فشل"}"#.to_string()),
        Err(e) => serde_json::to_string(&serde_json::json!({
            "error": e,
            "success": false
        })).unwrap_or_else(|_| r#"{"error": "فشل"}"#.to_string()),
    }
}

/// توليد كود وسيط من كلمة
#[wasm_bindgen]
pub fn generate(word: &str) -> String {
    match Musarrif::analyse(word) {
        Ok(analysis) => {
            let gen = Generator::new();
            match gen.generate(&analysis) {
                Ok(code) => serde_json::to_string(&serde_json::json!({
                    "intrinsic": code.intrinsic,
                    "is_async": code.is_async,
                    "is_parallel": code.is_parallel,
                    "ir": code.ir,
                    "success": true
                })).unwrap_or_else(|_| r#"{"error": "فشل"}"#.to_string()),
                Err(e) => serde_json::to_string(&serde_json::json!({
                    "error": e,
                    "success": false
                })).unwrap_or_else(|_| r#"{"error": "فشل"}"#.to_string()),
            }
        }
        Err(e) => serde_json::to_string(&serde_json::json!({
            "error": e,
            "success": false
        })).unwrap_or_else(|_| r#"{"error": "فشل"}"#.to_string()),
    }
}

/// تقرير البلاغة
#[wasm_bindgen]
pub fn balagha(code: &str) -> String {
    let sentences: Vec<_> = code.lines()
        .filter(|l| !l.trim().is_empty() && !l.trim().starts_with("//"))
        .filter_map(|l| SentenceParser::parse(l.trim()).ok())
        .collect();

    let report = BalaghaAnalyzer::analyze(&sentences);
    let report_text = BalaghaAnalyzer::report(&report);

    serde_json::to_string(&serde_json::json!({
        "level": format!("{:?}", report.level),
        "score": report.score,
        "praise": report.praise,
        "critique": report.critique,
        "suggestion": report.suggestion,
        "report": report_text,
        "success": true
    })).unwrap_or_else(|_| r#"{"error": "فشل"}"#.to_string())
}

/// تحسين الكود
#[wasm_bindgen]
pub fn optimize(code: &str) -> String {
    let sentences: Vec<_> = code.lines()
        .filter(|l| !l.trim().is_empty() && !l.trim().starts_with("//"))
        .filter_map(|l| SentenceParser::parse(l.trim()).ok())
        .collect();

    let mut optimizer = CodeOptimizer::new();
    optimizer.analyze(&sentences);

    serde_json::to_string(&serde_json::json!({
        "suggestions": optimizer.suggestions,
        "report": optimizer.report(),
        "success": true
    })).unwrap_or_else(|_| r#"{"error": "فشل"}"#.to_string())
}

/// تنفيذ كود البيان (محاكاة)
#[wasm_bindgen]
pub fn execute(code: &str) -> String {
    let sentences: Vec<_> = code.lines()
        .filter(|l| !l.trim().is_empty() && !l.trim().starts_with("//"))
        .filter_map(|l| SentenceParser::parse(l.trim()).ok())
        .collect();

    let mut results = Vec::new();
    for s in &sentences {
        if let Some(ref verb) = s.verb {
            let gen = Generator::new();
            if let Ok(generated) = gen.generate(verb) {
                results.push(format!("{}: {}", verb.original, generated.intrinsic));
            }
        }
    }

    serde_json::to_string(&serde_json::json!({
        "executed": results,
        "count": results.len(),
        "success": true
    })).unwrap_or_else(|_| r#"{"error": "فشل"}"#.to_string())
}

/// معلومات اللغة
#[wasm_bindgen]
pub fn version() -> String {
    serde_json::to_string(&serde_json::json!({
        "name": "لغة البيان",
        "version": "0.4.0",
        "slogan": "الكود قرآن",
        "author": "محمد مجد الخطيب - حران العواميد"
    })).unwrap()
}
