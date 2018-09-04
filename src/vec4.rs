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
    pub const UNIT_X: Vec4f = Vec4f { x: 1.0, y: 0.0, z: 0.0, w: 0.0 };
    pub const UNIT_Y: Vec4f = Vec4f { x: 0.0, y: 1.0, z: 0.0, w: 0.0 };
    pub const UNIT_Z: Vec4f = Vec4f { x: 0.0, y: 0.0, z: 1.0, w: 0.0 };
    pub const UNIT_W: Vec4f = Vec4f { x: 0.0, y: 0.0, z: 0.0, w: 1.0 };

    pub fn normalized(self) -> Vec4f {
        self / self.norm()
    }

    // L2-norm
    pub fn norm(self) -> f32 {
        self.norm_squared().sqrt()
    }

    fn norm_squared(self) -> f32 {
        self.x * self.x + self.y * self.y + self.z * self.z + self.w * self.w
    }
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

impl Sub for Vec4f {
    type Output = Vec4f;

    fn sub(self, other: Vec4f) -> Vec4f {
        self + -other
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

impl Div<f32> for Vec4f {
    type Output = Vec4f;

    fn div(self, s: f32) -> Vec4f {
        Vec4f {
            x: self.x / s,
            y: self.y / s,
            z: self.z / s,
            w: self.w / s,
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
pub mod tests {
    use super::*;

    #[test]
    fn add() {
        assert_vec4f_approx_eq(
            Vec4f { x: 6.0, y: 8.0, z: 11.0, w: 15.0 },
            Vec4f { x: 1.0, y: 2.0, z: 3.0, w: 4.0 } + Vec4f { x: 5.0, y: 6.0, z: 8.0, w: 11.0 });
    }

    #[test]
    fn scale() {
        assert_vec4f_approx_eq(
            Vec4f { x: 2.5, y: 5.0, z: 7.5, w: 10.0 },
            Vec4f { x: 1.0, y: 2.0, z: 3.0, w: 4.0 } * 2.5);
    }

    pub fn assert_vec4f_approx_eq(a: Vec4f, b: Vec4f) {
        let eps = 1.0e-6;
        assert!((a - b).norm() < eps, r#"assertion failed: `(left ~= right)`
  left: `{:?}`
 right: `{:?}"#,
                a, b);
    }
}
