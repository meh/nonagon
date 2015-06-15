use na::Mat4;

pub mod color;
pub use self::color::Color;

pub fn scale(x: f32, y: f32, z: f32) -> Mat4<f32> {
	Mat4::new(x, 0.0, 0.0, 0.0, 0.0, y, 0.0, 0.0, 0.0, 0.0, z, 0.0, 0.0, 0.0, 0.0, 1.0)
}

pub fn deg(v: f32) -> f32 {
	::std::f32::consts::PI * v / 180.0
}

pub fn rgb(r: u8, g: u8, b: u8) -> Color {
	Color::rgb(r, g, b)
}

pub fn rgba(r: u8, g: u8, b: u8, a: u8) -> Color {
	Color::rgba(r, g, b, a)
}
