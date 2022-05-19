#![allow(dead_code)]

extern crate core;

mod graphics;
mod math;

fn main() {
  let mut canvas = graphics::Canvas::new(256, 256);

  for y in 0..canvas.height() {
    for x in 0..canvas.width() {
      canvas.set_pixel(x, y, math::rgb(
        x as f32 / canvas.width() as f32,
        y as f32 / canvas.height() as f32,
        0.2,
      ));
    }
  }

  canvas.save_to_png("./output.png").expect("Failed to save PNG file!");
}