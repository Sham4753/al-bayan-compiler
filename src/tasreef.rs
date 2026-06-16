use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Wazn { Faala, Fa3ala, Faa3ala }

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Jidhr { Qaraa, Hasaba, Ba3atha }

#[derive(Debug, Clone)]
pub struct Behaviour {
    pub llvm_intrinsic: String,
    pub requires_async: bool,
    pub spawns_threads: bool,
    pub description: &'static str,
}

pub struct TasreefRegister {
    pub register: HashMap<(Jidhr, Wazn), Behaviour>,
}

impl TasreefRegister {
    pub fn new() -> Self {
        let mut reg = HashMap::new();

        reg.insert((Jidhr::Qaraa, Wazn::Faala), Behaviour {
            llvm_intrinsic: "bayan.io.read_sync".into(),
            requires_async: false, spawns_threads: false,
            description: "قَرَأَ: قراءة متزامنة",
        });

        reg.insert((Jidhr::Qaraa, Wazn::Faa3ala), Behaviour {
            llvm_intrinsic: "bayan.io.read_async".into(),
            requires_async: true, spawns_threads: false,
            description: "قَارَأَ: قراءة غير متزامنة",
        });

        reg.insert((Jidhr::Hasaba, Wazn::Faala), Behaviour {
            llvm_intrinsic: "bayan.compute.sync".into(),
            requires_async: false, spawns_threads: false,
            description: "حَسَبَ: حساب متزامن",
        });

        reg.insert((Jidhr::Hasaba, Wazn::Fa3ala), Behaviour {
            llvm_intrinsic: "bayan.compute.parallel".into(),
            requires_async: false, spawns_threads: true,
            description: "حَسَّبَ: حساب متوازي",
        });

        reg.insert((Jidhr::Ba3atha, Wazn::Faala), Behaviour {
            llvm_intrinsic: "bayan.net.send_sync".into(),
            requires_async: false, spawns_threads: false,
            description: "بَعَثَ: إرسال متزامن",
        });

        reg.insert((Jidhr::Ba3atha, Wazn::Faa3ala), Behaviour {
            llvm_intrinsic: "bayan.net.send_async".into(),
            requires_async: true, spawns_threads: false,
            description: "بَاعَثَ: إرسال غير متزامن",
        });

        TasreefRegister { register: reg }
    }

    pub fn lookup(&self, j: &Jidhr, w: &Wazn) -> Option<&Behaviour> {
        self.register.get(&(j.clone(), w.clone()))
    }
}
