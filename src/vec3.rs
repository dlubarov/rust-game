use std::ops::*;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Vec3f {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vec3f {
    pub const ZERO: Vec3f = Vec3f { x: 0.0, y: 0.0, z: 0.0 };
    pub const UNIT_X: Vec3f = Vec3f { x: 1.0, y: 0.0, z: 0.0 };
    pub const UNIT_Y: Vec3f = Vec3f { x: 0.0, y: 1.0, z: 0.0 };
    pub const UNIT_Z: Vec3f = Vec3f { x: 0.0, y: 0.0, z: 1.0 };

    // L2-norm
    pub fn norm(self) -> f32 {
        self.norm_squared().sqrt()
    }

    fn norm_squared(self) -> f32 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }
}

impl Add for Vec3f {
    type Output = Vec3f;

    fn add(self, other: Vec3f) -> Vec3f {
        Vec3f {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl Sub for Vec3f {
    type Output = Vec3f;

    fn sub(self, other: Vec3f) -> Vec3f {
        self + -other
    }
}

impl Mul<f32> for Vec3f {
    type Output = Vec3f;

    fn mul(self, s: f32) -> Vec3f {
        Vec3f {
            x: self.x * s,
            y: self.y * s,
            z: self.z * s,
        }
    }
}

impl Neg for Vec3f {
    type Output = Vec3f;

    fn neg(self) -> Vec3f {
        self * -1.0
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn add() {
        assert_vec3f_approx_eq(
            Vec3f { x: 6.0, y: 8.0, z: 11.0 },
            Vec3f { x: 1.0, y: 2.0, z: 3.0 } + Vec3f { x: 5.0, y: 6.0, z: 8.0 });
    }

    #[test]
    fn scale() {
        assert_vec3f_approx_eq(
            Vec3f { x: 2.5, y: 5.0, z: 7.5 },
            Vec3f { x: 1.0, y: 2.0, z: 3.0 } * 2.5);
    }

    pub fn assert_vec3f_approx_eq(a: Vec3f, b: Vec3f) {
        let eps = 1.0e-6;
        assert!((a - b).norm() < eps, r#"assertion failed: `(left ~= right)`
  left: `{:?}`
 right: `{:?}"#,
                a, b);
    }
}
