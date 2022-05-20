//! Light sources for scene rendering.

use crate::maths::{Color, Vector};
use crate::scene::Material;

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

/// Computes lighting for a particular point in the scene via phong model.
pub fn phong_lighting(
  material: &Material,
  light: &PointLight,
  position: Vector,
  eye: Vector,
  normal: Vector,
) -> Color {
  // combine surface color with the light color/intensity
  let effective_color = material.color * light.intensity;

  // find the direction of the light source
  let light_direction = (light.position - position).normalize();

  // compute color contributions
  let ambient = effective_color * material.ambient;
  let mut diffuse = Color::BLACK;
  let mut specular = Color::BLACK;

  // A negative number means the light is on the other side of the surface
  let light_dot_normal = light_direction.dot(normal);
  if light_dot_normal >= 0. {
    // compute the diffuse contribution
    diffuse = effective_color * material.diffuse * light_dot_normal;

    // A negative number means the light reflects away from the eye
    let reflect_direction = Vector::reflect(-light_direction, normal);
    let reflect_dot_eye = reflect_direction.dot(eye);

    if reflect_dot_eye >= 0. {
      // compute the specular contribution
      let factor = reflect_dot_eye.powf(material.shininess);
      specular = light.intensity * material.specular * factor;
    }
  }

  ambient + diffuse + specular
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

  #[test]
  fn phong_lighting_with_the_eye_between_light_and_surface() {
    let material = Material::default();
    let position = vec3(0., 0., 0.);

    let eye = vec3(0., 0., -1.);
    let normal = vec3(0., 0., -1.);
    let light = PointLight::new(vec3(0., 0., -10.), rgb(1., 1., 1.));

    let result = phong_lighting(&material, &light, position, eye, normal);

    assert_eq!(result, rgb(1.9, 1.9, 1.9));
  }

  #[test]
  fn phong_lighting_with_eye_between_light_and_surface_offset_45_degrees() {
    let material = Material::default();
    let position = vec3(0., 0., 0.);

    let eye = vec3(0., 2f32.sqrt() / 2., 2f32.sqrt() / 2.);
    let normal = vec3(0., 0., -1.);
    let light = PointLight::new(vec3(0., 0., -10.), rgb(1., 1., 1.));

    let result = phong_lighting(&material, &light, position, eye, normal);

    assert_eq!(result, rgb(1.0, 1.0, 1.0));
  }

  #[test]
  fn phong_lighting_with_eye_opposite_surface_light_offset_45_degrees() {
    let material = Material::default();
    let position = vec3(0., 0., 0.);

    let eye = vec3(0., 0., -1.);
    let normal = vec3(0., 0., -1.);
    let light = PointLight::new(vec3(0., 10., -10.), rgb(1., 1., 1.));

    let result = phong_lighting(&material, &light, position, eye, normal);

    assert_eq!(result, rgb(0.7364, 0.7364, 0.7364));
  }

  #[test]
  fn phong_lighting_with_eye_in_the_path_of_the_reflection_vector() {
    let material = Material::default();
    let position = vec3(0., 0., 0.);

    let eye = vec3(0., -2f32.sqrt() / 2., -2f32.sqrt() / 2.);
    let normal = vec3(0., 0., -1.);
    let light = PointLight::new(vec3(0., 10., -10.), rgb(1., 1., 1.));

    let result = phong_lighting(&material, &light, position, eye, normal);

    assert_eq!(result, rgb(1.6363853, 1.6363853, 1.6363853));
  }

  #[test]
  fn phong_lighting_with_light_behind_the_surface() {
    let material = Material::default();
    let position = vec3(0., 0., 0.);

    let eye = vec3(0., 0., -1.);
    let normal = vec3(0., 0., -1.);
    let light = PointLight::new(vec3(0., 0., 10.), rgb(1., 1., 1.));

    let result = phong_lighting(&material, &light, position, eye, normal);

    assert_eq!(result, rgb(0.1, 0.1, 0.1));
  }
}
