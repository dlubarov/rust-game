use vertex::*;
use glium::*;
use glium::index::*;

pub struct ColoredMesh {
    pub vertices: Vec<ColoredVertex>,
    pub indices: Vec<u16>,
}

pub struct MeshObjects {
    pub vertex_buffer: VertexBuffer<ColoredVertex>,
    pub index_buffer: IndexBuffer<u16>,
}

impl MeshObjects {
    pub fn create(display: &Display, mesh: &ColoredMesh) -> MeshObjects {
        let vertex_buffer = VertexBuffer::new(display, &mesh.vertices).unwrap();
        let index_buffer = IndexBuffer::new(display, PrimitiveType::TrianglesList, &mesh.indices).unwrap();
        MeshObjects { vertex_buffer, index_buffer }
    }
}
