use std::ops::*;
use std::f32::consts::PI;

use vec3::*;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Mat3f {
    pub values: [[f32; 3]; 3],
}

impl Mat3f {
    // Creates a matrix for rotating points about the x axis, assuming a right-handed system.
    pub fn rot_x(a: f32) -> Mat3f {
        let (s, c) = (a.sin(), a.cos());
        Mat3f { values: [
            [1.0, 0.0, 0.0],
            [0.0, c, -s],
            [0.0, s, c],
        ] }
    }

    // Creates a matrix for rotating points about the y axis, assuming a right-handed system.
    pub fn rot_y(a: f32) -> Mat3f {
        let (s, c) = (a.sin(), a.cos());
        Mat3f { values: [
            [c, 0.0, s],
            [0.0, 1.0, 0.0],
            [-s, 0.0, c],
        ] }
    }

    // Creates a matrix for rotating points about the z axis, assuming a right-handed system.
    pub fn rot_z(a: f32) -> Mat3f {
        let (s, c) = (a.sin(), a.cos());
        Mat3f { values: [
            [c, -s, 0.0],
            [s, c, 0.0],
            [0.0, 0.0, 1.0],
        ] }
    }
}

impl Mul<Mat3f> for Mat3f {
    type Output = Mat3f;

    fn mul(self, other: Mat3f) -> Mat3f {
        Mat3f { values: [
            [
                self.values[0][0] * other.values[0][0] + self.values[0][1] * other.values[1][0] + self.values[0][2] * other.values[2][0],
                self.values[0][0] * other.values[0][1] + self.values[0][1] * other.values[1][1] + self.values[0][2] * other.values[2][1],
                self.values[0][0] * other.values[0][2] + self.values[0][1] * other.values[1][2] + self.values[0][2] * other.values[2][2],
            ], [
                self.values[1][0] * other.values[0][0] + self.values[1][1] * other.values[1][0] + self.values[1][2] * other.values[2][0],
                self.values[1][0] * other.values[0][1] + self.values[1][1] * other.values[1][1] + self.values[1][2] * other.values[2][1],
                self.values[1][0] * other.values[0][2] + self.values[1][1] * other.values[1][2] + self.values[1][2] * other.values[2][2],
            ], [
                self.values[2][0] * other.values[0][0] + self.values[2][1] * other.values[1][0] + self.values[2][2] * other.values[2][0],
                self.values[2][0] * other.values[0][1] + self.values[2][1] * other.values[1][1] + self.values[2][2] * other.values[2][1],
                self.values[2][0] * other.values[0][2] + self.values[2][1] * other.values[1][2] + self.values[2][2] * other.values[2][2],
            ]
        ] }
    }
}

impl Mul<Vec3f> for Mat3f {
    type Output = Vec3f;

    fn mul(self, v: Vec3f) -> Vec3f {
        Vec3f {
            x: self.values[0][0] * v.x + self.values[0][1] * v.y + self.values[0][2] * v.z,
            y: self.values[1][0] * v.x + self.values[1][1] * v.y + self.values[1][2] * v.z,
            z: self.values[2][0] * v.x + self.values[2][1] * v.y + self.values[2][2] * v.z,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use vec3::tests::*;

    #[test]
    fn multiply() {
        // TODO
    }

    #[test]
    fn rot_x() {
        assert_vec3f_approx_eq(Vec3f::UNIT_X, Mat3f::rot_x(PI/2.0) * Vec3f::UNIT_X);
        assert_vec3f_approx_eq(Vec3f::UNIT_Z, Mat3f::rot_x(PI/2.0) * Vec3f::UNIT_Y);
        assert_vec3f_approx_eq(-Vec3f::UNIT_Y, Mat3f::rot_x(PI/2.0) * Vec3f::UNIT_Z);
    }

    #[test]
    fn rot_y() {
        assert_vec3f_approx_eq(-Vec3f::UNIT_Z, Mat3f::rot_y(PI/2.0) * Vec3f::UNIT_X);
        assert_vec3f_approx_eq(Vec3f::UNIT_Y, Mat3f::rot_y(PI/2.0) * Vec3f::UNIT_Y);
        assert_vec3f_approx_eq(Vec3f::UNIT_X, Mat3f::rot_y(PI/2.0) * Vec3f::UNIT_Z);
    }

    #[test]
    fn rot_z() {
        assert_vec3f_approx_eq(Vec3f::UNIT_Y, Mat3f::rot_z(PI/2.0) * Vec3f::UNIT_X);
        assert_vec3f_approx_eq(-Vec3f::UNIT_X, Mat3f::rot_z(PI/2.0) * Vec3f::UNIT_Y);
        assert_vec3f_approx_eq(Vec3f::UNIT_Z, Mat3f::rot_z(PI/2.0) * Vec3f::UNIT_Z);
    }
}
