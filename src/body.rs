use mat4::*;
use glium::*;

pub trait Body {
    fn name() -> &'static str where Self: Sized;
    fn draw(&self, target: &mut Frame, program: &Program, view: &Mat4f, projection: &Mat4f);
}

#[derive(Default)]
pub struct Joint {
    pub yaw: f32,
    pub pitch: f32,
    pub roll: f32,
}

impl Joint {
    pub fn to_matrix(&self) -> Mat4f {
        let rx = Mat4f::rot_x(self.yaw);
        let ry = Mat4f::rot_y(self.pitch);
        let rz = Mat4f::rot_z(self.roll);
        rx * ry * rz
    }
}
