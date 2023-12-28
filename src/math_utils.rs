use nalgebra::{Matrix4, Vector2, Vector3};

pub(crate) fn create_viewport(x: i32, y: i32, width: i32, height: i32) -> Matrix4<f32> {
    const DEPTH_MIN: f32 = 0.0;
    const DEPTH_MAX: f32 = 1.0;

    // Move to the point x, y
    let mut mat: Matrix4<f32> = Matrix4::zeros();
    mat.m14 = (x as f32) + (width as f32) / 2.0;
    mat.m24 = (y as f32) + (height as f32) / 2.0;
    mat.m34 = (DEPTH_MAX + DEPTH_MIN) / 2.0;

    // Scale
    mat.m11 = (width as f32) / 2.0;
    mat.m22 = (height as f32) / 2.0;
    mat.m33 = (DEPTH_MAX - DEPTH_MIN) / 2.0;
    mat.m44 = 1.0;

    return mat;
}

pub fn to_barycentric(
    p: Vector2<f32>,
    a: Vector2<f32>,
    b: Vector2<f32>,
    c: Vector2<f32>,
) -> Vector3<f32> {
    let ab = b - a;
    let ac = c - a;
    let pa = a - p;

    let mut r = Vector3::<f32>::new(ab.x, ac.x, pa.x).cross(&Vector3::<f32>::new(ab.y, ac.y, pa.y));

    if r.z.abs() < 1e-2 {
        return Vector3::<f32>::new(-1.0, 1.0, 1.0);
    } else {
        r = r / r.z;
        return Vector3::<f32>::new(1.0 - r.x - r.y, r.x, r.y);
    }
}
