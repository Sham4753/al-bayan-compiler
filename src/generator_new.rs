use crate::roots_map::RootEntry;
use crate::musarrif::Musarrafa;
use std::collections::HashMap;

/// المولد الجديد - يعتمد على قاعدة الجذور
pub struct Generator {
    /// خريطة الجذور للبحث السريع
    root_map: HashMap<String, &'static RootEntry>,
}

impl Generator {
    pub fn new() -> Self {
        let mut map = HashMap::new();
        
        // تحميل كل الجذور من القاعدة
        for root in crate::roots_map::ROOTS_DB {
        for root in crate::roots_map::ROOT_EXTENSION {
            map.insert(root.arabic.to_string(), root);
        }
            map.insert(root.arabic.to_string(), root);
        }
        
        Generator { root_map: map }
    }

    /// توليد intrinsic من تحليل الكلمة
    pub fn generate(&self, analysis: &Musarrafa) -> Result<String, String> {
        // ابحث عن الجذر في القاعدة
        let root = self.root_map.get(&analysis.jidhr)
            .ok_or_else(|| format!("جذر غير معروف: '{}'", analysis.jidhr))?;
        
        Ok(format!("{}: {}", root.intrinsic, root.description))
    }
    
    /// البحث المباشر عن جذر
    pub fn lookup(&self, arabic: &str) -> Option<&&'static RootEntry> {
        self.root_map.get(arabic)
    }
    
    /// عدد الجذور المحملة
    pub fn count(&self) -> usize {
        self.root_map.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_lookup() {
        let gen = Generator::new();
        assert!(gen.lookup("احتسب").is_some());
        assert!(gen.lookup("حفظ").is_some());
        assert!(gen.lookup("بعث").is_some());
    }
    
    #[test]
    fn test_unknown() {
        let gen = Generator::new();
        assert!(gen.lookup("كلمة_غير_موجودة").is_none());
    }
    
    #[test]
    fn test_count() {
        let gen = Generator::new();
        assert!(gen.count() >= 30);
    }
}
