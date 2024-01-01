pub mod shaders;
pub mod gl;
pub mod math_utils;
pub mod model_utils;
pub mod shader;
pub mod triangle_info;
pub mod texture_utils;

use std::time::Instant;

use gl::render;
use image::{imageops::flip_vertical_in_place, Rgb, RgbImage};
use image::io::Reader as ImageReader;
use math_utils::create_viewport;
use nalgebra::{Matrix4, Vector3};

use crate::math_utils::create_projection;
use crate::shaders::flat_shader::FlatShader;
use crate::shaders::flat_textured_shader::FlatTexturedShader;

const SCREEN_WIDTH: usize = 3840;
const SCREEN_HEIGHT: usize = 3840;
fn main() {
    let mut screen: image::ImageBuffer<image::Rgb<u8>, Vec<u8>> = RgbImage::new(
        SCREEN_WIDTH.try_into().unwrap(),
        SCREEN_HEIGHT.try_into().unwrap(),
    );

    let mut z_buffer: Vec<f32> = vec![0.0; SCREEN_WIDTH * SCREEN_HEIGHT];
    
    render_flat_perspective_shader(&mut screen, &mut z_buffer);

    flip_vertical_in_place(&mut screen);
    screen.save("render.png").unwrap();
}

fn render_flat_shader(screen: &mut image::ImageBuffer<Rgb<u8>, Vec<u8>>, z_buffer: &mut Vec<f32>) {
    let viewport: Matrix4<f32> = create_viewport(
        0,
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

fn render_flat_textured_shader(screen: &mut image::ImageBuffer<Rgb<u8>, Vec<u8>>, z_buffer: &mut Vec<f32>) {
    let viewport: Matrix4<f32> = create_viewport(
        0,
        0,
        SCREEN_WIDTH.try_into().unwrap(),
        SCREEN_HEIGHT.try_into().unwrap(),
    );
    let mut texture = ImageReader::open("Diffuse.png").unwrap().decode().unwrap().to_rgb8();
    let light: Vector3<f32> = Vector3::new(0.0, 0.0, -1.0).normalize();

    let mut flat_shader = FlatTexturedShader::new(viewport, light, texture);

    let model = model_utils::load_model("Model.obj");
    let start = Instant::now();
    
    render(screen, model, &mut flat_shader, z_buffer.to_owned());
    let elapsed = start.elapsed();
    println!("Time elapsed: {:?}", elapsed);
}

fn render_flat_perspective_shader(screen: &mut image::ImageBuffer<Rgb<u8>, Vec<u8>>, z_buffer: &mut Vec<f32>) {
    let camera_position = Vector3::<f32>::new(0.0, 0.0, 1.2);
    let viewport: Matrix4<f32> = create_viewport(
        0,
        0,
        SCREEN_WIDTH.try_into().unwrap(),
        SCREEN_HEIGHT.try_into().unwrap(),
    );
    let projection = create_projection(-1.0 / camera_position.z);
    let mut texture = ImageReader::open("Diffuse.png").unwrap().decode().unwrap().to_rgb8();
    let light: Vector3<f32> = Vector3::new(0.0, 0.0, -1.0).normalize();

    let mut flat_shader = FlatTexturedShader::new(viewport * projection, light, texture);

    let model = model_utils::load_model("Model.obj");
    let start = Instant::now();
    
    render(screen, model, &mut flat_shader, z_buffer.to_owned());
    let elapsed = start.elapsed();
    println!("Time elapsed: {:?}", elapsed);
}