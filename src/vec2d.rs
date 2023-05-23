use std::ops::{Add, Mul};

#[derive(Clone, Copy)]
pub struct Vec2D {
    pub x: f32,
    pub y: f32,
}

impl Add<Vec2D> for Vec2D {
    type Output = Vec2D;

    fn add(self, rhs: Vec2D) -> Self::Output {
        Vec2D{
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Mul<f32> for Vec2D {
    type Output = Vec2D;

    fn mul(self, rhs: f32) -> Self::Output {
        Vec2D{
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}
