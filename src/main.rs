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
  let mut scene = Scene::new();

  scene.add_point_light(PointLight::new(vec3(-10., 10., -10.), rgb(1., 1., 1.)));

  scene.add_object(
    Sphere::new()
      .with_transform(Matrix4x4::translate(1., 0., 0.))
      .with_material(Material::default().with_color(rgb(1., 0., 0.))),
  );

  scene.add_object(
    Sphere::new()
      .with_transform(Matrix4x4::translate(-1., 0., 0.))
      .with_material(Material::default().with_color(rgb(0., 1., 0.))),
  );

  for y in 0..canvas.height() {
    for x in 0..canvas.width() {
      let point = point(0., 0., -5.);
      let direction = vec3(
        x as f32 / canvas.width() as f32 - 0.5,
        y as f32 / canvas.height() as f32 - 0.5,
        1.,
      )
      .normalize();

      let ray = Ray::new(point, direction);

      if let Some(Intersection { object, distance }) = scene.intersect(ray).closest_hit() {
        let hit_position = ray.position(distance);
        let hit_normal = object.normal_at(hit_position);
        let eye = -ray.direction;

        let mut color = Color::BLACK;

        for light in scene.point_lights() {
          // TODO: mix lights appropriately?
          color = phong_lighting(&object.material(), &light, hit_position, hit_normal, eye);
        }

        canvas.set_pixel(x, y, color);
      };
    }
  }

  canvas
    .save_to_png("./output.png")
    .expect("Failed to save PNG file!");
}
