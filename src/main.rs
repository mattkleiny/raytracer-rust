//! A fun little Ray Tracer built with Rust.

#![allow(dead_code)]

#[macro_use]
extern crate anyhow;

mod graphics;
mod maths;
mod scene;

fn main() {
  use maths::*;
  use scene::*;

  let mut canvas = graphics::Canvas::new(256, 256);

  // lets render a simple scene
  let origin = point(0., 0., -5.);
  let sphere = Sphere::new(point(0., 0., 0.), 1.25);

  for y in 0..canvas.height() {
    for x in 0..canvas.width() {
      let direction = vec(x as f32 / canvas.width() as f32 - 0.5, y as f32 / canvas.height() as f32 - 0.5, 1.);
      let ray = Ray::new(origin, direction);

      let set = sphere.intersect(ray);

      if let Some(_) = set.closest_hit() {
        canvas.set_pixel(x, y, Color::RED);
      }
    }
  }

  canvas.save_to_png("./output.png").expect("Failed to save PNG file!");
}