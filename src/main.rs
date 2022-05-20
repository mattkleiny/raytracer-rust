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

  // lets render a simple scene
  let mut camera = Camera::new(1920 / 2, 1080 / 2, PI / 2.);
  let mut scene = Scene::new();

  camera.transform = Matrix4x4::translate(0., 0., -5.);

  scene.add_point_light(PointLight::new(vec3(-10., 10., 5.), rgb(1., 1., 1.)));

  scene.add_object(
    Sphere::new()
      .with_transform(Matrix4x4::translate(2., 0., 0.))
      .with_material(Material::default().with_color(rgb(1., 0., 0.))),
  );

  scene.add_object(
    Sphere::new()
      .with_transform(Matrix4x4::translate(-2., 0., 0.))
      .with_material(Material::default().with_color(rgb(0., 1., 0.))),
  );

  camera
    .render(&scene)
    .save_to_png("./output.png")
    .expect("Failed to save PNG file!");
}
