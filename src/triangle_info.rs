use nalgebra::{Vector2, Vector3};

#[derive(Clone)]
pub struct VertexInfo {
    pub vertex: Vector3<f32>,
    pub uv: Vector2<f32>,
    pub normal: Vector3<f32>,
}

impl Copy for VertexInfo {}

pub struct TriangleInfo {
    pub vertex0: VertexInfo,
    pub vertex1: VertexInfo,
    pub vertex2: VertexInfo,
}

impl TriangleInfo {
    pub fn get_index(&self, index: i32) -> VertexInfo {
        match index {
            0 => return self.vertex0,
            1 => return self.vertex1,
            2 => return self.vertex2,
            _ => panic!("Invalid index"),
        }
    }
}
