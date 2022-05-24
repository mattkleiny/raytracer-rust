//! Material management for objects.

use crate::graphics::ColorPattern;
use crate::maths::{Color, Vector};

/// A texture for use in material rendering.
pub enum Texture {
  Solid(Color),
  Pattern(Box<dyn ColorPattern>),
}

impl Texture {
  /// Samples the materials color at the given object point.
  pub fn sample_at(&self, point: Vector) -> Color {
    match self {
      Texture::Solid(color) => *color,
      Texture::Pattern(pattern) => pattern.sample_at(point)
    }
  }
}

/// Defines a material used in scene rendering.
pub struct Material {
  pub texture: Texture,
  pub ambient: f64,
  pub diffuse: f64,
  pub specular: f64,
  pub shininess: f64,
  pub reflective: f64,
  pub transparency: f64,
  pub refractive_index: f64,
}

impl Default for Material {
  /// Returns a default material.
  fn default() -> Self {
    Self {
      texture: Texture::Solid(Color::WHITE),
      ambient: 0.1,
      diffuse: 0.9,
      specular: 0.9,
      shininess: 200.0,
      reflective: 0.,
      transparency: 0.,
      refractive_index: 1.,
    }
  }
}

impl Material {
  /// Applies the given color.
  pub fn with_color(self, color: Color) -> Self {
    Material { texture: Texture::Solid(color), ..self }
  }

  /// Applies the given pattern.
  pub fn with_pattern(self, pattern: impl ColorPattern + 'static) -> Self {
    Material { texture: Texture::Pattern(Box::new(pattern)), ..self }
  }

  /// Applies the given ambient value.
  pub fn with_ambient(self, ambient: f64) -> Self {
    Material { ambient, ..self }
  }

  /// Applies the given diffuse value.
  pub fn with_diffuse(self, diffuse: f64) -> Self {
    Material { diffuse, ..self }
  }

  /// Applies the given specular value.
  pub fn with_specular(self, specular: f64) -> Self {
    Material { specular, ..self }
  }

  /// Applies the given shininess value.
  pub fn with_shininess(self, shininess: f64) -> Self {
    Material { shininess, ..self }
  }

  /// Applies the given reflective value.
  pub fn with_reflective(self, reflective: f64) -> Self {
    Material { reflective, ..self }
  }

  /// Applies the given transparency value.
  pub fn with_transparency(self, transparency: f64) -> Self {
    Material { transparency, ..self }
  }

  /// Applies the given refractive value.
  pub fn with_refractive_index(self, refractive: f64) -> Self {
    Material { refractive_index: refractive, ..self }
  }
}

#[cfg(test)]
mod tests {
  use crate::graphics::StripedPattern;
  use crate::maths::point;

  use super::*;

  #[test]
  fn material_should_yield_solid_texture() {
    let material = Material::default().with_color(Color::WHITE);

    assert_eq!(material.texture.sample_at(point(0., 0., 0.)), Color::WHITE);
    assert_eq!(material.texture.sample_at(point(1., 0., 0.)), Color::WHITE);
    assert_eq!(material.texture.sample_at(point(-1., 0., 0.)), Color::WHITE);
  }

  #[test]
  fn material_should_yield_striped_textures() {
    let material = Material::default().with_pattern(StripedPattern::new(Color::WHITE, Color::BLACK));

    assert_eq!(material.texture.sample_at(point(0., 0., 0.)), Color::WHITE);
    assert_eq!(material.texture.sample_at(point(1., 0., 0.)), Color::BLACK);
    assert_eq!(material.texture.sample_at(point(2., 0., 0.)), Color::WHITE);
  }
}