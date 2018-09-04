use std::ops::*;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Vec2f {
    pub x: f32,
    pub y: f32,
}

impl Vec2f {
    pub const ZERO: Vec2f = Vec2f { x: 0.0, y: 0.0 };
    pub const UNIT_X: Vec2f = Vec2f { x: 1.0, y: 0.0 };
    pub const UNIT_Y: Vec2f = Vec2f { x: 0.0, y: 1.0 };

    pub fn normalized(self) -> Vec2f {
        self / self.norm()
    }

    // L2-norm
    pub fn norm(self) -> f32 {
        self.norm_squared().sqrt()
    }

    fn norm_squared(self) -> f32 {
        self.x * self.x + self.y * self.y
    }
}

impl Add for Vec2f {
    type Output = Vec2f;

    fn add(self, other: Vec2f) -> Vec2f {
        Vec2f { x: self.x + other.x, y: self.y + other.y }
    }
}

impl AddAssign for Vec2f {
    fn add_assign(&mut self, other: Vec2f) {
        self.x += other.x;
        self.y += other.y;
    }
}

impl Mul<f32> for Vec2f {
    type Output = Vec2f;

    fn mul(self, s: f32) -> Vec2f {
        Vec2f { x: self.x * s, y: self.y * s }
    }
}

impl Div<f32> for Vec2f {
    type Output = Vec2f;

    fn div(self, s: f32) -> Vec2f {
        Vec2f { x: self.x / s, y: self.y / s }
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn add() {
        assert_eq!(Vec2f { x: 6.0, y: 9.0 }, Vec2f { x: 2.0, y: 3.0 } + Vec2f { x: 4.0, y: 6.0 });
    }
}
