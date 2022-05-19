//! Scene management abstractions.

use std::ops::{Deref, DerefMut};

pub use lighting::*;
pub use materials::*;
pub use spheres::*;

use crate::maths::{Color, Ray, Vector};

mod lighting;
mod materials;
mod spheres;

/// A scene that can be rendered via ray tracing.
pub struct Scene {
  background_color: Color,
  objects: Vec<Box<dyn SceneObject>>,
  point_lights: Vec<PointLight>,
}

impl Scene {
  /// Create a new scene.
  pub fn new() -> Self {
    Self {
      background_color: Color::BLACK,
      objects: Vec::new(),
      point_lights: Vec::new(),
    }
  }

  /// Add an object to the scene.
  pub fn add_object(&mut self, object: impl SceneObject + 'static) {
    self.objects.push(Box::new(object));
  }

  /// Add a point light to the scene.
  pub fn add_point_light(&mut self, light: PointLight) {
    self.point_lights.push(light);
  }

  /// Intersects the given ray with the entire scene.
  pub fn intersect(&self, ray: Ray) -> IntersectionSet {
    let mut results = IntersectionSet::new();

    for object in &self.objects {
      results.append(object.intersect(ray))
    }

    // sort results by distance in-place
    results.sort_by(|a, b| {
      a.distance.partial_cmp(&b.distance).unwrap()
    });

    results
  }
}

/// Allows an object to calculate intersection information with a Ray.
pub trait SceneObject {
  /// Returns the material for the object.
  fn material(&self) -> &Material;

  /// Calculates the distances of intersection for the given ray.
  fn intersect(&self, ray: Ray) -> IntersectionSet;

  /// Computes the normal vector at a given world point on the surface of the object.
  fn normal_at(&self, world_point: Vector) -> Vector;
}

/// A single intersection in an intersect set.
pub struct Intersection<'a> {
  pub object: &'a dyn SceneObject,
  pub distance: f32,
}

impl<'a> Intersection<'a> {
  /// Creates a new intersection
  pub fn new(object: &'a dyn SceneObject, distance: f32) -> Self {
    Self { object, distance }
  }
}

/// A set of intersections for a particular object.
pub struct IntersectionSet<'a> {
  hits: Vec<Intersection<'a>>,
}

impl<'a> IntersectionSet<'a> {
  /// Creates a new intersection set.
  pub fn new() -> Self {
    Self {
      hits: Vec::new()
    }
  }

  /// Adds an intersection to the set.
  pub fn push(&mut self, object: &'a dyn SceneObject, distance: f32) {
    self.hits.push(Intersection::new(object, distance));
  }

  /// Appends all items from the given other set to this set.
  pub fn append(&mut self, mut other: Self) {
    self.hits.append(&mut other.hits);
  }

  /// Finds the closest hit intersection.
  pub fn closest_hit(&self) -> Option<Intersection<'a>> {
    let mut closest_t = f32::MAX;
    let mut closest_object = None;

    for hit in &self.hits {
      let t = hit.distance;
      if t > 0. && t < closest_t {
        closest_t = t;
        closest_object = Some(hit.object);
      }
    }

    closest_object.map(|object| Intersection::new(object, closest_t))
  }
}

impl<'a> Deref for IntersectionSet<'a> {
  type Target = Vec<Intersection<'a>>;

  fn deref(&self) -> &Self::Target {
    &self.hits
  }
}

impl<'a> DerefMut for IntersectionSet<'a> {
  fn deref_mut(&mut self) -> &mut Self::Target {
    &mut self.hits
  }
}

#[cfg(test)]
mod tests {
  use crate::maths::{Matrix4x4, rgb, vec3};

  use super::*;

  #[test]
  fn intersection_set_should_return_closest_hit() {
    let sphere = &Sphere::new()
        .with_transform(Matrix4x4::translate(0., 0., -5.));

    let mut set = IntersectionSet::new();

    set.push(sphere, 1.);
    set.push(sphere, 2.);

    assert_eq!(set.closest_hit().unwrap().distance, 1.);
  }

  #[test]
  fn intersection_set_should_ignore_negative_t() {
    let sphere = &Sphere::new()
        .with_transform(Matrix4x4::translate(0., 0., -5.));

    let mut set = IntersectionSet::new();

    set.push(sphere, -1.);
    set.push(sphere, 1.);

    assert_eq!(set.closest_hit().unwrap().distance, 1.);
  }

  #[test]
  fn intersection_set_should_return_nothing_when_all_negative() {
    let sphere = &Sphere::new()
        .with_transform(Matrix4x4::translate(0., 0., -5.));

    let mut set = IntersectionSet::new();

    set.push(sphere, -2.);
    set.push(sphere, -1.);

    assert!(set.closest_hit().is_none());
  }

  #[test]
  fn intersection_set_should_always_return_lowest_non_negative_hit() {
    let sphere = &Sphere::new()
        .with_transform(Matrix4x4::translate(0., 0., -5.));

    let mut set = IntersectionSet::new();

    set.push(sphere, 5.);
    set.push(sphere, 7.);
    set.push(sphere, -3.);
    set.push(sphere, 2.);

    assert_eq!(set.closest_hit().unwrap().distance, 2.);
  }

  #[test]
  fn intersect_scene_with_ray_should_return_all_intersections() {
    let scene = create_test_scene();
    let ray = Ray::new(vec3(0., 0., -5.), vec3(0., 0., 1.));

    let set = scene.intersect(ray);

    assert_eq!(set.len(), 4);
    assert_eq!(set[0].distance, 4.);
    assert_eq!(set[1].distance, 4.5);
    assert_eq!(set[2].distance, 5.5);
    assert_eq!(set[3].distance, 6.);
  }

  /// Creates a default scene with two spheres a single light source.
  fn create_test_scene() -> Scene {
    let mut scene = Scene::new();

    scene.add_point_light(PointLight::new(vec3(-10., 10., -10.), Color::WHITE));

    scene.add_object(
      Sphere::new()
          .with_material(Material::default()
              .with_color(rgb(0.8, 1., 0.6))
              .with_diffuse(0.7)
              .with_specular(0.2))
    );

    scene.add_object(
      Sphere::new()
          .with_transform(Matrix4x4::scale(0.5, 0.5, 0.5))
          .with_material(Material::default()
              .with_color(rgb(0.8, 1., 0.6))
              .with_diffuse(0.7)
              .with_specular(0.2))
    );

    scene
  }
}