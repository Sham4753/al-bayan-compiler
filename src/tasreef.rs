use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Wazn { Faala, Fa3ala, Faa3ala, Ifta3ala, Istaf3ala, Infa3ala }

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Jidhr {
    Qaraa, Kataba, Hasaba, Khazana, Ba3atha, Jama3a,
    Fasala, Rasama, Alima, Hafitha, Nasara, Fataha,
    Nashara, Rafa3a, Rahala, Zarra3a, Sami3a, Bana,
    Wasala, Qata3a, Shukr, Sabr, Ghufr, Rahm, Dukhl, Khurj, Saal, Julus, Qaam, Naam, Dhahab, Raja3, Sakan, Hamal, Amal, Daras, Fahm, Hukm, Malik, Salm, Hallala, Jabara, Ghasala, Taba3a, Bahatha, Hathafa, Nasakha, Lasqata, Tarjama, Shaghghala, Ammana, Thalatha, Rattaba
}

#[derive(Debug, Clone)]
pub struct Behaviour {
    pub llvm_intrinsic: String,
    pub requires_async: bool,
    pub spawns_threads: bool,
    pub input_ownership: Ownership,
    pub output_ownership: Ownership,
    pub description: &'static str,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Ownership { Owned, Moved, Shared }

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ExecutionType { Synchronous, Parallel, Async, Introspection, ExternalService, EventListener }

impl Wazn {
    pub fn execution_type(&self) -> ExecutionType {
        match self {
            Wazn::Faala => ExecutionType::Synchronous,
            Wazn::Fa3ala => ExecutionType::Parallel,
            Wazn::Faa3ala => ExecutionType::Async,
            Wazn::Ifta3ala => ExecutionType::Introspection,
            Wazn::Istaf3ala => ExecutionType::ExternalService,
            Wazn::Infa3ala => ExecutionType::EventListener,
        }
    }
}

pub struct TasreefRegister {
    pub register: HashMap<(Jidhr, Wazn), Behaviour>,
}

macro_rules! add {
    ($reg:expr, $jidhr:expr, $wazn:expr, $intrinsic:expr, $async:expr, $threads:expr, $desc:expr) => {
        $reg.insert(($jidhr, $wazn), Behaviour {
            llvm_intrinsic: $intrinsic.into(),
            requires_async: $async,
            spawns_threads: $threads,
            input_ownership: Ownership::Moved,
            output_ownership: Ownership::Owned,
            description: $desc,
        });
    };
}

impl TasreefRegister {
    pub fn new() -> Self {
        let mut reg = HashMap::new();

        // ============ ق-ر-أ ============
        add!(reg, Jidhr::Qaraa, Wazn::Faala, "bayan.io.read_sync", false, false, "قَرَأَ: قراءة متزامنة");
        add!(reg, Jidhr::Qaraa, Wazn::Faa3ala, "bayan.io.read_async", true, false, "قَارَأَ: قراءة غير متزامنة");
        add!(reg, Jidhr::Qaraa, Wazn::Istaf3ala, "bayan.net.http_get", true, false, "اِستَقرَأَ: طلب API خارجي");

        // ============ ك-ت-ب ============
        add!(reg, Jidhr::Kataba, Wazn::Faala, "bayan.io.write_sync", false, false, "كَتَبَ: كتابة متزامنة");
        add!(reg, Jidhr::Kataba, Wazn::Istaf3ala, "bayan.cloud.save", true, false, "اِستَكتَبَ: حفظ سحابي");

        // ============ ح-س-ب ============
        add!(reg, Jidhr::Hasaba, Wazn::Faala, "bayan.compute.sync", false, false, "حَسَبَ: حساب بسيط");
        add!(reg, Jidhr::Hasaba, Wazn::Fa3ala, "bayan.compute.parallel_map", false, true, "حَسَّبَ: معالجة متوازية");
        add!(reg, Jidhr::Hasaba, Wazn::Faa3ala, "bayan.compute.async", true, false, "حَاسَبَ: عملية غير متزامنة");
        add!(reg, Jidhr::Hasaba, Wazn::Ifta3ala, "bayan.system.profile", false, false, "اِحتَسَبَ: مراقبة أداء ذاتي");

        // ============ خ-ز-ن ============
        add!(reg, Jidhr::Khazana, Wazn::Faala, "bayan.memory.store", false, false, "خَزَنَ: تخزين في الذاكرة");
        add!(reg, Jidhr::Khazana, Wazn::Istaf3ala, "bayan.cloud.store", true, false, "اِستَخزَنَ: تخزين سحابي");

        // ============ ب-ع-ث ============
        add!(reg, Jidhr::Ba3atha, Wazn::Faala, "bayan.net.send_sync", false, false, "بَعَثَ: إرسال متزامن");
        add!(reg, Jidhr::Ba3atha, Wazn::Faa3ala, "bayan.net.send_async", true, false, "بَاعَثَ: إرسال غير متزامن");
        add!(reg, Jidhr::Ba3atha, Wazn::Infa3ala, "bayan.net.listen", true, true, "اِنبَعَثَ: فتح مستمع");

        // ============ ج-م-ع ============
        add!(reg, Jidhr::Jama3a, Wazn::Faala, "bayan.collection.create", false, false, "جَمَعَ: تجميع عناصر");

        // ============ ف-ص-ل ============
        add!(reg, Jidhr::Fasala, Wazn::Faala, "bayan.control.if_else", false, false, "فَصَلَ: تفرع شرطي");

        // ============ ر-س-م ============
        add!(reg, Jidhr::Rasama, Wazn::Faala, "bayan.ui.render", false, false, "رَسَمَ: رسم واجهة");

        // ============ ع-ل-م ============
        add!(reg, Jidhr::Alima, Wazn::Faala, "bayan.data.search", false, false, "عَلِمَ: بحث");
        add!(reg, Jidhr::Alima, Wazn::Istaf3ala, "bayan.data.query", true, false, "اِستَعلَمَ: استعلام متقدم");

        // ============ ح-ف-ظ ============
        add!(reg, Jidhr::Hafitha, Wazn::Faala, "bayan.security.encrypt", false, false, "حَفِظَ: تشفير");
        add!(reg, Jidhr::Hafitha, Wazn::Faa3ala, "bayan.security.audit", true, false, "حَافَظَ: تدقيق أمني");
        add!(reg, Jidhr::Hafitha, Wazn::Ifta3ala, "bayan.backup.create", false, false, "اِحتَفَظَ: نسخة احتياطية");

        // ============ ن-ص-ر ============
        add!(reg, Jidhr::Nasara, Wazn::Faala, "bayan.help.call", false, false, "نَصَرَ: طلب مساعدة");

        // ============ ف-ت-ح ============
        add!(reg, Jidhr::Fataha, Wazn::Faala, "bayan.io.open", false, false, "فَتَحَ: فتح اتصال");

        // ============ ن-ش-ر ============
        add!(reg, Jidhr::Nashara, Wazn::Faala, "bayan.publish.send", false, false, "نَشَرَ: نشر محتوى");

        // ============ ر-ف-ع ============
        add!(reg, Jidhr::Rafa3a, Wazn::Faala, "bayan.deploy.upload", false, false, "رَفَعَ: رفع إصدار");

        // ============ ر-ح-ل ============
        add!(reg, Jidhr::Rahala, Wazn::Faala, "bayan.migrate.run", false, false, "رَحَلَ: ترحيل بيانات");

        // ============ ز-ر-ع ============
        add!(reg, Jidhr::Zarra3a, Wazn::Faala, "bayan.seed.create", false, false, "زَرَعَ: زرع بيانات");

        // ============ س-م-ع ============
        add!(reg, Jidhr::Sami3a, Wazn::Faala, "bayan.event.listen", true, false, "سَمِعَ: استماع لحدث");
        add!(reg, Jidhr::Sami3a, Wazn::Infa3ala, "bayan.event.subscribe", true, true, "اِنَسَمَعَ: اشتراك بحدث");

        // ============ ب-ن-ي ============
        add!(reg, Jidhr::Bana, Wazn::Faala, "bayan.build.start", false, false, "بَنَى: بناء مشروع");

        // ============ و-ص-ل ============
        add!(reg, Jidhr::Wasala, Wazn::Faala, "bayan.net.connect", false, false, "وَصَلَ: اتصال");
        add!(reg, Jidhr::Wasala, Wazn::Infa3ala, "bayan.net.receive", true, true, "اِتَّصَلَ: استقبال اتصال");

        // ============ ق-ط-ع ============
        add!(reg, Jidhr::Qata3a, Wazn::Faala, "bayan.net.disconnect", false, false, "قَطَعَ: قطع اتصال");
        add!(reg, Jidhr::Jabara, Wazn::Faala, "bayan.math.add", false, false, "جَبَرَ: جبر");
        add!(reg, Jidhr::Ghasala, Wazn::Faala, "bayan.clean.wash", false, false, "غَسَلَ: غسيل");
        add!(reg, Jidhr::Taba3a, Wazn::Faala, "bayan.io.print", false, false, "طَبَعَ: طباعة");
        add!(reg, Jidhr::Bahatha, Wazn::Faala, "bayan.data.search", false, false, "بَحَثَ: بحث");
        add!(reg, Jidhr::Hathafa, Wazn::Faala, "bayan.data.delete", false, false, "حَذَفَ: حذف");
        add!(reg, Jidhr::Nasakha, Wazn::Faala, "bayan.io.copy", false, false, "نَسَخَ: نسخ");
        add!(reg, Jidhr::Lasqata, Wazn::Faala, "bayan.io.paste", false, false, "لَصَقَ: لصق");
        add!(reg, Jidhr::Tarjama, Wazn::Faala, "bayan.ai.translate", false, false, "تَرجَمَ: ترجمة");
        add!(reg, Jidhr::Shaghghala, Wazn::Faala, "bayan.exec.run", false, false, "شَغَّلَ: تشغيل");
        add!(reg, Jidhr::Ammana, Wazn::Faala, "bayan.security.lock", false, false, "أَمَّنَ: تأمين");
        add!(reg, Jidhr::Thalatha, Wazn::Faala, "bayan.data.count", false, false, "عَدَّ: عدّ");
        add!(reg, Jidhr::Rattaba, Wazn::Faala, "bayan.data.sort", false, false, "رَتَّبَ: ترتيب");

        add!(reg, Jidhr::Hallala, Wazn::Faala, "bayan.ai.analyze", false, false, "حَلَّلَ: تحليل");
        add!(reg, Jidhr::Hallala, Wazn::Fa3ala, "bayan.ai.analyze_deep", false, false, "حَلَّلَ: تحليل عميق");
        TasreefRegister { register: reg }
    }

    pub fn lookup(&self, j: &Jidhr, w: &Wazn) -> Option<&Behaviour> {
        self.register.get(&(j.clone(), w.clone()))
    }
}
