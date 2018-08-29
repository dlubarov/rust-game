use vec3::*;
use mat4::*;
use std::f32::consts::PI;

pub fn perspective_matrix(width: f32, height: f32) -> Mat4f {
    let aspect = height / width;

    let fov: f32 = 3.141592 / 3.0;
    let zfar = 1024.0;
    let znear = 0.1;

    let f = 1.0 / (fov / 2.0).tan();

    Mat4f { values: [
        [f * aspect, 0.0, 0.0,                           0.0],
        [0.0,        f,   0.0,                           0.0],
        [0.0,        0.0, (zfar+znear)/(zfar-znear),     1.0],
        [0.0,        0.0, (2.0*zfar*znear)/(zfar-znear), 0.0],
    ] }
}

pub fn view_matrix(position: &[f32; 3], direction: &[f32; 3], up: &[f32; 3]) -> Mat4f {
    // TODO
    let position = Vec3f { x: 0.0, y: 0.0, z: 0.0 };
    let direction = PI / 4.0;

    let translation = Mat4f::translation_vec(-position);
    let rotation = Mat4f::rot_y(-direction);
    rotation * translation

    //let f = {
    //    let f = direction;
    //    let len = f[0] * f[0] + f[1] * f[1] + f[2] * f[2];
    //    let len = len.sqrt();
    //    [f[0] / len, f[1] / len, f[2] / len]
    //};

    //let s = [up[1] * f[2] - up[2] * f[1],
    //         up[2] * f[0] - up[0] * f[2],
    //         up[0] * f[1] - up[1] * f[0]];

    //let s_norm = {
    //    let len = s[0] * s[0] + s[1] * s[1] + s[2] * s[2];
    //    let len = len.sqrt();
    //    [s[0] / len, s[1] / len, s[2] / len]
    //};

    //let u = [f[1] * s_norm[2] - f[2] * s_norm[1],
    //         f[2] * s_norm[0] - f[0] * s_norm[2],
    //         f[0] * s_norm[1] - f[1] * s_norm[0]];

    //let p = [-position[0] * s_norm[0] - position[1] * s_norm[1] - position[2] * s_norm[2],
    //         -position[0] * u[0] - position[1] * u[1] - position[2] * u[2],
    //         -position[0] * f[0] - position[1] * f[1] - position[2] * f[2]];

    //Mat4f { values: [
    //    [s_norm[0], u[0], f[0], 0.0],
    //    [s_norm[1], u[1], f[1], 0.0],
    //    [s_norm[2], u[2], f[2], 0.0],
    //    [p[0], p[1], p[2], 1.0],
    //] }
}
