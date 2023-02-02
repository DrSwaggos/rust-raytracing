use std::{
    error::Error,
    fs::File,
    io::{BufWriter, Write},
};

pub fn write_color(out: &mut BufWriter<File>, pixel_color: crate::vec3::Vec3) {
    out.write_all(
        format!(
            "{} {} {}\n",
            (255.999 * pixel_color.x).round() as i32,
            (255.999 * pixel_color.y).round() as i32,
            (255.999 * pixel_color.z).round() as i32
        )
        .as_bytes(),
    )
    .expect("Could not write data");
}

pub const INFTY: f64 = f64::MAX;
pub const PI: f64 = std::f64::consts::PI;

pub fn degrees_to_radians(degrees: f64) -> f64 {
    degrees * PI / 180.0
}
