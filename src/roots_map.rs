// ============================================================
// قاعدة الجذور - Roots Database
// أي جذر يضاف هنا يفهمه المترجم تلقائياً
// ============================================================

/// بنية الجذر
pub struct RootEntry {
    pub arabic: &'static str,      // الجذر العربي
    pub intrinsic: &'static str,  // الأمر البرمجي
    pub description: &'static str, // الوصف
    pub category: &'static str,    // المجال
}

/// قاعدة الجذور الكاملة
pub const ROOTS_DB: &[RootEntry] = &[
    // ========== النظام ==========
    RootEntry { arabic: "احتسب", intrinsic: "bayan.system.profile", description: "فحص النظام", category: "نظام" },
    RootEntry { arabic: "استقرأ", intrinsic: "bayan.net.http_get", description: "استدعاء API", category: "نظام" },

    // ========== الأمان ==========
    RootEntry { arabic: "حفظ", intrinsic: "bayan.security.encrypt", description: "تشفير SHA256", category: "أمان" },
    RootEntry { arabic: "أمن", intrinsic: "bayan.security.lock", description: "تأمين", category: "أمان" },

    // ========== الشبكة ==========
    RootEntry { arabic: "بعث", intrinsic: "bayan.net.send_sync", description: "إرسال", category: "شبكة" },
    RootEntry { arabic: "انبعث", intrinsic: "bayan.net.listen", description: "فتح مستمع", category: "شبكة" },
    RootEntry { arabic: "فتح", intrinsic: "bayan.io.open", description: "فتح اتصال", category: "شبكة" },

    // ========== البيانات ==========
    RootEntry { arabic: "خزن", intrinsic: "bayan.memory.store", description: "تخزين", category: "بيانات" },
    RootEntry { arabic: "جمع", intrinsic: "bayan.collection.create", description: "تجميع", category: "بيانات" },
    RootEntry { arabic: "فصل", intrinsic: "bayan.control.if_else", description: "تفرع", category: "بيانات" },
    RootEntry { arabic: "بحث", intrinsic: "bayan.data.search", description: "بحث", category: "بيانات" },
    RootEntry { arabic: "حذف", intrinsic: "bayan.data.delete", description: "حذف", category: "بيانات" },
    RootEntry { arabic: "عد", intrinsic: "bayan.data.count", description: "عد", category: "بيانات" },
    RootEntry { arabic: "رتب", intrinsic: "bayan.data.sort", description: "ترتيب", category: "بيانات" },

    // ========== الواجهة ==========
    RootEntry { arabic: "رسم", intrinsic: "bayan.ui.render", description: "رسم واجهة", category: "واجهة" },

    // ========== الملفات ==========
    RootEntry { arabic: "قرأ", intrinsic: "bayan.io.read_sync", description: "قراءة", category: "ملفات" },
    RootEntry { arabic: "كتب", intrinsic: "bayan.io.write_sync", description: "كتابة", category: "ملفات" },
    RootEntry { arabic: "نسخ", intrinsic: "bayan.io.copy", description: "نسخ", category: "ملفات" },
    RootEntry { arabic: "لصق", intrinsic: "bayan.io.paste", description: "لصق", category: "ملفات" },

    // ========== الذكاء ==========
    RootEntry { arabic: "حلل", intrinsic: "bayan.ai.analyze", description: "تحليل", category: "ذكاء" },
    RootEntry { arabic: "صمم", intrinsic: "bayan.design.make", description: "تصميم", category: "ذكاء" },
    RootEntry { arabic: "طور", intrinsic: "bayan.dev.upgrade", description: "تطوير", category: "ذكاء" },
    RootEntry { arabic: "ترجم", intrinsic: "bayan.ai.translate", description: "ترجمة", category: "ذكاء" },

    // ========== القيم ==========
    RootEntry { arabic: "صدق", intrinsic: "bayan.value.truth", description: "صدق", category: "قيم" },
    RootEntry { arabic: "كذب", intrinsic: "bayan.value.false", description: "كذب", category: "قيم" },
    RootEntry { arabic: "ربح", intrinsic: "bayan.value.win", description: "ربح", category: "قيم" },
    RootEntry { arabic: "خسر", intrinsic: "bayan.value.lose", description: "خسر", category: "قيم" },
    RootEntry { arabic: "قوي", intrinsic: "bayan.value.strong", description: "قوي", category: "قيم" },
    RootEntry { arabic: "ضعف", intrinsic: "bayan.value.weak", description: "ضعف", category: "قيم" },
    RootEntry { arabic: "نجح", intrinsic: "bayan.value.success", description: "نجح", category: "قيم" },
    RootEntry { arabic: "فشل", intrinsic: "bayan.value.fail", description: "فشل", category: "قيم" },
];

impl RootEntry {
    /// البحث عن جذر في القاعدة
    pub fn find(arabic: &str) -> Option<&'static RootEntry> {
        ROOTS_DB.iter().find(|r| r.arabic == arabic)
    }

    /// البحث عن intrinsic
    pub fn find_intrinsic(arabic: &str) -> Option<&'static str> {
        Self::find(arabic).map(|r| r.intrinsic)
    }
}
