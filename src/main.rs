pub mod flat_shader;
pub mod gl;
pub mod math_utils;
pub mod model_utils;
pub mod shader;
pub mod triangle_info;

use std::time::Instant;

use flat_shader::FlatShader;
use gl::render;
use image::{imageops::flip_vertical_in_place, Rgb, RgbImage};
use math_utils::create_viewport;
use nalgebra::{Matrix4, Vector3};

const SCREEN_WIDTH: usize = 200;
const SCREEN_HEIGHT: usize = 200;
fn main() {
    let mut screen: image::ImageBuffer<image::Rgb<u8>, Vec<u8>> = RgbImage::new(
        SCREEN_WIDTH.try_into().unwrap(),
        SCREEN_HEIGHT.try_into().unwrap(),
    );

    let mut z_buffer: Vec<f32> = vec![0.0; SCREEN_WIDTH * SCREEN_HEIGHT];
    
    render_flat_shader(&mut screen, &mut z_buffer);

    flip_vertical_in_place(&mut screen);
    screen.save("render.png").unwrap();
}

fn render_flat_shader(screen: &mut image::ImageBuffer<Rgb<u8>, Vec<u8>>, z_buffer: &mut Vec<f32>) {
    let viewport: Matrix4<f32> = create_viewport(
        20,
        0,
        SCREEN_WIDTH.try_into().unwrap(),
        SCREEN_HEIGHT.try_into().unwrap(),
    );
    let light: Vector3<f32> = Vector3::new(0.0, 0.0, -1.0).normalize();

    let mut flat_shader = FlatShader::new(viewport, light);

    let model = model_utils::load_model("Model.obj");
    let start = Instant::now();
    
    render(screen, model, &mut flat_shader, z_buffer.to_owned());
    let elapsed = start.elapsed();
    println!("Time elapsed: {:?}", elapsed);
}
