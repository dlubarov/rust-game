use vec3::*;
use mat4::*;
use std::f32::consts::PI;

// Right-handed perspective projection matrix.
pub fn perspective_matrix(width: f32, height: f32) -> Mat4f {
    let aspect = height / width;

    let fov: f32 = PI / 2.0;
    let zfar = 1024.0;
    let znear = 0.1;

    let f = 1.0 / (fov / 2.0).tan();

    Mat4f { values: [
        [f * aspect, 0.0, 0.0,                             0.0],
        [0.0,        f,   0.0,                             0.0],
        [0.0,        0.0, (zfar + znear) / (zfar - znear), (2.0 * zfar * znear) / (zfar - znear)],
        [0.0,        0.0, -1.0,                            0.0],
    ] }
}

pub fn view_matrix(position: &Vec3f, direction: f32) -> Mat4f {
    let translation = Mat4f::translation_vec(-*position);
    let rot_to_pos_x = Mat4f::rot_z(-direction);

    // +x forward, +z up -> -z forward, +x up
    let rot_to_neg_z = Mat4f::rot_y(PI / 2.0);
    // -z forward, +x up -> -z forward, +y up
    let y_up = Mat4f::rot_z(PI / 2.0);

    y_up * rot_to_neg_z * rot_to_pos_x * translation
}
