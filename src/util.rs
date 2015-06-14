use na::Mat4;

pub fn scale(x: f32, y: f32, z: f32) -> Mat4<f32> {
	Mat4::new(x, 0.0, 0.0, 0.0, 0.0, y, 0.0, 0.0, 0.0, 0.0, z, 0.0, 0.0, 0.0, 0.0, 1.0)
}
