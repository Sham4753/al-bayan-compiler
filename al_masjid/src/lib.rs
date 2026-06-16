use std::collections::HashMap;
use std::fs;

/// حزمة بيان
#[derive(Debug, Clone)]
pub struct Package {
    pub name: String,
    pub version: String,
    pub description: String,
    pub author: String,
    pub dependencies: Vec<String>,
}

/// سجل الحزم (المسجد)
pub struct Masjid {
    pub packages: HashMap<String, Package>,
}

impl Masjid {
    pub fn new() -> Self {
        Masjid { packages: HashMap::new() }
    }

    /// نشر حزمة
    pub fn publish(&mut self, pkg: Package) {
        println!("📦 تم نشر: {} v{}", pkg.name, pkg.version);
        self.packages.insert(pkg.name.clone(), pkg);
    }

    /// تثبيت حزمة
    pub fn install(&self, name: &str) -> Option<&Package> {
        self.packages.get(name)
    }

    /// بحث عن حزمة
    pub fn search(&self, query: &str) -> Vec<&Package> {
        self.packages.values()
            .filter(|p| p.name.contains(query) || p.description.contains(query))
            .collect()
    }

    /// قائمة كل الحزم
    pub fn list(&self) -> Vec<&Package> {
        self.packages.values().collect()
    }

    /// إزالة حزمة
    pub fn remove(&mut self, name: &str) -> bool {
        self.packages.remove(name).is_some()
    }

    /// حفظ السجل إلى ملف
    pub fn save(&self, path: &str) -> Result<(), String> {
        let mut content = String::from("[\n");
        for (i, (_, pkg)) in self.packages.iter().enumerate() {
            if i > 0 { content.push_str(",\n"); }
            content.push_str(&format!(
                "  {{\"name\":\"{}\",\"version\":\"{}\",\"description\":\"{}\",\"author\":\"{}\"}}",
                pkg.name, pkg.version, pkg.description, pkg.author
            ));
        }
        content.push_str("\n]");
        fs::write(path, content).map_err(|e| e.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_publish_search() {
        let mut m = Masjid::new();
        m.publish(Package {
            name: "معالج_نصوص".into(),
            version: "1.0".into(),
            description: "أدوات معالجة النصوص العربية".into(),
            author: "محمد".into(),
            dependencies: vec![],
        });
        assert_eq!(m.search("نصوص").len(), 1);
        assert!(m.install("معالج_نصوص").is_some());
    }
}
