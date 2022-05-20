//! Material management for objects.

use crate::maths::Color;

/// Defines a material used in scene rendering.
#[derive(Clone, Debug)]
pub struct Material {
  pub color: Color,
  pub ambient: f32,
  pub diffuse: f32,
  pub specular: f32,
  pub shininess: f32,
}

impl Default for Material {
  /// Returns a default material.
  fn default() -> Self {
    Self {
      color: Color::WHITE,
      ambient: 0.1,
      diffuse: 0.9,
      specular: 0.9,
      shininess: 200.0,
    }
  }
}

impl Material {
  /// Applies the given color.
  pub fn with_color(self, color: Color) -> Self {
    Material { color, ..self }
  }

  /// Applies the given ambient value.
  pub fn with_ambient(self, ambient: f32) -> Self {
    Material { ambient, ..self }
  }

  /// Applies the given diffuse value.
  pub fn with_diffuse(self, diffuse: f32) -> Self {
    Material { diffuse, ..self }
  }

  /// Applies the given specular value.
  pub fn with_specular(self, specular: f32) -> Self {
    Material { specular, ..self }
  }

  /// Applies the given shininess value.
  pub fn with_shininess(self, shininess: f32) -> Self {
    Material { shininess, ..self }
  }
}
