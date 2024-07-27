use bevy::prelude::*;

#[derive(Debug, Clone, Copy)]
pub struct Rect2D {
    min: Vec2,
    max: Vec2,
}

impl Rect2D {
    pub fn new(min: Vec2, max: Vec2) -> Self {
        Self { min, max }
    }

    pub fn intersect(&self, other: &Self) -> bool {
        self.min.x <= other.max.x
            && self.max.x >= other.min.x
            && self.min.y <= other.max.y
            && self.max.y >= other.min.y
    }

    pub fn quadrants(&self) -> Vec<Self> {
        let center = (self.min + self.max) / 2.0;
        vec![
            Self::new(self.min, center),
            Self::new(
                Vec2::new(center.x, self.min.y),
                Vec2::new(self.max.x, center.y),
            ),
            Self::new(
                Vec2::new(self.min.x, center.x),
                Vec2::new(center.x, self.max.y),
            ),
            Self::new(center, self.max),
        ]
    }
}
