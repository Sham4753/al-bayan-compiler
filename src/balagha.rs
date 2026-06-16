//! مُحلِّل البلاغة - يُقيِّم جمال وكفاءة الكود العربي

use crate::parser::ArabicSentence;

/// درجة البلاغة
#[derive(Debug, PartialEq, Eq)]
pub enum BalaghaLevel {
    Maqbul,     // مقبول - يعمل لكن فيه تطويل
    Jayyid,     // جيد - صحيح نحوياً
    Mumtaz,     // ممتاز - مُحسَّن
    Baligh,     // بليغ - موجز وفعال
    Mu3jiz,     // معجز - كلمة واحدة = نظام كامل
}

/// تقرير البلاغة
pub struct BalaghaReport {
    pub level: BalaghaLevel,
    pub score: u32,
    pub praise: Vec<String>,
    pub critique: Vec<String>,
    pub suggestion: Option<String>,
    pub improved_code: Option<String>,
}

pub struct BalaghaAnalyzer;

impl BalaghaAnalyzer {
    /// تحليل بلاغة مجموعة من الجمل
    pub fn analyze(sentences: &[ArabicSentence]) -> BalaghaReport {
        let mut report = BalaghaReport {
            level: BalaghaLevel::Maqbul,
            score: 0,
            praise: vec![],
            critique: vec![],
            suggestion: None,
            improved_code: None,
        };

        let count = sentences.len();
        let mut total_words = 0;
        let mut total_verbs = 0;

        for s in sentences {
            total_words += s.original.split_whitespace().count();
            if s.verb.is_some() { total_verbs += 1; }
        }

        // ========== معايير البلاغة ==========

        // ١. الإيجاز: كلما قل عدد الكلمات لكل عملية، كان أبلغ
        if total_verbs > 0 {
            let ratio = total_words as f64 / total_verbs as f64;
            if ratio <= 2.0 {
                report.score += 30;
                report.praise.push("🎯 إيجاز عالي: معدل كلمة لكل فعل ممتاز".to_string());
            } else if ratio <= 3.0 {
                report.score += 15;
                report.praise.push("📏 إيجاز جيد".to_string());
            } else {
                report.critique.push("📝 الكود فيه تطويل. حاول دمج الأفعال".to_string());
            }
        }

        // ٢. استخدام الأوزان البليغة
        let mut has_fa33ala = false;
        let mut has_faa3ala = false;
        let mut has_istaf3ala = false;

        for s in sentences {
            if let Some(ref verb) = s.verb {
                match verb.wazn.as_str() {
                    "فَعَّلَ" => has_fa33ala = true,
                    "فَاعَلَ" => has_faa3ala = true,
                    "اِستَفعَلَ" => has_istaf3ala = true,
                    _ => {}
                }
            }
        }

        if has_istaf3ala {
            report.score += 25;
            report.praise.push("👑 استخدام وزن 'اِستَفعَلَ' = طلب متقدم (بليغ جداً)".to_string());
        }
        if has_fa33ala {
            report.score += 20;
            report.praise.push("⚡ استخدام وزن 'فَعَّلَ' = معالجة متوازية (بليغ)".to_string());
        }
        if has_faa3ala {
            report.score += 15;
            report.praise.push("🔄 استخدام وزن 'فَاعَلَ' = غير متزامن (جيد)".to_string());
        }

        // ٣. الصحة النحوية
        let mut errors = 0;
        for s in sentences {
            errors += s.errors.len();
        }

        if errors == 0 {
            report.score += 15;
            report.praise.push("✅ الكود صحيح نحوياً بالكامل".to_string());
        } else {
            report.critique.push(format!("⚠️ يوجد {} خطأ نحوياً", errors));
        }

        // ٤. دمج الأوزان - الكشف عن فرص البلاغة
        let verbs: Vec<&str> = sentences.iter()
            .filter_map(|s| s.verb.as_ref().map(|v| v.jidhr.as_str()))
            .collect();

        // قاعدة: إذا وجدنا 4 أفعال متصلة، نقترح وزناً واحداً
        if verbs.len() >= 4 {
            report.score += 10;
            report.suggestion = Some(
                "💎 اقتراح بلاغي: يمكن دمج هذه الأفعال في وزن واحد 'اِستَحفَظَ' أو 'اِستَعلَمَ'".to_string()
            );
            report.improved_code = Some("اِستَحفَظَ (المُدخل، المَخرج)".to_string());
        }

        // ٥. وضوح المعنى
        let mut has_subject = false;
        let mut has_object = false;
        for s in sentences {
            if s.subject.is_some() { has_subject = true; }
            if s.object.is_some() { has_object = true; }
        }

        if has_subject && has_object {
            report.score += 10;
            report.praise.push("📋 أركان الجملة مكتملة (فاعل + مفعول)".to_string());
        }

        // ========== تحديد المستوى ==========
        report.level = match report.score {
            0..=30 => BalaghaLevel::Maqbul,
            31..=55 => BalaghaLevel::Jayyid,
            56..=75 => BalaghaLevel::Mumtaz,
            76..=90 => BalaghaLevel::Baligh,
            _ => BalaghaLevel::Mu3jiz,
        };

        report
    }

    /// عرض تقرير البلاغة
    pub fn report(report: &BalaghaReport) -> String {
        let mut r = String::new();

        let level_str = match report.level {
            BalaghaLevel::Maqbul => "📝 مقبول",
            BalaghaLevel::Jayyid => "✅ جيد",
            BalaghaLevel::Mumtaz => "🌟 ممتاز",
            BalaghaLevel::Baligh => "👑 بليغ",
            BalaghaLevel::Mu3jiz => "💎 معجز",
        };

        r.push_str("╔══════════════════════════════════╗\n");
        r.push_str(&format!("║   📜 تقرير البلاغة: {}      ║\n", level_str));
        r.push_str(&format!("║   النتيجة: {}/100                    ║\n", report.score));
        r.push_str("╚══════════════════════════════════╝\n\n");

        if !report.praise.is_empty() {
            r.push_str("🌹 نقاط القوة:\n");
            for p in &report.praise {
                r.push_str(&format!("   {}\n", p));
            }
        }

        if !report.critique.is_empty() {
            r.push_str("\n📝 ملاحظات:\n");
            for c in &report.critique {
                r.push_str(&format!("   {}\n", c));
            }
        }

        if let Some(ref sug) = report.suggestion {
            r.push_str(&format!("\n💡 اقتراح:\n   {}\n", sug));
        }

        if let Some(ref improved) = report.improved_code {
            r.push_str(&format!("\n✨ الكود البليغ المقترح:\n   {}\n", improved));
        }

        r.push_str("\n");
        r
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::SentenceParser;

    #[test]
    fn test_balagha_for_good_code() {
        let lines = vec![
            SentenceParser::parse("قَرَأَ محمدُ الكتابَ").unwrap(),
        ];
        let report = BalaghaAnalyzer::analyze(&lines);
        assert!(report.score > 0);
        assert!(!report.praise.is_empty());
    }

    #[test]
    fn test_balagha_for_verbose_code() {
        let lines = vec![
            SentenceParser::parse("قَرَأَ محمدُ").unwrap(),
            SentenceParser::parse("حَسَبَ محمدُ").unwrap(),
            SentenceParser::parse("حَفِظَ محمدُ").unwrap(),
            SentenceParser::parse("بَعَثَ محمدُ").unwrap(),
        ];
        let report = BalaghaAnalyzer::analyze(&lines);
        assert!(report.suggestion.is_some());
    }

    #[test]
    fn test_balagha_levels() {
        let good = vec![SentenceParser::parse("اِستَحفَظَ محمدُ").unwrap()];
        let report = BalaghaAnalyzer::analyze(&good);
        assert!(report.level == BalaghaLevel::Baligh || report.level == BalaghaLevel::Mumtaz);
    }
}
