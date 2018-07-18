use std::ops::*;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Vec4f {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

impl Vec4f {
    pub const ZERO: Vec4f = Vec4f { x: 0.0, y: 0.0, z: 0.0, w: 0.0 };
}

impl Add for Vec4f {
    type Output = Vec4f;

    fn add(self, other: Vec4f) -> Vec4f {
        Vec4f {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
            w: self.w + other.w,
        }
    }
}

impl Mul<f32> for Vec4f {
    type Output = Vec4f;

    fn mul(self, s: f32) -> Vec4f {
        Vec4f {
            x: self.x * s,
            y: self.y * s,
            z: self.z * s,
            w: self.w * s,
        }
    }
}

impl Neg for Vec4f {
    type Output = Vec4f;

    fn neg(self) -> Vec4f {
        self * -1.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add() {
    }

    #[test]
    fn scale() {
    }
}
