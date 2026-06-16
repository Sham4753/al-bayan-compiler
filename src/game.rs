//! محرك ألعاب البيان

use std::collections::HashMap;

/// كائن في اللعبة
#[derive(Debug, Clone)]
pub struct GameObject {
    pub name: String,
    pub x: f64,
    pub y: f64,
    pub properties: HashMap<String, String>,
}

impl GameObject {
    pub fn new(name: &str, x: f64, y: f64) -> Self {
        GameObject { name: name.to_string(), x, y, properties: HashMap::new() }
    }

    /// تحريك الكائن
    pub fn move_to(&mut self, x: f64, y: f64) {
        self.x = x;
        self.y = y;
    }

    /// تحريك نسبي
    pub fn move_by(&mut self, dx: f64, dy: f64) {
        self.x += dx;
        self.y += dy;
    }

    /// المسافة من كائن آخر
    pub fn distance(&self, other: &GameObject) -> f64 {
        ((self.x - other.x).powi(2) + (self.y - other.y).powi(2)).sqrt()
    }
}

/// عالم اللعبة
pub struct GameWorld {
    pub objects: Vec<GameObject>,
    pub score: u64,
    pub level: u32,
}

impl GameWorld {
    pub fn new() -> Self {
        GameWorld { objects: vec![], score: 0, level: 1 }
    }

    /// إضافة كائن للعالم
    pub fn spawn(&mut self, obj: GameObject) {
        self.objects.push(obj);
    }

    /// تصادم بين كائنين
    pub fn check_collision(&self, a: &GameObject, b: &GameObject) -> bool {
        self.distance(a, b) < 1.0
    }

    fn distance(&self, a: &GameObject, b: &GameObject) -> f64 {
        ((a.x - b.x).powi(2) + (a.y - b.y).powi(2)).sqrt()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_game_object() {
        let mut player = GameObject::new("لاعب", 0.0, 0.0);
        player.move_to(10.0, 5.0);
        assert_eq!(player.x, 10.0);
        assert_eq!(player.y, 5.0);
    }

    #[test]
    fn test_distance() {
        let a = GameObject::new("أ", 0.0, 0.0);
        let b = GameObject::new("ب", 3.0, 4.0);
        assert_eq!(a.distance(&b), 5.0);
    }

    #[test]
    fn test_collision() {
        let a = GameObject::new("أ", 0.0, 0.0);
        let b = GameObject::new("ب", 0.5, 0.5);
        let world = GameWorld::new();
        assert!(world.check_collision(&a, &b));
    }
}
