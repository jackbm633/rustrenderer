use image::Rgb;
use nalgebra::{Vector3, Vector4};

use crate::triangle_info::TriangleInfo;

pub trait Shader {
    fn vertex(&mut self, triangle: &TriangleInfo, vertex_index: i32) -> Vector4<f32>;
    fn fragment(
        &self,
        fragment: (i32, i32, f32),
        barycentric: Vector3<f32>,
        pixel_color: &mut Rgb<u8>,
    ) -> bool;
}
