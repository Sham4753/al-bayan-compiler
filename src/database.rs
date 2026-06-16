//! محرك قواعد بيانات البيان - SQL عربي

use std::collections::HashMap;

/// جدول في قاعدة البيانات
#[derive(Debug, Clone)]
pub struct Table {
    pub name: String,
    pub columns: Vec<String>,
    pub rows: Vec<HashMap<String, String>>,
}

impl Table {
    pub fn new(name: &str, columns: Vec<&str>) -> Self {
        Table {
            name: name.to_string(),
            columns: columns.iter().map(|s| s.to_string()).collect(),
            rows: vec![],
        }
    }

    /// إدراج صف
    pub fn insert(&mut self, values: Vec<&str>) -> Result<(), String> {
        if values.len() != self.columns.len() {
            return Err("عدد القيم لا يساوي عدد الأعمدة".to_string());
        }
        let mut row = HashMap::new();
        for (col, val) in self.columns.iter().zip(values) {
            row.insert(col.clone(), val.to_string());
        }
        self.rows.push(row);
        Ok(())
    }

    /// تحديد - كل الصفوف
    pub fn select_all(&self) -> &Vec<HashMap<String, String>> {
        &self.rows
    }

    /// تحديد بشرط
    pub fn select_where(&self, column: &str, value: &str) -> Vec<&HashMap<String, String>> {
        self.rows.iter().filter(|row| row.get(column) == Some(&value.to_string())).collect()
    }

    /// حذف بشرط
    pub fn delete_where(&mut self, column: &str, value: &str) {
        self.rows.retain(|row| row.get(column) != Some(&value.to_string()));
    }

    /// عدد الصفوف
    pub fn count(&self) -> usize {
        self.rows.len()
    }
}

/// قاعدة بيانات كاملة
pub struct Database {
    pub tables: HashMap<String, Table>,
}

impl Database {
    pub fn new() -> Self {
        Database { tables: HashMap::new() }
    }

    /// إنشاء جدول
    pub fn create_table(&mut self, name: &str, columns: Vec<&str>) {
        self.tables.insert(name.to_string(), Table::new(name, columns));
    }

    /// الحصول على جدول
    pub fn table(&mut self, name: &str) -> Option<&mut Table> {
        self.tables.get_mut(name)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_insert_select() {
        let mut db = Database::new();
        db.create_table("المستخدمون", vec!["اسم", "عمر"]);
        let table = db.table("المستخدمون").unwrap();
        table.insert(vec!["أحمد", "30"]).unwrap();
        table.insert(vec!["محمد", "25"]).unwrap();

        assert_eq!(table.count(), 2);
        let result = table.select_where("اسم", "أحمد");
        assert_eq!(result.len(), 1);
    }

    #[test]
    fn test_delete() {
        let mut db = Database::new();
        db.create_table("t", vec!["x"]);
        let table = db.table("t").unwrap();
        table.insert(vec!["1"]).unwrap();
        table.insert(vec!["2"]).unwrap();
        table.delete_where("x", "1");
        assert_eq!(table.count(), 1);
    }
}
