use std::cmp;

use image::{ImageBuffer, Rgb};
use nalgebra::{Vector2, Vector3, Vector4};

use crate::{math_utils::to_barycentric, shader::Shader, triangle_info::TriangleInfo};

pub fn render(
    image: &mut ImageBuffer<Rgb<u8>, Vec<u8>>,
    model: Vec<TriangleInfo>,
    shader: &mut impl Shader,
    mut z_buffer: Vec<f32>,
) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
    for triangle in model {
        triangle_rasterize(
            image,
            shader.vertex(&triangle, 0),
            shader.vertex(&triangle, 1),
            shader.vertex(&triangle, 2),
            shader,
            &mut z_buffer,
        )
    }
    return image.to_owned();
}

pub fn triangle_rasterize(
    image: &mut ImageBuffer<Rgb<u8>, Vec<u8>>,
    a: Vector4<f32>,
    b: Vector4<f32>,
    c: Vector4<f32>,
    shader: &impl Shader,
    z_buffer: &mut Vec<f32>,
) {
    let a2 = Vector2::<f32>::new(a.x, a.y) / a.w;
    let b2 = Vector2::<f32>::new(b.x, b.y) / b.w;
    let c2 = Vector2::<f32>::new(c.x, c.y) / c.w;

    let min_x = cmp::max(0, cmp::min(a2.x.round() as u32, cmp::min(b2.x.round() as u32, c2.x.round() as u32)));
    let min_y = cmp::max(0, cmp::min(a2.y.round() as u32, cmp::min(b2.y.round() as u32, c2.y.round() as u32)));

    let max_x = cmp::min(
        image.width() - 1,
        cmp::max(a2.x.round() as u32, cmp::max(b2.x.round() as u32, c2.x.round() as u32)),
    );
    let max_y = cmp::min(
        image.height() - 1,
        cmp::max(a2.y.round() as u32, cmp::max(b2.y.round() as u32, c2.y.round() as u32)),
    );

    for x in min_x..max_x + 1 {
        for y in min_y..max_y + 1 {
            let barycentric_screen =
                to_barycentric(Vector2::<f32>::new(x as f32, y as f32), a2, b2, c2);

            if barycentric_screen.x < 0.0
                || barycentric_screen.y < 0.0
                || barycentric_screen.z < 0.0
            {
                continue;
            }

            let mut barycentric_global = Vector3::<f32>::new(
                barycentric_screen.x / a.w,
                barycentric_screen.y / b.w,
                barycentric_screen.z / c.w,
            );

            barycentric_global /=
                barycentric_global.x + barycentric_global.y + barycentric_global.z;

            let depth = a.z * barycentric_global.x
                + b.z * barycentric_global.y
                + c.z * barycentric_global.z;

            if z_buffer[(y * image.width() + x) as usize] >= depth {
                continue;
            }

            z_buffer[(y * image.width() + x) as usize] = depth;

            let mut pixel_color = Rgb([255, 255, 255]);

            shader.fragment(
                (x as i32, y as i32, depth),
                barycentric_global,
                &mut pixel_color,
            );

            image.put_pixel(x, y, pixel_color);
        }
    }
}
