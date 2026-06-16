use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Wazn {
    Faala,
    Fa3ala,
    Faa3ala,
    Ifta3ala,
    Istaf3ala,
    Infa3ala,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Jidhr {
    Qaraa,
    Kataba,
    Hasaba,
    Khazana,
    Ba3atha,
    Jama3a,
    Fasala,
    Rasama,
    Alima,
    Hafitha,
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

impl TasreefRegister {
    pub fn new() -> Self {
        let mut reg = HashMap::new();

        // ق-ر-أ
        reg.insert((Jidhr::Qaraa, Wazn::Faala), Behaviour {
            llvm_intrinsic: "bayan.io.read_sync".into(),
            requires_async: false, spawns_threads: false,
            input_ownership: Ownership::Moved, output_ownership: Ownership::Owned,
            description: "قَرَأَ: قراءة متزامنة",
        });
        reg.insert((Jidhr::Qaraa, Wazn::Faa3ala), Behaviour {
            llvm_intrinsic: "bayan.io.read_async".into(),
            requires_async: true, spawns_threads: false,
            input_ownership: Ownership::Moved, output_ownership: Ownership::Shared,
            description: "قَارَأَ: قراءة غير متزامنة",
        });
        reg.insert((Jidhr::Qaraa, Wazn::Istaf3ala), Behaviour {
            llvm_intrinsic: "bayan.net.http_get".into(),
            requires_async: true, spawns_threads: false,
            input_ownership: Ownership::Moved, output_ownership: Ownership::Moved,
            description: "اِستَقرَأَ: طلب قراءة من API خارجي",
        });

        // ح-س-ب
        reg.insert((Jidhr::Hasaba, Wazn::Faala), Behaviour {
            llvm_intrinsic: "bayan.compute.sync".into(),
            requires_async: false, spawns_threads: false,
            input_ownership: Ownership::Owned, output_ownership: Ownership::Owned,
            description: "حَسَبَ: حساب متزامن",
        });
        reg.insert((Jidhr::Hasaba, Wazn::Fa3ala), Behaviour {
            llvm_intrinsic: "bayan.compute.parallel_map".into(),
            requires_async: false, spawns_threads: true,
            input_ownership: Ownership::Shared, output_ownership: Ownership::Shared,
            description: "حَسَّبَ: معالجة متوازية",
        });
        reg.insert((Jidhr::Hasaba, Wazn::Faa3ala), Behaviour {
            llvm_intrinsic: "bayan.compute.async".into(),
            requires_async: true, spawns_threads: false,
            input_ownership: Ownership::Moved, output_ownership: Ownership::Moved,
            description: "حَاسَبَ: عملية غير متزامنة",
        });
        reg.insert((Jidhr::Hasaba, Wazn::Ifta3ala), Behaviour {
            llvm_intrinsic: "bayan.system.profile".into(),
            requires_async: false, spawns_threads: false,
            input_ownership: Ownership::Owned, output_ownership: Ownership::Owned,
            description: "اِحتَسَبَ: مراقبة أداء ذاتي",
        });

        // ب-ع-ث
        reg.insert((Jidhr::Ba3atha, Wazn::Faala), Behaviour {
            llvm_intrinsic: "bayan.net.send_sync".into(),
            requires_async: false, spawns_threads: false,
            input_ownership: Ownership::Moved, output_ownership: Ownership::Owned,
            description: "بَعَثَ: إرسال متزامن",
        });
        reg.insert((Jidhr::Ba3atha, Wazn::Faa3ala), Behaviour {
            llvm_intrinsic: "bayan.net.send_async".into(),
            requires_async: true, spawns_threads: false,
            input_ownership: Ownership::Moved, output_ownership: Ownership::Shared,
            description: "بَاعَثَ: إرسال غير متزامن",
        });
        reg.insert((Jidhr::Ba3atha, Wazn::Infa3ala), Behaviour {
            llvm_intrinsic: "bayan.net.listen".into(),
            requires_async: true, spawns_threads: true,
            input_ownership: Ownership::Shared, output_ownership: Ownership::Shared,
            description: "اِنبَعَثَ: الاستماع للرسائل",
        });

        // ج-م-ع
        reg.insert((Jidhr::Jama3a, Wazn::Faala), Behaviour {
            llvm_intrinsic: "bayan.collection.create".into(),
            requires_async: false, spawns_threads: false,
            input_ownership: Ownership::Owned, output_ownership: Ownership::Owned,
            description: "جَمَعَ: تجميع في حاوية",
        });

        // ف-ص-ل
        reg.insert((Jidhr::Fasala, Wazn::Faala), Behaviour {
            llvm_intrinsic: "bayan.control.if_else".into(),
            requires_async: false, spawns_threads: false,
            input_ownership: Ownership::Owned, output_ownership: Ownership::Owned,
            description: "فَصَلَ: تفرع شرطي",
        });

        // خ-ز-ن
        reg.insert((Jidhr::Khazana, Wazn::Faala), Behaviour {
            llvm_intrinsic: "bayan.memory.store".into(),
            requires_async: false, spawns_threads: false,
            input_ownership: Ownership::Moved, output_ownership: Ownership::Owned,
            description: "خَزَنَ: تخزين في الذاكرة",
        });

        // ع-ل-م
        reg.insert((Jidhr::Alima, Wazn::Istaf3ala), Behaviour {
            llvm_intrinsic: "bayan.data.query".into(),
            requires_async: true, spawns_threads: false,
            input_ownership: Ownership::Moved, output_ownership: Ownership::Shared,
            description: "اِستَعلَمَ: استعلام من قاعدة بيانات",
        });

        // ح-ف-ظ
        reg.insert((Jidhr::Hafitha, Wazn::Faala), Behaviour {
            llvm_intrinsic: "bayan.security.encrypt".into(),
            requires_async: false, spawns_threads: false,
            input_ownership: Ownership::Moved, output_ownership: Ownership::Owned,
            description: "حَفِظَ: تشفير وحماية",
        });

        // ر-س-م
        reg.insert((Jidhr::Rasama, Wazn::Faala), Behaviour {
            llvm_intrinsic: "bayan.ui.render".into(),
            requires_async: false, spawns_threads: false,
            input_ownership: Ownership::Owned, output_ownership: Ownership::Owned,
            description: "رَسَمَ: رسم على الواجهة",
        });

        // ك-ت-ب
        reg.insert((Jidhr::Kataba, Wazn::Faala), Behaviour {
            llvm_intrinsic: "bayan.io.write_sync".into(),
            requires_async: false, spawns_threads: false,
            input_ownership: Ownership::Moved, output_ownership: Ownership::Owned,
            description: "كَتَبَ: كتابة متزامنة",
        });

        TasreefRegister { register: reg }
    }

    pub fn lookup(&self, j: &Jidhr, w: &Wazn) -> Option<&Behaviour> {
        self.register.get(&(j.clone(), w.clone()))
    }
}
