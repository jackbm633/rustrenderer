use image::{ImageBuffer, Rgb};
use nalgebra::Vector2;

pub fn get_color(image: &ImageBuffer<Rgb<u8>, Vec<u8>>, uv: Vector2<f32>) -> Rgb<u8>
{
    return *image.get_pixel((uv[0]*image.width() as f32) as u32, (image.height() as f32 - (uv[1]*image.height() as f32)) as u32)
}