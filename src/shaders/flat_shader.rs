use image::Rgb;
use nalgebra::{Matrix4, Vector3, Vector4};

use crate::{shader::Shader, triangle_info::TriangleInfo};

pub struct FlatShader {
    varying_intensity: f32,
    pub transform: Matrix4<f32>,
    pub light: Vector3<f32>,
}

impl Shader for FlatShader {
    fn vertex(&mut self, triangle: &TriangleInfo, vertex_index: i32) -> Vector4<f32> {
        let a = triangle.vertex0.vertex;
        let b = triangle.vertex1.vertex;
        let c = triangle.vertex2.vertex;

        let normal = (c - a).cross(&(b - a)).normalize();
        self.varying_intensity = normal.dot(&self.light);

        let tri_vertex = triangle.get_index(vertex_index).vertex;
        let vertex = Vector4::new(tri_vertex.x, tri_vertex.y, tri_vertex.z, 1.0);
        return self.transform * vertex;
    }

    fn fragment(
        &self,
        _fragment: (i32, i32, f32),
        _barycentric: Vector3<f32>,
        pixel_color: &mut Rgb<u8>,
    ) -> bool {
        if self.varying_intensity <= 0.0 {
            pixel_color.0 = [0, 0, 0];
        };

        pixel_color.0 = pixel_color
            .0
            .map(|_| (255.0 * self.varying_intensity) as u8);
        return true;
    }
}

impl FlatShader {
    pub fn new(transform: Matrix4<f32>, light: Vector3<f32>) -> FlatShader {
        return FlatShader {
            varying_intensity: 0.0,
            transform,
            light,
        };
    }
}
