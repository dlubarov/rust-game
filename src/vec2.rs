use std::ops::*;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Vec2f {
    pub x: f32,
    pub y: f32,
}

impl Add for Vec2f {
    type Output = Vec2f;

    fn add(self, other: Vec2f) -> Vec2f {
        Vec2f { x: self.x + other.x, y: self.y + other.y }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add() {
        assert_eq!(Vec2f { x: 6.0, y: 9.0 }, Vec2f { x: 2.0, y: 3.0 } + Vec2f { x: 4.0, y: 6.0 });
    }
}
