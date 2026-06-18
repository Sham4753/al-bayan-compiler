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

// ========== 50 جذر جديد ==========
pub const ROOT_EXTENSION: &[RootEntry] = &[
    // حركة
    RootEntry { arabic: "مشى", intrinsic: "bayan.move.walk", description: "مشى", category: "حركة" },
    RootEntry { arabic: "جرى", intrinsic: "bayan.move.run", description: "جرى", category: "حركة" },
    RootEntry { arabic: "وقف", intrinsic: "bayan.move.stop", description: "وقف", category: "حركة" },
    RootEntry { arabic: "قفز", intrinsic: "bayan.move.jump", description: "قفز", category: "حركة" },
    RootEntry { arabic: "طار", intrinsic: "bayan.move.fly", description: "طار", category: "حركة" },
    
    // تواصل
    RootEntry { arabic: "قال", intrinsic: "bayan.comm.say", description: "قال", category: "تواصل" },
    RootEntry { arabic: "سأل", intrinsic: "bayan.comm.ask", description: "سأل", category: "تواصل" },
    RootEntry { arabic: "أجاب", intrinsic: "bayan.comm.answer", description: "أجاب", category: "تواصل" },
    RootEntry { arabic: "نادى", intrinsic: "bayan.comm.call", description: "نادى", category: "تواصل" },
    RootEntry { arabic: "صاح", intrinsic: "bayan.comm.shout", description: "صاح", category: "تواصل" },
    
    // إدراك
    RootEntry { arabic: "رأى", intrinsic: "bayan.sense.see", description: "رأى", category: "إدراك" },
    RootEntry { arabic: "سمع", intrinsic: "bayan.sense.hear", description: "سمع", category: "إدراك" },
    RootEntry { arabic: "شم", intrinsic: "bayan.sense.smell", description: "شم", category: "إدراك" },
    RootEntry { arabic: "ذاق", intrinsic: "bayan.sense.taste", description: "ذاق", category: "إدراك" },
    RootEntry { arabic: "لمس", intrinsic: "bayan.sense.touch", description: "لمس", category: "إدراك" },
    
    // عواطف
    RootEntry { arabic: "فرح", intrinsic: "bayan.emoji.happy", description: "فرح", category: "عواطف" },
    RootEntry { arabic: "حزن", intrinsic: "bayan.emoji.sad", description: "حزن", category: "عواطف" },
    RootEntry { arabic: "غضب", intrinsic: "bayan.emoji.angry", description: "غضب", category: "عواطف" },
    RootEntry { arabic: "خاف", intrinsic: "bayan.emoji.fear", description: "خاف", category: "عواطف" },
    RootEntry { arabic: "أحب", intrinsic: "bayan.emoji.love", description: "أحب", category: "عواطف" },
    
    // عمليات
    RootEntry { arabic: "زاد", intrinsic: "bayan.math.add", description: "زاد", category: "عمليات" },
    RootEntry { arabic: "نقص", intrinsic: "bayan.math.sub", description: "نقص", category: "عمليات" },
    RootEntry { arabic: "ضرب", intrinsic: "bayan.math.mul", description: "ضرب", category: "عمليات" },
    RootEntry { arabic: "قسم", intrinsic: "bayan.math.div", description: "قسم", category: "عمليات" },
    RootEntry { arabic: "بقي", intrinsic: "bayan.math.mod", description: "بقي", category: "عمليات" },
    
    // زمن
    RootEntry { arabic: "أمس", intrinsic: "bayan.time.yesterday", description: "أمس", category: "زمن" },
    RootEntry { arabic: "يوم", intrinsic: "bayan.time.today", description: "يوم", category: "زمن" },
    RootEntry { arabic: "غد", intrinsic: "bayan.time.tomorrow", description: "غد", category: "زمن" },
    RootEntry { arabic: "ساع", intrinsic: "bayan.time.hour", description: "ساعة", category: "زمن" },
    RootEntry { arabic: "دقي", intrinsic: "bayan.time.minute", description: "دقيقة", category: "زمن" },
    
    // طبيعة
    RootEntry { arabic: "شمس", intrinsic: "bayan.nature.sun", description: "شمس", category: "طبيعة" },
    RootEntry { arabic: "قمر", intrinsic: "bayan.nature.moon", description: "قمر", category: "طبيعة" },
    RootEntry { arabic: "نجم", intrinsic: "bayan.nature.star", description: "نجم", category: "طبيعة" },
    RootEntry { arabic: "بحر", intrinsic: "bayan.nature.sea", description: "بحر", category: "طبيعة" },
    RootEntry { arabic: "نهر", intrinsic: "bayan.nature.river", description: "نهر", category: "طبيعة" },
    
    // ألوان
    RootEntry { arabic: "أحمر", intrinsic: "bayan.color.red", description: "أحمر", category: "ألوان" },
    RootEntry { arabic: "أخضر", intrinsic: "bayan.color.green", description: "أخضر", category: "ألوان" },
    RootEntry { arabic: "أزرق", intrinsic: "bayan.color.blue", description: "أزرق", category: "ألوان" },
    RootEntry { arabic: "أبيض", intrinsic: "bayan.color.white", description: "أبيض", category: "ألوان" },
    RootEntry { arabic: "أسود", intrinsic: "bayan.color.black", description: "أسود", category: "ألوان" },
    
    // أحجام
    RootEntry { arabic: "كبير", intrinsic: "bayan.size.big", description: "كبير", category: "أحجام" },
    RootEntry { arabic: "صغير", intrinsic: "bayan.size.small", description: "صغير", category: "أحجام" },
    RootEntry { arabic: "طويل", intrinsic: "bayan.size.tall", description: "طويل", category: "أحجام" },
    RootEntry { arabic: "قصير", intrinsic: "bayan.size.short", description: "قصير", category: "أحجام" },
    RootEntry { arabic: "واسع", intrinsic: "bayan.size.wide", description: "واسع", category: "أحجام" },
    
    // تقنية
    RootEntry { arabic: "شغل", intrinsic: "bayan.tech.on", description: "شغل", category: "تقنية" },
    RootEntry { arabic: "أطفأ", intrinsic: "bayan.tech.off", description: "أطفأ", category: "تقنية" },
    RootEntry { arabic: "حمل", intrinsic: "bayan.tech.load", description: "حمل", category: "تقنية" },
    RootEntry { arabic: "أرسل", intrinsic: "bayan.tech.send", description: "أرسل", category: "تقنية" },
    RootEntry { arabic: "استلم", intrinsic: "bayan.tech.receive", description: "استلم", category: "تقنية" },
];

// ========== فئة الشبكات ==========
pub const NETWORK_ROOTS: &[RootEntry] = &[
    RootEntry { arabic: "اتصل", intrinsic: "bayan.net.connect", description: "اتصال", category: "شبكة" },
    RootEntry { arabic: "اقطع", intrinsic: "bayan.net.disconnect", description: "قطع اتصال", category: "شبكة" },
    RootEntry { arabic: "أصغ", intrinsic: "bayan.net.listen", description: "استماع", category: "شبكة" },
    RootEntry { arabic: "اردد", intrinsic: "bayan.net.reply", description: "رد", category: "شبكة" },
    RootEntry { arabic: "وجه", intrinsic: "bayan.net.route", description: "توجيه", category: "شبكة" },
    RootEntry { arabic: "مرر", intrinsic: "bayan.net.forward", description: "تمرير", category: "شبكة" },
    RootEntry { arabic: "حزم", intrinsic: "bayan.net.packet", description: "حزمة", category: "شبكة" },
    RootEntry { arabic: "فكك", intrinsic: "bayan.net.unpack", description: "فك حزمة", category: "شبكة" },
    RootEntry { arabic: "بث", intrinsic: "bayan.net.broadcast", description: "بث", category: "شبكة" },
    RootEntry { arabic: "أمن", intrinsic: "bayan.net.secure", description: "تأمين اتصال", category: "شبكة" },
];

// ========== فئة التزامن ==========
pub const CONCURRENCY_ROOTS: &[RootEntry] = &[
    RootEntry { arabic: "تفرع", intrinsic: "bayan.concurrency.spawn", description: "تشغيل متوازي", category: "تزامن" },
    RootEntry { arabic: "انتظر", intrinsic: "bayan.concurrency.await", description: "انتظار", category: "تزامن" },
    RootEntry { arabic: "أقفل", intrinsic: "bayan.concurrency.lock", description: "قفل", category: "تزامن" },
    RootEntry { arabic: "حرر", intrinsic: "bayan.concurrency.unlock", description: "تحرير", category: "تزامن" },
    RootEntry { arabic: "زامن", intrinsic: "bayan.concurrency.sync", description: "مزامنة", category: "تزامن" },
    RootEntry { arabic: "شارك", intrinsic: "bayan.concurrency.share", description: "مشاركة", category: "تزامن" },
    RootEntry { arabic: "نافس", intrinsic: "bayan.concurrency.race", description: "تنافس", category: "تزامن" },
    RootEntry { arabic: "أجل", intrinsic: "bayan.concurrency.delay", description: "تأجيل", category: "تزامن" },
    RootEntry { arabic: "دفع", intrinsic: "bayan.concurrency.push", description: "دفع بيانات", category: "تزامن" },
    RootEntry { arabic: "سحب", intrinsic: "bayan.concurrency.pull", description: "سحب بيانات", category: "تزامن" },
];

// ========== فئة الوعي ==========
pub const AWARE_ROOTS: &[RootEntry] = &[
    RootEntry { arabic: "اقترح", intrinsic: "bayan.aware.suggest", description: "اقتراح", category: "وعي" },
    RootEntry { arabic: "تعلم", intrinsic: "bayan.aware.learn", description: "تعلم", category: "وعي" },
    RootEntry { arabic: "تذكر", intrinsic: "bayan.aware.remember", description: "تذكر", category: "وعي" },
    RootEntry { arabic: "انس", intrinsic: "bayan.aware.forget", description: "نسيان", category: "وعي" },
    RootEntry { arabic: "فكر", intrinsic: "bayan.aware.think", description: "تفكير", category: "وعي" },
    RootEntry { arabic: "أدرك", intrinsic: "bayan.aware.realize", description: "إدراك", category: "وعي" },
    RootEntry { arabic: "تكيف", intrinsic: "bayan.aware.adapt", description: "تكيف", category: "وعي" },
    RootEntry { arabic: "حسن", intrinsic: "bayan.aware.improve", description: "تحسين", category: "وعي" },
    RootEntry { arabic: "قارن", intrinsic: "bayan.aware.compare", description: "مقارنة", category: "وعي" },
    RootEntry { arabic: "اختار", intrinsic: "bayan.aware.choose", description: "اختيار", category: "وعي" },
];

// ========== فئة المجسات ==========
pub const SENSOR_ROOTS: &[RootEntry] = &[
    RootEntry { arabic: "استشعر", intrinsic: "bayan.sensor.detect", description: "استشعار", category: "مجسات" },
    RootEntry { arabic: "قس", intrinsic: "bayan.sensor.measure", description: "قياس", category: "مجسات" },
    RootEntry { arabic: "حرك", intrinsic: "bayan.sensor.move", description: "تحريك", category: "مجسات" },
    RootEntry { arabic: "أوقف", intrinsic: "bayan.sensor.stop", description: "إيقاف", category: "مجسات" },
    RootEntry { arabic: "وجه", intrinsic: "bayan.sensor.orient", description: "توجيه", category: "مجسات" },
    RootEntry { arabic: "التقط", intrinsic: "bayan.sensor.capture", description: "التقاط", category: "مجسات" },
    RootEntry { arabic: "تعرف", intrinsic: "bayan.sensor.recognize", description: "تعرف", category: "مجسات" },
    RootEntry { arabic: "تتبع", intrinsic: "bayan.sensor.track", description: "تتبع", category: "مجسات" },
    RootEntry { arabic: "حذر", intrinsic: "bayan.sensor.alert", description: "تحذير", category: "مجسات" },
    RootEntry { arabic: "استجب", intrinsic: "bayan.sensor.respond", description: "استجابة", category: "مجسات" },
];

// ========== فئة الديناميكية ==========
pub const DYNAMIC_ROOTS: &[RootEntry] = &[
    RootEntry { arabic: "حدث", intrinsic: "bayan.dynamic.update", description: "تحديث", category: "ديناميكية" },
    RootEntry { arabic: "جدد", intrinsic: "bayan.dynamic.refresh", description: "تجديد", category: "ديناميكية" },
    RootEntry { arabic: "أعد", intrinsic: "bayan.dynamic.restart", description: "إعادة", category: "ديناميكية" },
    RootEntry { arabic: "بدل", intrinsic: "bayan.dynamic.swap", description: "تبديل", category: "ديناميكية" },
    RootEntry { arabic: "أضف", intrinsic: "bayan.dynamic.add", description: "إضافة", category: "ديناميكية" },
    RootEntry { arabic: "أزل", intrinsic: "bayan.dynamic.remove", description: "إزالة", category: "ديناميكية" },
    RootEntry { arabic: "دمج", intrinsic: "bayan.dynamic.merge", description: "دمج", category: "ديناميكية" },
    RootEntry { arabic: "قسم", intrinsic: "bayan.dynamic.split", description: "تقسيم", category: "ديناميكية" },
    RootEntry { arabic: "وسع", intrinsic: "bayan.dynamic.expand", description: "توسيع", category: "ديناميكية" },
    RootEntry { arabic: "قلص", intrinsic: "bayan.dynamic.shrink", description: "تقليص", category: "ديناميكية" },
];
