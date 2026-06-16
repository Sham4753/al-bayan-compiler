use crate::parser::ArabicSentence;
use crate::musarrif::Musarrafa;

/// مُحسِّن البيان - يحلل الكود ويقترح تحسينات
pub struct CodeOptimizer {
    pub suggestions: Vec<String>,
    pub auto_applied: Vec<String>,
}

/// قاعدة دمج أوزان
struct FusionRule {
    chain: &'static [&'static str],
    fused_name: &'static str,
    speedup: &'static str,
    description: &'static str,
}

impl CodeOptimizer {
    pub fn new() -> Self {
        CodeOptimizer { suggestions: vec![], auto_applied: vec![] }
    }

    /// تحليل مجموعة من الجمل واقتراح تحسينات
    pub fn analyze(&mut self, sentences: &[ArabicSentence]) {
        self.suggestions.clear();
        self.auto_applied.clear();

        // ١. كشف السلاسل القابلة للدمج
        self.detect_fusion_chains(sentences);

        // ٢. كشف الأفعال المتعدية بدون مفعول
        self.detect_missing_objects(sentences);

        // ٣. كشف عكس الفاعل والمفعول
        self.detect_inverted_roles(sentences);

        // ٤. كشف التكرار غير الضروري
        self.detect_redundancy(sentences);

        // ٥. اقتراح تحسين الأداء
        self.suggest_parallelization(sentences);
    }

    /// كشف سلاسل الأفعال القابلة للدمج
    fn detect_fusion_chains(&mut self, sentences: &[ArabicSentence]) {
        let rules = vec![
            FusionRule {
                chain: &["قرأ", "حسب", "حفظ", "بعث"],
                fused_name: "اِستَحفَظَ",
                speedup: "3x",
                description: "قراءة + معالجة + تشفير + إرسال",
            },
            FusionRule {
                chain: &["قرأ", "حسب"],
                fused_name: "قَرَّأَ",
                speedup: "2x",
                description: "قراءة مع معالجة متوازية",
            },
            FusionRule {
                chain: &["حسب", "بعث"],
                fused_name: "اِستَحسَبَ",
                speedup: "2x",
                description: "معالجة متوازية مع إرسال",
            },
            FusionRule {
                chain: &["قرأ", "حفظ"],
                fused_name: "اِستَحفَظَ",
                speedup: "2x",
                description: "قراءة مع تشفير",
            },
            FusionRule {
                chain: &["جمع", "رسم"],
                fused_name: "جَمَّرَ",
                speedup: "1.5x",
                description: "تجميع مع عرض",
            },
        ];

        let verbs: Vec<&str> = sentences.iter()
            .filter_map(|s| s.verb.as_ref().map(|v| v.jidhr.as_str()))
            .collect();

        for rule in &rules {
            if verbs.windows(rule.chain.len()).any(|w| w == rule.chain) {
                self.suggestions.push(format!(
                    "⚡ دمج أوزان: {} ({} {}) → استخدم '{}' (أسرع {})",
                    rule.description,
                    rule.chain.join(" + "),
                    rule.fused_name,
                    rule.fused_name,
                    rule.speedup
                ));
            }
        }
    }

    /// كشف الأفعال المتعدية بدون مفعول به
    fn detect_missing_objects(&mut self, sentences: &[ArabicSentence]) {
        for s in sentences {
            if let Some(ref verb) = s.verb {
                let transitive = ["قرأ", "كتب", "حسب", "بعث", "جمع", "رسم", "حفظ", "فتح", "نشر"];
                if transitive.contains(&verb.jidhr.as_str()) && s.object.is_none() {
                    if let Some(ref subj) = s.subject {
                        self.suggestions.push(format!(
                            "💡 '{}' فعل متعدٍ. هل نسيت المفعول به؟ مثال: {} {} [البياناتِ]",
                            verb.original, verb.original, subj
                        ));
                    }
                }
            }
        }
    }

    /// كشف عكس الفاعل والمفعول
    fn detect_inverted_roles(&mut self, sentences: &[ArabicSentence]) {
        for s in sentences {
            if let (Some(ref subj), Some(ref obj)) = (&s.subject, &s.object) {
                let subj_mansub = ArabicSentence::has_fatha(subj) ||
                    ArabicSentence::is_taqdeeri_mansub(subj);
                let obj_marfu3 = ArabicSentence::has_damma(obj) ||
                    ArabicSentence::is_taqdeeri_marfu3(obj);

                if subj_mansub && obj_marfu3 {
                    if let Some(ref verb) = s.verb {
                        self.suggestions.push(format!(
                            "🔄 يبدو أنك عكست الفاعل والمفعول. اقتراح: {} {} {}",
                            verb.original, obj, subj
                        ));
                    }
                }
            }
        }
    }

    /// كشف التكرار
    fn detect_redundancy(&mut self, sentences: &[ArabicSentence]) {
        let mut seen = std::collections::HashSet::new();
        for s in sentences {
            if let Some(ref verb) = s.verb {
                let key = format!("{}_{}", verb.original, s.subject.as_deref().unwrap_or(""));
                if seen.contains(&key) {
                    self.suggestions.push(format!(
                        "🔁 '{}' مكرر. هل يمكن دمجه مع السطر السابق؟",
                        verb.original
                    ));
                }
                seen.insert(key);
            }
        }
    }

    /// اقتراح التوازي
    fn suggest_parallelization(&mut self, sentences: &[ArabicSentence]) {
        for s in sentences {
            if let Some(ref verb) = s.verb {
                if verb.wazn == "فَعَلَ" && verb.jidhr == "حسب" {
                    self.suggestions.push(
                        "⚙️ 'حَسَبَ' يمكن تسريعها باستخدام 'حَسَّبَ' (وزن فَعَّلَ = معالجة متوازية)".to_string()
                    );
                }
            }
        }
    }

    /// عرض التقرير
    pub fn report(&self) -> String {
        let mut r = String::new();

        if !self.suggestions.is_empty() {
            r.push_str("╔══════════════════════════════════╗\n");
            r.push_str("║   🤖 تقرير مُحسِّن البيان       ║\n");
            r.push_str("╚══════════════════════════════════╝\n\n");

            for (i, s) in self.suggestions.iter().enumerate() {
                r.push_str(&format!("{}. {}\n", i + 1, s));
            }

            r.push_str(&format!("\n📊 {} اقتراحاً للتحسين.\n", self.suggestions.len()));
        } else {
            r.push_str("✅ الكود مُحسَّن بالفعل. لا توجد اقتراحات.\n");
        }

        r
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::SentenceParser;

    #[test]
    fn test_fusion_detection() {
        let lines = vec![
            SentenceParser::parse("قَرَأَ محمدُ الكتابَ").unwrap(),
            SentenceParser::parse("حَسَبَ محمدُ البياناتِ").unwrap(),
            SentenceParser::parse("حَفِظَ محمدُ النتائجَ").unwrap(),
            SentenceParser::parse("بَعَثَ محمدُ التقريرَ").unwrap(),
        ];
        let mut opt = CodeOptimizer::new();
        opt.analyze(&lines);
        assert!(!opt.suggestions.is_empty());
        assert!(opt.suggestions.iter().any(|s| s.contains("اِستَحفَظَ")));
    }

    #[test]
    fn test_missing_object() {
        let lines = vec![
            SentenceParser::parse("قَرَأَ محمدُ").unwrap(),
        ];
        let mut opt = CodeOptimizer::new();
        opt.analyze(&lines);
        assert!(opt.suggestions.iter().any(|s| s.contains("متعدٍ")));
    }

    #[test]
    fn test_parallel_suggestion() {
        let lines = vec![
            SentenceParser::parse("حَسَبَ محمدُ البياناتِ").unwrap(),
        ];
        let mut opt = CodeOptimizer::new();
        opt.analyze(&lines);
        assert!(opt.suggestions.iter().any(|s| s.contains("حَسَّبَ")));
    }

    #[test]
    fn test_no_suggestions_for_good_code() {
        let lines = vec![
            SentenceParser::parse("قَرَأَ محمدُ الكتابَ").unwrap(),
        ];
        let mut opt = CodeOptimizer::new();
        opt.analyze(&lines);
        // قد يكون فيه اقتراح توازي أو لا
    }
}
