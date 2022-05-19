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
  let light = PointLight::new(vec3(-10., 10., -10.), rgb(1., 1., 1.));
  let sphere = Sphere::new()
      .with_material(Material::default().with_color(rgb(1., 0.2, 1.)));

  for y in 0..canvas.height() {
    for x in 0..canvas.width() {
      let point = point(0., 0., -5.);
      let direction = vec3(
        x as f32 / canvas.width() as f32 - 0.5,
        y as f32 / canvas.height() as f32 - 0.5,
        1.,
      ).normalize();

      let ray = Ray::new(point, direction);

      if let Some(intersection) = sphere.intersect(ray).closest_hit() {
        let hit_position = ray.position(intersection.distance);
        let hit_normal = sphere.normal_at(hit_position);
        let eye = -ray.direction;

        let color = phong_lighting(
          &intersection.object.material(),
          &light,
          hit_position,
          hit_normal,
          eye,
        );

        canvas.set_pixel(x, y, color);
      };
    }
  }

  canvas.save_to_png("./output.png").expect("Failed to save PNG file!");
}