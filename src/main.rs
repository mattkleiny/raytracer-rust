//! A fun little Ray Tracer built with Rust.

#![allow(dead_code)]

#[macro_use]
extern crate anyhow;

mod graphics;
mod maths;
mod scene;

fn main() {
  use graphics::*;
  use maths::*;
  use scene::*;

  // lets render a simple scene
  let mut camera = Camera::new(1920 / 2, 1080 / 2, PI / 3.);
  let mut scene = Scene::new();

  camera.transform = Matrix4x4::look_at(point(0., 1.5, -5.), point(0., 1., 0.), vec3(0., 1., 0.));

  scene.add_light(PointLight::new(vec3(-10., 10., -10.), rgb(1., 1., 1.)));

  // floor
  scene.add_object(
    Plane::new(vec3(0., 1., 0.))
      .with_material(Material::default()
        .with_color(rgb(1., 0.9, 0.9))
        .with_specular(0.)),
  );

  // sphere 1
  scene.add_object(
    Sphere::new()
      .with_transform(Matrix4x4::translate(-0.5, 1., 0.5))
      .with_material(Material::default()
        .with_pattern(TransformPattern::new(GradientPattern::new(rgb(1., 0.8, 0.1), rgb(0.5, 1., 0.1)))
          .with_transform(Matrix4x4::scale(0.1, 0.1, 0.1))
          .with_transform(Matrix4x4::rotate_z(PI / 2.))
        )
        .with_diffuse(0.7)
        .with_specular(0.3)
      )
  );

  // sphere 2
  scene.add_object(
    Sphere::new()
      .with_transform(Matrix4x4::translate(1.5, 0.5, -0.5))
      .with_transform(Matrix4x4::scale(0.5, 0.5, 0.5))
      .with_material(Material::default()
        .with_color(rgb(0.5, 1., 0.1))
        .with_diffuse(0.7)
        .with_specular(0.3)
      ),
  );

  // sphere 3
  scene.add_object(
    Sphere::new()
      .with_transform(Matrix4x4::translate(-1.5, 0.33, -0.75))
      .with_transform(Matrix4x4::scale(0.33, 0.33, 0.33))
      .with_material(Material::default()
        .with_color(rgb(1., 0.8, 0.1))
        .with_diffuse(0.7)
        .with_specular(0.3)
      ),
  );

  camera
    .render(&scene)
    .save_to_png("./output.png")
    .expect("Failed to save PNG file!");
}
