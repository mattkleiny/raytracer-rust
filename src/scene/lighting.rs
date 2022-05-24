//! Light sources for scene rendering.

use crate::maths::{Color, Point, Ray, Vector};
use crate::scene::{Hit, HitList, Material, Traceable};

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
  pub refractive_indices: [f64; 2],
}

impl<'a> LightingData<'a> {
  /// Pre-computes the lighting data used in the phong model.
  pub fn calculate(ray: Ray, main_hit: &'a Hit, hits: &'a HitList) -> Self {
    let object = main_hit.object;

    // calculation positions, directions and normals
    let world_position = ray.position(main_hit.distance);
    let eye = -ray.direction;
    let distance = main_hit.distance;

    let mut normal = object.normal_at(world_position);

    let world_position_bias = world_position + normal * 0.0001;
    let object_position = object.world_to_object(world_position_bias);
    let reflect_direction = ray.direction.reflect(normal);

    // determine if we're inside the object
    let mut inside = false;

    if normal.dot(eye) < 0. {
      normal = -normal;
      inside = true;
    }

    // determine first and last refractive indices
    let [mut n1, mut n2] = [1., 1.];
    let mut containers: Vec<&Hit> = Vec::new();

    for hit in hits.iter() {
      if hit == main_hit {
        n1 = 1.0;
      } else {
        if let Some(hit) = containers.last() {
          n1 = hit.object.material().refractivity;
        }
      }

      if containers.contains(&hit) {
        containers.retain(|it| *it != hit);
      } else {
        containers.push(hit);
      }

      if hit == main_hit {
        if containers.is_empty() {
          n2 = 1.0;
        } else {
          if let Some(hit) = containers.last() {
            n2 = hit.object.material().refractivity;
          }
        }

        break;
      }
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
      refractive_indices: [n1, n2],
    }
  }
}

/// Computes lighting for a particular point in the scene via phong model.
pub fn phong_lighting(light: &PointLight, material: &Material, world_position: Vector, object_position: Vector, eye: Vector, normal: Vector, in_shadow: bool) -> Color {
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
  use crate::scene::{HitList, Plane, SceneNode, Sphere};

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

    let result = phong_lighting(&light, &material, position, position, eye, normal, false);

    assert_eq!(result, rgb(1.9, 1.9, 1.9));
  }

  #[test]
  fn phong_lighting_with_eye_between_light_and_surface_offset_45_degrees() {
    let material = Material::default();
    let position = vec3(0., 0., 0.);
    let eye = vec3(0., 2f64.sqrt() / 2., 2f64.sqrt() / 2.);
    let normal = vec3(0., 0., -1.);
    let light = PointLight::new(vec3(0., 0., -10.), rgb(1., 1., 1.));

    let result = phong_lighting(&light, &material, position, position, eye, normal, false);

    assert_eq!(result, rgb(1.0, 1.0, 1.0));
  }

  #[test]
  fn phong_lighting_with_eye_opposite_surface_light_offset_45_degrees() {
    let material = Material::default();
    let position = vec3(0., 0., 0.);
    let eye = vec3(0., 0., -1.);
    let normal = vec3(0., 0., -1.);
    let light = PointLight::new(vec3(0., 10., -10.), rgb(1., 1., 1.));

    let result = phong_lighting(&light, &material, position, position, eye, normal, false);

    assert_eq!(result, rgb(0.7364, 0.7364, 0.7364));
  }

  #[test]
  fn phong_lighting_with_eye_in_the_path_of_the_reflection_vector() {
    let material = Material::default();
    let position = vec3(0., 0., 0.);
    let eye = vec3(0., -2f64.sqrt() / 2., -2f64.sqrt() / 2.);
    let normal = vec3(0., 0., -1.);
    let light = PointLight::new(vec3(0., 10., -10.), rgb(1., 1., 1.));

    let result = phong_lighting(&light, &material, position, position, eye, normal, false);

    assert_eq!(result, rgb(1.6363961030678928, 1.6363961030678928, 1.6363961030678928));
  }

  #[test]
  fn phong_lighting_with_light_behind_the_surface() {
    let material = Material::default();
    let position = vec3(0., 0., 0.);
    let eye = vec3(0., 0., -1.);
    let normal = vec3(0., 0., -1.);
    let light = PointLight::new(vec3(0., 0., 10.), rgb(1., 1., 1.));

    let result = phong_lighting(&light, &material, position, position, eye, normal, false);

    assert_eq!(result, rgb(0.1, 0.1, 0.1));
  }

  #[test]
  fn calculate_lighting_data_for_an_intersection() {
    let ray = Ray::new(point(0., 0., -5.), vec3(0., 0., 1.));
    let sphere = Sphere::new();

    let hit = Hit::new(&sphere, 4.);
    let hits = HitList::from(&[hit]);

    let data = LightingData::calculate(ray, &hits[0], &hits);

    assert_eq!(data.world_position, point(0., 0., -1.));
    assert_eq!(data.eye, vec3(0., 0., -1.));
    assert_eq!(data.normal, vec3(0., 0., -1.));
  }

  #[test]
  fn calculate_lighting_data_determines_outside() {
    let ray = Ray::new(point(0., 0., -5.), vec3(0., 0., 1.));
    let sphere = Sphere::new();

    let hit = Hit::new(&sphere, 4.);
    let hits = HitList::from(&[hit]);

    let data = LightingData::calculate(ray, &hits[0], &hits);

    assert_eq!(data.inside, false);
  }

  #[test]
  fn calculate_lighting_data_determines_inside() {
    let ray = Ray::new(point(0., 0., 0.), vec3(0., 0., 1.));
    let sphere = Sphere::new();

    let hit = Hit::new(&sphere, 1.);
    let hits = HitList::from(&[hit]);

    let data = LightingData::calculate(ray, &hits[0], &hits);

    assert_eq!(data.world_position, point(0., 0., 1.));
    assert_eq!(data.eye, vec3(0., 0., -1.));
    assert_eq!(data.normal, vec3(0., 0., -1.));
    assert_eq!(data.inside, true);
  }

  #[test]
  fn calculate_lighting_data_adds_point_in_direction_of_normal() {
    let ray = Ray::new(point(0., 0., -5.), vec3(0., 0., 1.));
    let sphere = Sphere::new().with_transform(Matrix4x4::translate(0., 0., 1.));

    let hit = Hit::new(&sphere, 5.);
    let hits = HitList::from(&[hit]);

    let data = LightingData::calculate(ray, &hits[0], &hits);

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

    let color = phong_lighting(&light, &material, position, position, eye, normal, true);

    assert_eq!(color, rgb(0.1, 0.1, 0.1));
  }

  #[test]
  fn calculate_lighting_data_computes_reflection_vector() {
    let ray = Ray::new(point(0., 1., -1.), vec3(0., -2f64.sqrt() / 2., 2f64.sqrt() / 2.));
    let plane = Plane::new(vec3(0., 1., 0.));

    let hit = Hit::new(&plane, 1.);
    let hits = HitList::from(&[hit]);

    let data = LightingData::calculate(ray, &hits[0], &hits);

    assert_eq!(data.reflect_direction, vec3(0., 2f64.sqrt() / 2., 2f64.sqrt() / 2.));
  }

  #[test]
  fn calculate_lighting_data_finds_refractive_indices_at_various_intersections() {
    fn create_glass_sphere(refractive: f64) -> SceneNode<Sphere> {
      Sphere::new()
        .with_material(Material::default()
          .with_transparency(1.)
          .with_refractive_index(refractive))
    }

    let a = create_glass_sphere(1.5).with_transform(Matrix4x4::scale(2., 2., 2.));
    let b = create_glass_sphere(2.0).with_transform(Matrix4x4::translate(0., 0., -0.25));
    let c = create_glass_sphere(2.5).with_transform(Matrix4x4::translate(0., 0., 0.25));

    let ray = Ray::new(point(0., 0., -4.), vec3(0., 0., 1.));
    let mut hits = HitList::new();

    hits.push(&a, 2.);
    hits.push(&b, 2.75);
    hits.push(&c, 3.25);
    hits.push(&b, 4.75);
    hits.push(&c, 5.25);
    hits.push(&a, 6.);

    // compute lighting data for each intersection
    let data: Vec<_> = hits
      .iter()
      .map(|hit| LightingData::calculate(ray, hit, &hits))
      .collect();

    assert_eq!(data[0].refractive_indices, [1.0, 1.5]);
    assert_eq!(data[1].refractive_indices, [1.5, 2.0]);
    assert_eq!(data[2].refractive_indices, [2.0, 2.5]);
    assert_eq!(data[3].refractive_indices, [2.5, 2.5]);
    assert_eq!(data[4].refractive_indices, [2.5, 1.5]);
    assert_eq!(data[5].refractive_indices, [1.5, 1.0]);
  }
}
