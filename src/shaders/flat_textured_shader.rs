use image::{Rgb, ImageBuffer};
use nalgebra::{Matrix4, Vector3, Vector4, Vector2};

use crate::{shader::Shader, triangle_info::TriangleInfo, texture_utils::get_color};

pub struct FlatTexturedShader {
    varying_uv: Vec<Vector2<f32>>,
    varying_intensity: f32,
    pub transform: Matrix4<f32>,
    pub light: Vector3<f32>,
    pub texture: ImageBuffer<Rgb<u8>, Vec<u8>>
}

impl Shader for FlatTexturedShader {
    fn vertex(&mut self, triangle: &TriangleInfo, vertex_index: i32) -> Vector4<f32> {
        let vertex_info = triangle.get_index(vertex_index);
        // Get uv coordinates for texturing.
        let uv = vertex_info.uv;
        self.varying_uv[vertex_index as usize] = uv;

        // Get triangle vertices.
        let a = triangle.vertex0.vertex;
        let b = triangle.vertex1.vertex;
        let c = triangle.vertex2.vertex;

        let normal = (c - a).cross(&(b - a)).normalize();
        self.varying_intensity = normal.dot(&self.light);

        let tri_vertex = vertex_info.vertex;
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
            return false;
        };

        let uv = self.varying_uv[0] * _barycentric[0] + self.varying_uv[1] * _barycentric[1] + self.varying_uv[2] * _barycentric[2];

        let color = get_color(&self.texture, uv);
        pixel_color.0  = [(color[0] as f32 * self.varying_intensity) as u8, (color[1] as f32 * self.varying_intensity) as u8, (color[2] as f32 * self.varying_intensity) as u8];
        return true;
    }
}

impl FlatTexturedShader {
    pub fn new(transform: Matrix4<f32>, light: Vector3<f32>, texture: ImageBuffer<Rgb<u8>, Vec<u8>>) -> FlatTexturedShader {
        return FlatTexturedShader {
            varying_uv: vec![Vector2::<f32>::new(0.0, 0.0); 3],
            varying_intensity: 0.0,
            transform,
            light,
            texture
        };
    }
}
