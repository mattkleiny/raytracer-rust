#![allow(dead_code)]

#[macro_use]
extern crate anyhow;

mod graphics;
mod math;

fn main() {
  use math::*;

  let mut canvas = graphics::Canvas::new(256, 256);

  // lets paint a clock!
  let radius = (3. / 8.) * canvas.width() as f32;
  let origin = point(canvas.width() as f32 / 2., canvas.height() as f32 / 2., 0.);
  let twelve = point(0., 1., 0.);

  for i in 0..12 {
    let rotation = Matrix4x4::rotate_z(i as f32 * std::f32::consts::PI / 6.);
    let position = origin + (rotation * twelve) * radius;

    canvas.set_pixel(position.x as usize, position.y as usize, Color::WHITE);
  }

  canvas.save_to_png("./output.png").expect("Failed to save PNG file!");
}