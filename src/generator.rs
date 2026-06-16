use crate::musarrif::{Musarrafa, Zaman, Damir};
use crate::tasreef::{TasreefRegister, Jidhr, Wazn};

#[derive(Debug, Clone)]
pub struct GeneratedCode {
    pub intrinsic: String,
    pub is_async: bool,
    pub is_parallel: bool,
    pub input_ownership: String,
    pub output_ownership: String,
    pub original: String,
    pub ir: String,
}

pub struct Generator {
    register: TasreefRegister,
}

impl Generator {
    pub fn new() -> Self {
        Generator { register: TasreefRegister::new() }
    }

    pub fn generate(&self, analysis: &Musarrafa) -> Result<GeneratedCode, String> {
        let jidhr = self.match_jidhr(&analysis.jidhr)?;
        let wazn = self.match_wazn(&analysis.wazn)?;
        let behaviour = self.register.lookup(&jidhr, &wazn)
            .ok_or_else(|| format!("لا يوجد سلوك للجذر '{:?}' مع الوزن '{:?}'", jidhr, wazn))?;

        let ir = self.build_ir(analysis, behaviour)?;

        Ok(GeneratedCode {
            intrinsic: behaviour.llvm_intrinsic.clone(),
            is_async: behaviour.requires_async,
            is_parallel: behaviour.spawns_threads,
            input_ownership: format!("{:?}", behaviour.input_ownership),
            output_ownership: format!("{:?}", behaviour.output_ownership),
            original: analysis.original.clone(),
            ir,
        })
    }

    fn match_jidhr(&self, j: &str) -> Result<Jidhr, String> {
        match j {
            "قرأ" => Ok(Jidhr::Qaraa),
            "كتب" => Ok(Jidhr::Kataba),
            "حسب" => Ok(Jidhr::Hasaba),
            "خزن" => Ok(Jidhr::Khazana),
            "بعث" => Ok(Jidhr::Ba3atha),
            "جمع" => Ok(Jidhr::Jama3a),
            "فصل" => Ok(Jidhr::Fasala),
            "رسم" => Ok(Jidhr::Rasama),
            "علم" => Ok(Jidhr::Alima),
            "حفظ" => Ok(Jidhr::Hafitha),
            "نصر" => Ok(Jidhr::Nasara),
            "فتح" => Ok(Jidhr::Fataha),
            "نشر" => Ok(Jidhr::Nashara),
            "رفع" => Ok(Jidhr::Rafa3a),
            "رحل" => Ok(Jidhr::Rahala),
            "زرع" => Ok(Jidhr::Zarra3a),
            "سمع" => Ok(Jidhr::Sami3a),
            "بنى" => Ok(Jidhr::Bana),
            "وصل" => Ok(Jidhr::Wasala),
            "قطع" => Ok(Jidhr::Qata3a),
            "كتب" => Ok(Jidhr::Kataba),
            "حسب" => Ok(Jidhr::Hasaba),
            "خزن" => Ok(Jidhr::Khazana),
            "بعث" => Ok(Jidhr::Ba3atha),
            "جمع" => Ok(Jidhr::Jama3a),
            "فصل" => Ok(Jidhr::Fasala),
            "رسم" => Ok(Jidhr::Rasama),
            "علم" => Ok(Jidhr::Alima),
            "حفظ" => Ok(Jidhr::Hafitha),
            _ => Err(format!("جذر غير معروف: '{}'", j)),
        }
    }

    fn match_wazn(&self, w: &str) -> Result<Wazn, String> {
        match w {
            "فَعَلَ" => Ok(Wazn::Faala),
            "فَعَّلَ" => Ok(Wazn::Fa3ala),
            "فَاعَلَ" => Ok(Wazn::Faa3ala),
            "اِفتَعَلَ" => Ok(Wazn::Ifta3ala),
            "اِستَفعَلَ" => Ok(Wazn::Istaf3ala),
            "اِنفَعَلَ" => Ok(Wazn::Infa3ala),
            "اِفعَل" => Ok(Wazn::Faala),
            _ => Err(format!("وزن غير معروف: '{}'", w)),
        }
    }

    fn build_ir(&self, analysis: &Musarrafa, b: &crate::tasreef::Behaviour) -> Result<String, String> {
        let mut ir = format!("// {} -> {}\n", analysis.original, b.description);
        match analysis.zaman {
            Zaman::Mustaqbal => ir.push_str("SCHEDULE FUTURE "),
            Zaman::Mudari3 if b.requires_async => ir.push_str("AWAIT "),
            Zaman::Madin => ir.push_str("CALL "),
            _ => ir.push_str("EXECUTE "),
        }
        ir.push_str(&b.llvm_intrinsic);
        if !analysis.damair.is_empty() {
            ir.push_str(" WITH_CONTEXT [");
            for (i, d) in analysis.damair.iter().enumerate() {
                if i > 0 { ir.push_str(", "); }
                ir.push_str(&format!("{:?}", d));
            }
            ir.push_str("]");
        }
        if b.spawns_threads { ir.push_str(" PARALLEL"); }
        if b.requires_async { ir.push_str(" ASYNC"); }
        ir.push_str(";\n");
        Ok(ir)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::musarrif::Musarrif;

    #[test]
    fn test_qaraa() {
        let g = Generator::new();
        let a = Musarrif::analyse("قَرَأَ").unwrap();
        let c = g.generate(&a).unwrap();
        assert_eq!(c.intrinsic, "bayan.io.read_sync");
        assert!(!c.is_async);
    }

    #[test]
    fn test_yuhasibu() {
        let g = Generator::new();
        let a = Musarrif::analyse("يُحَاسِبُ").unwrap();
        let c = g.generate(&a).unwrap();
        assert_eq!(c.intrinsic, "bayan.compute.async");
        assert!(c.is_async);
    }

    #[test]
    fn test_sayuhassibu() {
        let g = Generator::new();
        let a = Musarrif::analyse("سَيُحَسِّبُ").unwrap();
        let c = g.generate(&a).unwrap();
        assert_eq!(c.intrinsic, "bayan.compute.parallel_map");
        assert!(c.is_parallel);
    }

    #[test]
    fn test_yahfathuhu() {
        let g = Generator::new();
        let a = Musarrif::analyse("يَحْفَظُهُ").unwrap();
        let c = g.generate(&a).unwrap();
        assert_eq!(c.intrinsic, "bayan.security.encrypt");
        assert!(c.ir.contains("WITH_CONTEXT"));
    }

    #[test]
    fn test_ihtasaba() {
        let g = Generator::new();
        let a = Musarrif::analyse("اِحتَسَبَ").unwrap();
        let c = g.generate(&a).unwrap();
        assert_eq!(c.intrinsic, "bayan.system.profile");
    }

    #[test]
    fn test_inba3atha() {
        let g = Generator::new();
        let a = Musarrif::analyse("اِنبَعَثَ").unwrap();
        let c = g.generate(&a).unwrap();
        assert_eq!(c.intrinsic, "bayan.net.listen");
        assert!(c.is_parallel);
    }

    #[test]
    fn test_istaqraa() {
        let g = Generator::new();
        let a = Musarrif::analyse("اِستَقرَأَ").unwrap();
        let c = g.generate(&a).unwrap();
        assert_eq!(c.intrinsic, "bayan.net.http_get");
        assert!(c.is_async);
    }
}
