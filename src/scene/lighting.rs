//! Light sources for scene rendering.

use crate::maths::{Color, Point, Ray, Vector};
use crate::scene::{Intersection, Material, Traceable};

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

/// Lighting data used in the phong model; computed from intersection information in the scene.
pub struct LightingData<'a> {
  pub object: &'a dyn Traceable,
  pub world_position: Point,
  pub world_position_bias: Point,
  pub object_position: Point,
  pub eye: Vector,
  pub normal: Vector,
  pub reflect_direction: Vector,
  pub distance: f64,
  pub inside: bool,
}

impl<'a> LightingData<'a> { 
  /// Pre-computes the lighting data used in the phong model.
  pub fn calculate(intersection: &'a Intersection, ray: Ray) -> Self {
    let object = intersection.object;
    let world_position = ray.position(intersection.distance);
    let eye = -ray.direction;
    let distance = intersection.distance;

    let mut normal = object.normal_at(world_position);
    let mut inside = false;

    let world_position_bias = world_position + normal * 0.0001;
    let object_position = object.world_to_object(world_position);

    let reflect_direction = ray.direction.reflect(normal);

    if normal.dot(eye) < 0. {
      normal = -normal;
      inside = true;
    }

    Self {
      object,
      world_position,
      world_position_bias,
      object_position,
      eye,
      normal,
      reflect_direction,
      inside,
      distance,
    }
  }
}

/// Computes lighting for a particular point in the scene via phong model.
pub fn phong_lighting(material: &Material, light: &PointLight, world_position: Vector, object_position: Vector, eye: Vector, normal: Vector, in_shadow: bool) -> Color {
  // combine surface color with the light color/intensity
  let effective_color = material.texture.sample_at(object_position) * light.intensity;

  // find the direction of the light source
  let light_direction = (light.position - world_position).normalize();

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
    let reflect_direction = -light_direction.reflect(normal);
    let reflect_dot_eye = reflect_direction.dot(eye);

    if reflect_dot_eye >= 0. {
      // compute the specular contribution
      let factor = reflect_dot_eye.powf(material.shininess);
      specular = light.intensity * material.specular * factor;
    }
  }

  if in_shadow {
    ambient
  } else {
    ambient + diffuse + specular
  }
}

#[cfg(test)]
mod tests {
  use crate::maths::{EPSILON, Matrix4x4, point, rgb, vec3};
  use crate::scene::{Plane, Sphere};

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

    let result = phong_lighting(&material, &light, position, position, eye, normal, false);

    assert_eq!(result, rgb(1.9, 1.9, 1.9));
  }

  #[test]
  fn phong_lighting_with_eye_between_light_and_surface_offset_45_degrees() {
    let material = Material::default();
    let position = vec3(0., 0., 0.);

    let eye = vec3(0., 2f64.sqrt() / 2., 2f64.sqrt() / 2.);
    let normal = vec3(0., 0., -1.);
    let light = PointLight::new(vec3(0., 0., -10.), rgb(1., 1., 1.));

    let result = phong_lighting(&material, &light, position, position, eye, normal, false);

    assert_eq!(result, rgb(1.0, 1.0, 1.0));
  }

  #[test]
  fn phong_lighting_with_eye_opposite_surface_light_offset_45_degrees() {
    let material = Material::default();
    let position = vec3(0., 0., 0.);

    let eye = vec3(0., 0., -1.);
    let normal = vec3(0., 0., -1.);
    let light = PointLight::new(vec3(0., 10., -10.), rgb(1., 1., 1.));

    let result = phong_lighting(&material, &light, position, position, eye, normal, false);

    assert_eq!(result, rgb(0.7364, 0.7364, 0.7364));
  }

  #[test]
  fn phong_lighting_with_eye_in_the_path_of_the_reflection_vector() {
    let material = Material::default();
    let position = vec3(0., 0., 0.);

    let eye = vec3(0., -2f64.sqrt() / 2., -2f64.sqrt() / 2.);
    let normal = vec3(0., 0., -1.);
    let light = PointLight::new(vec3(0., 10., -10.), rgb(1., 1., 1.));

    let result = phong_lighting(&material, &light, position, position, eye, normal, false);

    assert_eq!(result, rgb(1.6363961030678928, 1.6363961030678928, 1.6363961030678928));
  }

  #[test]
  fn phong_lighting_with_light_behind_the_surface() {
    let material = Material::default();
    let position = vec3(0., 0., 0.);

    let eye = vec3(0., 0., -1.);
    let normal = vec3(0., 0., -1.);
    let light = PointLight::new(vec3(0., 0., 10.), rgb(1., 1., 1.));

    let result = phong_lighting(&material, &light, position, position, eye, normal, false);

    assert_eq!(result, rgb(0.1, 0.1, 0.1));
  }

  #[test]
  fn calculate_lighting_data_for_an_intersection() {
    let ray = Ray::new(point(0., 0., -5.), vec3(0., 0., 1.));
    let sphere = Sphere::new();
    let intersection = Intersection::new(&sphere, 4.);

    let data = LightingData::calculate(&intersection, ray);

    assert_eq!(data.world_position, point(0., 0., -1.));
    assert_eq!(data.eye, vec3(0., 0., -1.));
    assert_eq!(data.normal, vec3(0., 0., -1.));
  }

  #[test]
  fn calculate_lighting_data_determines_outside() {
    let ray = Ray::new(point(0., 0., -5.), vec3(0., 0., 1.));
    let sphere = Sphere::new();
    let intersection = Intersection::new(&sphere, 4.);

    let data = LightingData::calculate(&intersection, ray);

    assert_eq!(data.inside, false);
  }

  #[test]
  fn calculate_lighting_data_determines_inside() {
    let ray = Ray::new(point(0., 0., 0.), vec3(0., 0., 1.));
    let sphere = Sphere::new();
    let intersection = Intersection::new(&sphere, 1.);

    let data = LightingData::calculate(&intersection, ray);

    assert_eq!(data.world_position, point(0., 0., 1.));
    assert_eq!(data.eye, vec3(0., 0., -1.));
    assert_eq!(data.normal, vec3(0., 0., -1.));
    assert_eq!(data.inside, true);
  }

  #[test]
  fn calculate_lighting_data_adds_point_in_direction_of_normal() {
    let ray = Ray::new(point(0., 0., -5.), vec3(0., 0., 1.));
    let sphere = Sphere::new().with_transform(Matrix4x4::translate(0., 0., 1.));
    let intersection = Intersection::new(&sphere, 5.);

    let data = LightingData::calculate(&intersection, ray);

    assert!(data.world_position_bias.z < EPSILON / 2.);
    assert!(data.world_position.z > data.world_position_bias.z);
  }

  #[test]
  fn lighting_with_surface_in_shadow() {
    let material = Material::default();
    let position = point(0., 0., 0.);

    let eye = vec3(0., 0., -1.);
    let normal = vec3(0., 0., -1.);

    let light = PointLight::new(vec3(0., 0., -10.), rgb(1., 1., 1.));

    let color = phong_lighting(&material, &light, position, position, eye, normal, true);

    assert_eq!(color, rgb(0.1, 0.1, 0.1));
  }

  #[test]
  fn calculate_lighting_data_computes_reflection_vector() {
    let ray = Ray::new(point(0., 1., -1.), vec3(0., -2f64.sqrt() / 2., 2f64.sqrt() / 2.));
    let plane = Plane::new(vec3(0., 1., 0.));
    let intersection = Intersection::new(&plane, 1.);

    let data = LightingData::calculate(&intersection, ray);

    assert_eq!(data.reflect_direction, vec3(0., 2f64.sqrt() / 2., 2f64.sqrt() / 2.));
  }
}
