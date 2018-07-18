use std::f32::consts::PI;
use vertex::*;
use mesh::*;

const CIRCLE_POINTS: u16 = 32;

pub fn make_cylinder(dimension: u8, h1: f32, h2: f32, radius: f32) -> ColoredMesh {
    let mut vertices = Vec::new();
    for i in 0..CIRCLE_POINTS {
        let angle: f32 = 2.0 * PI * i as f32 / CIRCLE_POINTS as f32;
        let c = radius * angle.cos();
        let s = radius * angle.sin();
        let (bottom, top, normal) = match dimension {
            0 => ( // x
                [h1, c, s],
                [h2, c, s],
                [0.0, c, s],
            ),
            1 => ( // y
                [c, h1, s],
                [c, h2, s],
                [c, 0.0, s],
            ),
            2 => ( // z
                [c, s, h1],
                [c, s, h2],
                [c, s, 0.0],
            ),
            _ => panic!("Unexpected dimension"),
        };
        vertices.push(ColoredVertex { position: bottom, normal });
        vertices.push(ColoredVertex { position: top, normal });
    }
    let mut indices = Vec::new();
    for i in 0..CIRCLE_POINTS {
        let curr_bottom = 2 * i;
        let curr_top = 2 * i + 1;
        let next_bottom = (2 * i + 2) % vertices.len() as u16;
        let next_top = (2 * i + 3) % vertices.len() as u16;
        indices.push(curr_bottom);
        indices.push(next_bottom);
        indices.push(next_top);
        indices.push(next_top);
        indices.push(curr_top);
        indices.push(curr_bottom);
    }
    return ColoredMesh { vertices, indices };
}
