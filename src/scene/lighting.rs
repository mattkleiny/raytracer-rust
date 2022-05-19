//! Light sources for scene rendering.

use crate::maths::{Color, Vector};

/// A point light in the scene.
#[derive(Clone, Debug)]
pub struct PointLight {
  pub position: Vector,
  pub intensity: Color,
}

impl PointLight {
  /// Constructs a new point light.
  pub fn new(position: Vector, intensity: Color) -> Self {
    Self {
      position,
      intensity,
    }
  }
}

#[cfg(test)]
mod tests {
  use crate::maths::{rgb, vec3};

  use super::*;

  #[test]
  fn point_light_should_have_position_and_intensity() {
    let light = PointLight::new(vec3(0., 0., 2.), rgb(1., 0., 1.));

    assert_eq!(light.position, vec3(0., 0., 2.));
    assert_eq!(light.intensity, rgb(1., 0., 1.));
  }
}