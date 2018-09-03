use std::ops::*;
use std::f32::consts::PI;

use mat3::*;
use vec3::*;
use vec4::*;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Mat4f {
    pub values: [[f32; 4]; 4],
}

impl Mul<Mat4f> for Mat4f {
    type Output = Mat4f;

    fn mul(self, other: Mat4f) -> Mat4f {
        Mat4f { values: [
            [
                self.values[0][0] * other.values[0][0] + self.values[0][1] * other.values[1][0] + self.values[0][2] * other.values[2][0] + self.values[0][3] * other.values[3][0],
                self.values[0][0] * other.values[0][1] + self.values[0][1] * other.values[1][1] + self.values[0][2] * other.values[2][1] + self.values[0][3] * other.values[3][1],
                self.values[0][0] * other.values[0][2] + self.values[0][1] * other.values[1][2] + self.values[0][2] * other.values[2][2] + self.values[0][3] * other.values[3][2],
                self.values[0][0] * other.values[0][3] + self.values[0][1] * other.values[1][3] + self.values[0][2] * other.values[2][3] + self.values[0][3] * other.values[3][3],
            ], [
                self.values[1][0] * other.values[0][0] + self.values[1][1] * other.values[1][0] + self.values[1][2] * other.values[2][0] + self.values[1][3] * other.values[3][0],
                self.values[1][0] * other.values[0][1] + self.values[1][1] * other.values[1][1] + self.values[1][2] * other.values[2][1] + self.values[1][3] * other.values[3][1],
                self.values[1][0] * other.values[0][2] + self.values[1][1] * other.values[1][2] + self.values[1][2] * other.values[2][2] + self.values[1][3] * other.values[3][2],
                self.values[1][0] * other.values[0][3] + self.values[1][1] * other.values[1][3] + self.values[1][2] * other.values[2][3] + self.values[1][3] * other.values[3][3],
            ], [
                self.values[2][0] * other.values[0][0] + self.values[2][1] * other.values[1][0] + self.values[2][2] * other.values[2][0] + self.values[2][3] * other.values[3][0],
                self.values[2][0] * other.values[0][1] + self.values[2][1] * other.values[1][1] + self.values[2][2] * other.values[2][1] + self.values[2][3] * other.values[3][1],
                self.values[2][0] * other.values[0][2] + self.values[2][1] * other.values[1][2] + self.values[2][2] * other.values[2][2] + self.values[2][3] * other.values[3][2],
                self.values[2][0] * other.values[0][3] + self.values[2][1] * other.values[1][3] + self.values[2][2] * other.values[2][3] + self.values[2][3] * other.values[3][3],
            ], [
                self.values[3][0] * other.values[0][0] + self.values[3][1] * other.values[1][0] + self.values[3][2] * other.values[2][0] + self.values[3][3] * other.values[3][0],
                self.values[3][0] * other.values[0][1] + self.values[3][1] * other.values[1][1] + self.values[3][2] * other.values[2][1] + self.values[3][3] * other.values[3][1],
                self.values[3][0] * other.values[0][2] + self.values[3][1] * other.values[1][2] + self.values[3][2] * other.values[2][2] + self.values[3][3] * other.values[3][2],
                self.values[3][0] * other.values[0][3] + self.values[3][1] * other.values[1][3] + self.values[3][2] * other.values[2][3] + self.values[3][3] * other.values[3][3],
            ]
        ] }
    }
}

impl Mul<Vec4f> for Mat4f {
    type Output = Vec4f;

    fn mul(self, v: Vec4f) -> Vec4f {
        Vec4f {
            x: self.values[0][0] * v.x + self.values[0][1] * v.y + self.values[0][2] * v.z + self.values[0][3] * v.w,
            y: self.values[1][0] * v.x + self.values[1][1] * v.y + self.values[1][2] * v.z + self.values[1][3] * v.w,
            z: self.values[2][0] * v.x + self.values[2][1] * v.y + self.values[2][2] * v.z + self.values[2][3] * v.w,
            w: self.values[3][0] * v.x + self.values[3][1] * v.y + self.values[3][2] * v.z + self.values[3][3] * v.w,
        }
    }
}

impl Mat4f {
    pub fn rot_x(a: f32) -> Mat4f {
        let m3 = Mat3f::rot_x(a);
        Mat4f { values: [
            [m3.values[0][0], m3.values[0][1], m3.values[0][2], 0.0],
            [m3.values[1][0], m3.values[1][1], m3.values[1][2], 0.0],
            [m3.values[2][0], m3.values[2][1], m3.values[2][2], 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ] }
    }

    pub fn rot_y(a: f32) -> Mat4f {
        let m3 = Mat3f::rot_y(a);
        Mat4f { values: [
            [m3.values[0][0], m3.values[0][1], m3.values[0][2], 0.0],
            [m3.values[1][0], m3.values[1][1], m3.values[1][2], 0.0],
            [m3.values[2][0], m3.values[2][1], m3.values[2][2], 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ] }
    }

    pub fn rot_z(a: f32) -> Mat4f {
        let m3 = Mat3f::rot_z(a);
        Mat4f { values: [
            [m3.values[0][0], m3.values[0][1], m3.values[0][2], 0.0],
            [m3.values[1][0], m3.values[1][1], m3.values[1][2], 0.0],
            [m3.values[2][0], m3.values[2][1], m3.values[2][2], 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ] }
    }

    pub fn translation(dx: f32, dy: f32, dz: f32) -> Self {
        Mat4f { values: [
            [1.0, 0.0, 0.0, dx],
            [0.0, 1.0, 0.0, dy],
            [0.0, 0.0, 1.0, dz],
            [0.0, 0.0, 0.0, 1.0],
        ] }
    }

    pub fn translation_vec(delta: Vec3f) -> Self {
        Mat4f::translation(delta.x, delta.y, delta.z)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use vec4::tests::*;

    #[test]
    fn multiply() {
    }

    #[test]
    fn rot_x() {
        assert_vec4f_approx_eq(Vec4f::UNIT_X, Mat4f::rot_x(PI/2.0) * Vec4f::UNIT_X);
        assert_vec4f_approx_eq(Vec4f::UNIT_Z, Mat4f::rot_x(PI/2.0) * Vec4f::UNIT_Y);
        assert_vec4f_approx_eq(-Vec4f::UNIT_Y, Mat4f::rot_x(PI/2.0) * Vec4f::UNIT_Z);
    }

    #[test]
    fn rot_y() {
        assert_vec4f_approx_eq(-Vec4f::UNIT_Z, Mat4f::rot_y(PI/2.0) * Vec4f::UNIT_X);
        assert_vec4f_approx_eq(Vec4f::UNIT_Y, Mat4f::rot_y(PI/2.0) * Vec4f::UNIT_Y);
        assert_vec4f_approx_eq(Vec4f::UNIT_X, Mat4f::rot_y(PI/2.0) * Vec4f::UNIT_Z);
    }

    #[test]
    fn rot_z() {
        assert_vec4f_approx_eq(Vec4f::UNIT_Y, Mat4f::rot_z(PI/2.0) * Vec4f::UNIT_X);
        assert_vec4f_approx_eq(-Vec4f::UNIT_X, Mat4f::rot_z(PI/2.0) * Vec4f::UNIT_Y);
        assert_vec4f_approx_eq(Vec4f::UNIT_Z, Mat4f::rot_z(PI/2.0) * Vec4f::UNIT_Z);
    }
}
