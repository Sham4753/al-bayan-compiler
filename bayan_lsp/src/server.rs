use tower_lsp::lsp_types::*;
use tower_lsp::{Client, LanguageServer};
use bayan_compiler::musarrif::Musarrif;
use bayan_compiler::parser::SentenceParser;
use bayan_compiler::optimizer::CodeOptimizer;
use bayan_compiler::balagha::{BalaghaAnalyzer, BalaghaReport};

pub struct BayanLanguageServer {
    client: Client,
}

impl BayanLanguageServer {
    pub fn new(client: Client) -> Self {
        BayanLanguageServer { client }
    }

    /// تحليل كلمة وعرض تلميحة
    fn hover_info(&self, word: &str) -> String {
        if let Ok(analysis) = Musarrif::analyse(word) {
            format!(
                "🕌 **{}**\n\n🔤 الجذر: {}\n⚖️ الوزن: {}\n⏳ الزمن: {:?}\n👤 الضمائر: {:?}\n📌 الإعراب: {:?}",
                analysis.original,
                analysis.jidhr,
                analysis.wazn,
                analysis.zaman,
                analysis.damair,
                analysis.irab
            )
        } else {
            format!("❌ '{}' ليست كلمة عربية معروفة", word)
        }
    }

    /// اقتراحات الإكمال التلقائي
    fn completion_items(&self) -> Vec<CompletionItem> {
        vec![
            completion("بِسْمِ اللَّهِ", "🕌 بداية البرنامج", CompletionItemKind::KEYWORD, "بِسْمِ اللَّهِ"),
            completion("قَرَأَ", "📖 قراءة متزامنة", CompletionItemKind::FUNCTION, "قَرَأَ (\"ملف.txt\")"),
            completion("كَتَبَ", "✍️ كتابة متزامنة", CompletionItemKind::FUNCTION, "كَتَبَ (\"ملف.txt\"، \"محتوى\")"),
            completion("حَسَبَ", "🔢 حساب بسيط", CompletionItemKind::FUNCTION, "حَسَبَ (أرقام)"),
            completion("حَسَّبَ", "⚡ معالجة متوازية", CompletionItemKind::FUNCTION, "حَسَّبَ (بيانات)"),
            completion("حَاسَبَ", "🔄 غير متزامن", CompletionItemKind::FUNCTION, "حَاسَبَ (بيانات)"),
            completion("حَفِظَ", "🛡️ تشفير", CompletionItemKind::FUNCTION, "حَفِظَ (بيانات)"),
            completion("بَعَثَ", "📡 إرسال", CompletionItemKind::FUNCTION, "بَعَثَ (بيانات، عنوان)"),
            completion("اِستَقرَأَ", "🌐 قراءة من API", CompletionItemKind::FUNCTION, "اِستَقرَأَ (\"رابط\")"),
            completion("اِحتَسَبَ", "📊 مراقبة النظام", CompletionItemKind::FUNCTION, "اِحتَسَبَ ()"),
            completion("اِنبَعَثَ", "👂 فتح مستمع", CompletionItemKind::FUNCTION, "اِنبَعَثَ (8080)"),
            completion("اِستَحفَظَ", "👑 الكلمة الواحدة", CompletionItemKind::FUNCTION, "اِستَحفَظَ (ملف، خادم)"),
            completion("اطبع", "🖨️ إخراج", CompletionItemKind::FUNCTION, "اطبع (\"نص\")"),
        ]
    }

    /// توليد تقرير البلاغة
    fn balagha_report(&self, text: &str) -> String {
        let sentences: Vec<_> = text.lines()
            .filter(|l| !l.trim().is_empty() && !l.trim().starts_with("//"))
            .filter_map(|l| SentenceParser::parse(l.trim()).ok())
            .collect();

        let report = BalaghaAnalyzer::analyze(&sentences);
        BalaghaAnalyzer::report(&report)
    }
}

fn completion(label: &str, detail: &str, kind: CompletionItemKind, insert_text: &str) -> CompletionItem {
    CompletionItem {
        label: label.to_string(),
        detail: Some(detail.to_string()),
        kind: Some(kind),
        insert_text: Some(insert_text.to_string()),
        insert_text_format: Some(InsertTextFormat::SNIPPET),
        ..Default::default()
    }
}

#[tower_lsp::async_trait]
impl LanguageServer for BayanLanguageServer {
    async fn initialize(&self, _: InitializeParams) -> Result<InitializeResult, tower_lsp::jsonrpc::Error> {
        Ok(InitializeResult {
            capabilities: ServerCapabilities {
                hover_provider: Some(HoverProviderCapability::Simple(true)),
                completion_provider: Some(CompletionOptions::default()),
                text_document_sync: Some(TextDocumentSyncCapability::Kind(TextDocumentSyncKind::FULL)),
                ..Default::default()
            },
            ..Default::default()
        })
    }

    async fn hover(&self, params: HoverParams) -> Result<Option<Hover>, tower_lsp::jsonrpc::Error> {
        let position = params.text_document_position_params.position;
        let uri = params.text_document_position_params.text_document.uri;

        // الكلمة تحت المؤشر - مبسطة: نأخذ السطر كاملاً
        // في النسخة الكاملة: نستخرج الكلمة من الموقع
        let word = "قَرَأَ"; // TODO: استخراج الكلمة الحقيقية من الموقع

        let info = self.hover_info(word);

        Ok(Some(Hover {
            contents: HoverContents::Markup(MarkupContent {
                kind: MarkupKind::Markdown,
                value: info,
            }),
            range: None,
        }))
    }

    async fn completion(&self, _: CompletionParams) -> Result<Option<CompletionResponse>, tower_lsp::jsonrpc::Error> {
        let items = self.completion_items();
        Ok(Some(CompletionResponse::Array(items)))
    }

    async fn shutdown(&self) -> Result<(), tower_lsp::jsonrpc::Error> {
        Ok(())
    }
}
