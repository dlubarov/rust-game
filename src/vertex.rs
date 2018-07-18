#[macro_use]

use glium::*;
use vec3::*;

#[derive(Copy, Clone)]
pub struct ColoredVertex {
    pub position: [f32; 3],
    pub normal: [f32; 3],
    //tex_coords: [f32; 2],
    //color: [f32; 3],
}

impl ColoredVertex {
    pub fn new(position: Vec3f, normal: Vec3f) -> ColoredVertex {
        ColoredVertex {
            position: [position.x, position.y, position.z],
            normal: [normal.x, normal.y, normal.z],
        }
    }
}

implement_vertex!(ColoredVertex, position, normal);
