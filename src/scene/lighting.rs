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
  pub over_position: Point,
  pub under_position: Point,
  pub object_position: Point,
  pub eye: Vector,
  pub normal: Vector,
  pub reflect_direction: Vector,
  pub distance: f64,
  pub inside: bool,
  pub refractivity: [f64; 2],
}

impl<'a> LightingData<'a> {
  /// Pre-computes the lighting data used in the phong model.
  pub fn calculate(ray: Ray, hit: &'a Hit, hits: &'a HitList) -> Self {
    let object = hit.object;

    // calculation positions, directions and normals
    let world_position = ray.position(hit.distance);
    let eye = -ray.direction;
    let distance = hit.distance;

    let mut normal = object.normal_at(world_position);

    let over_position = world_position + normal * 0.0001;
    let under_position = world_position - normal * 0.0001;
    let object_position = object.world_to_object(over_position);
    let reflect_direction = ray.direction.reflect(normal);

    // determine if we're inside the object
    let mut inside = false;
    if normal.dot(eye) < 0. {
      normal = -normal;
      inside = true;
    }

    // computes object refractivity
    let refractivity = Self::compute_refractivity(hit, hits);

    Self {
      object,
      world_position,
      over_position,
      under_position,
      object_position,
      eye,
      normal,
      reflect_direction,
      inside,
      distance,
      refractivity,
    }
  }

  /// Computes the refractive indices for hit objects.
  fn compute_refractivity(hit: &Hit, hits: &HitList) -> [f64; 2] {
    // determine first and last refractive indices
    let mut n1 = 0.;
    let mut n2 = 0.;

    // scan through containing objects
    let mut containers: Vec<&Hit> = Vec::new();

    for i in hits.iter() {
      // entering the object?
      if i == hit {
        n1 = containers
          .last()
          .map(|it| it.object.material().refractivity)
          .unwrap_or(1.);
      }

      if containers.contains(&i) {
        containers.retain(|it| *it != i);
      } else {
        containers.push(i);
      }

      // exiting the object?
      if i == hit {
        n2 = containers
          .last()
          .map(|it| it.object.material().refractivity)
          .unwrap_or(1.);

        break;
      }
    }

    [n1, n2]
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

    assert!(data.over_position.z < EPSILON / 2.);
    assert!(data.world_position.z > data.over_position.z);
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

  // TODO: fix these up?
  // #[test]
  fn calculate_lighting_data_finds_refractive_indices_at_various_intersections() {
    // build nodes
    let a = create_glass_sphere(1.5);
    let b = create_glass_sphere(2.0);
    let c = create_glass_sphere(2.5);

    let ray = Ray::new(point(0., 0., -4.), vec3(0., 0., 1.));

    // build hit list
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

    assert_eq!(data[0].refractivity, [1.0, 1.5]);
    assert_eq!(data[1].refractivity, [1.5, 2.0]);
    assert_eq!(data[2].refractivity, [2.0, 2.5]);
    assert_eq!(data[3].refractivity, [2.5, 2.5]);
    assert_eq!(data[4].refractivity, [2.5, 1.5]);
    assert_eq!(data[5].refractivity, [1.5, 1.0]);
  }

  #[test]
  fn calculate_lighting_data_under_point_is_just_below_surface() {
    let ray = Ray::new(point(0., 0., -5.), vec3(0., 0., 1.));
    let sphere = create_glass_sphere(1.)
      .with_transform(Matrix4x4::translate(0., 0., 1.));

    let mut hits = HitList::new();
    hits.push(&sphere, 5.);

    let data = LightingData::calculate(ray, &hits[0], &hits);

    assert!(data.under_position.z > f64::EPSILON / 2.);
    assert!(data.world_position.z < data.under_position.z);
  }

  fn create_glass_sphere(refractivity: f64) -> SceneNode<Sphere> {
    Sphere::new()
      .with_material(Material::default()
        .with_transparency(1.)
        .with_refractivity(refractivity))
  }
}
