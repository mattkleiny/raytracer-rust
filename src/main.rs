//! A fun little Ray Tracer built with Rust.

#![allow(dead_code)]

use crate::loader::PackedScene;

mod graphics;
mod maths;
mod scene;
mod loader;

fn main() -> anyhow::Result<()> {
  use maths::*;
  use scene::*;

  // lets render a simple scene
  let scene = PackedScene::from_yaml_file("assets/scenes/test01.yaml")?.build()?;
  let camera = Camera::new(1920, 1080, PI / 3.);

  camera.render(&scene).save_to_png("./output.png")?;

  Ok(())
}
