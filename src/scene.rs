//! Scene management abstractions.

use std::ops::{Deref, DerefMut};

pub use cameras::*;
pub use lighting::*;
pub use materials::*;
pub use shapes::*;

use crate::maths::{ApproxEq, Color, Matrix4x4, Point, Ray, vec3, Vector};

mod cameras;
mod lighting;
mod materials;
mod shapes;

/// An object in the scene that can be ray-traced.
pub trait Traceable {
  /// Returns the material for the object.
  fn material(&self) -> &Material;

  /// Calculates the distances of intersection for the given ray.
  fn intersect(&self, world_ray: Ray) -> IntersectionSet;

  /// Computes the normal vector at a given world point on the surface of the object.
  fn normal_at(&self, world_point: Vector) -> Vector;

  /// Transforms the given world point to object space.
  fn world_to_object(&self, world_point: Vector) -> Vector;

  /// Transforms the given object point to world space.
  fn object_to_world(&self, object_point: Vector) -> Vector;
}

/// A node in a scene with associated material and transform.
pub struct SceneNode<S> {
  object: S,
  material: Material,
  transform: Matrix4x4,
}

impl<S> SceneNode<S> {
  /// Creates a new node.
  pub fn new(object: S) -> Self {
    Self {
      object,
      transform: Matrix4x4::identity(),
      material: Material::default(),
    }
  }

  /// Sets the transform for this node.
  pub fn with_transform(self, transform: Matrix4x4) -> Self {
    Self { transform: self.transform * transform, ..self }
  }

  /// Sets the material for this node.
  pub fn with_material(self, material: Material) -> Self {
    Self { material, ..self }
  }
}

impl<S> Traceable for SceneNode<S> where S: Shape {
  fn material(&self) -> &Material {
    &self.material
  }

  fn intersect(&self, world_ray: Ray) -> IntersectionSet {
    let mut results = IntersectionSet::new();

    if let Ok(inverse) = self.transform.invert() {
      for distance in self.object.intersect(inverse * world_ray) {
        results.push(self, distance);
      }
    }

    results
  }

  fn normal_at(&self, world_point: Vector) -> Vector {
    if let Ok(inverse) = self.transform.invert() {
      self.object.normal_at(world_point, inverse)
    } else {
      vec3(0., 0., 0.)
    }
  }

  fn world_to_object(&self, world_point: Vector) -> Vector {
    if let Ok(inverse) = self.transform.invert() {
      inverse * world_point
    } else {
      world_point
    }
  }

  fn object_to_world(&self, object_point: Vector) -> Vector {
    self.transform * object_point
  }
}

/// A scene that can be rendered via ray tracing.
pub struct Scene {
  ambient_color: Color,
  nodes: Vec<Box<dyn Traceable>>,
  lights: Vec<PointLight>,
}

impl Scene {
  const MAX_DEPTH: usize = 5;

  /// Create a new scene.
  pub fn new() -> Self {
    Self {
      ambient_color: Color::BLACK,
      nodes: Vec::new(),
      lights: Vec::new(),
    }
  }

  /// Add an object to the scene.
  pub fn add_object(&mut self, object: impl Traceable + 'static) {
    self.nodes.push(Box::new(object));
  }

  /// Add a point light to the scene.
  pub fn add_light(&mut self, light: PointLight) {
    self.lights.push(light);
  }

  /// Computes the color of the scene at the given ray.
  pub fn trace(&self, ray: Ray) -> Color {
    self.trace_inner(ray, 0)
  }

  /// Computes the color of the scene at the given ray.
  fn trace_inner(&self, ray: Ray, depth: usize) -> Color {
    if depth > Self::MAX_DEPTH {
      return self.ambient_color;
    }

    if let Some(intersection) = self.intersect(ray).closest_hit() {
      self.apply_lighting(ray, intersection, depth)
    } else {
      self.ambient_color
    }
  }

  /// Intersects the given ray with the entire scene.
  fn intersect(&self, ray: Ray) -> IntersectionSet {
    let mut results = IntersectionSet::new();

    for object in &self.nodes {
      results.append(object.intersect(ray))
    }

    // sort results by distance in-place
    results.sort_by(|a, b| {
      a.distance.partial_cmp(&b.distance).unwrap()
    });

    results
  }

  /// Calculates lighting for the given ray intersection.
  fn apply_lighting(&self, ray: Ray, intersection: Intersection, depth: usize) -> Color {
    let mut surface = self.ambient_color;

    let lighting_data = calculate_lighting_data(&intersection, ray);
    let in_shadow = self.is_shadowed(lighting_data.world_position_bias);

    // calculate direct surface lighting
    for light in &self.lights {
      surface = surface + phong_lighting(
        &lighting_data.object.material(),
        light,
        lighting_data.world_position_bias,
        lighting_data.object_position,
        lighting_data.eye,
        lighting_data.normal,
        in_shadow,
      );
    }

    // calculate reflective properties
    let reflected = self.reflected_color(ray, intersection, depth);

    surface + reflected
  }

  /// Determines if the given point is in shadow.
  fn is_shadowed(&self, point: Point) -> bool {
    for light in &self.lights {
      let light_vector = light.position - point;

      let distance = light_vector.magnitude();
      let direction = light_vector.normalize();

      let ray = Ray::new(point, direction);

      if let Some(intersection) = self.intersect(ray).closest_hit() {
        if intersection.distance < distance {
          return true;
        }
      }
    }

    false
  }

  /// Determines the reflected color of the given ray.
  fn reflected_color(&self, ray: Ray, intersection: Intersection, depth: usize) -> Color {
    let material = intersection.object.material();

    if material.reflective.is_approx(0.) {
      return Color::BLACK;
    }

    // TODO: re-use this between two methods?
    let lighting_data = calculate_lighting_data(&intersection, ray);

    let reflect_ray = Ray::new(
      lighting_data.world_position_bias,
      lighting_data.reflect_direction,
    );

    self.trace_inner(reflect_ray, depth + 1) * material.reflective
  }
}

/// A single intersection in an set.
pub struct Intersection<'a> {
  pub object: &'a dyn Traceable,
  pub distance: f32,
}

impl<'a> Intersection<'a> {
  /// Creates a new intersection
  pub fn new(object: &'a dyn Traceable, distance: f32) -> Self {
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
    Self { hits: Vec::new() }
  }

  /// Adds an intersection to the set.
  pub fn push(&mut self, object: &'a dyn Traceable, distance: f32) {
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

    closest_object.map(|object| {
      Intersection::new(object, closest_t)
    })
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
  use crate::maths::{Matrix4x4, point, rgb, vec3};

  use super::*;

  #[test]
  fn intersection_set_should_return_closest_hit() {
    let sphere = &Sphere::new().with_transform(Matrix4x4::translate(0., 0., -5.));

    let mut set = IntersectionSet::new();

    set.push(sphere, 1.);
    set.push(sphere, 2.);

    assert_eq!(set.closest_hit().unwrap().distance, 1.);
  }

  #[test]
  fn intersection_set_should_ignore_negative_t() {
    let sphere = &Sphere::new().with_transform(Matrix4x4::translate(0., 0., -5.));

    let mut set = IntersectionSet::new();

    set.push(sphere, -1.);
    set.push(sphere, 1.);

    assert_eq!(set.closest_hit().unwrap().distance, 1.);
  }

  #[test]
  fn intersection_set_should_return_nothing_when_all_negative() {
    let sphere = &Sphere::new().with_transform(Matrix4x4::translate(0., 0., -5.));

    let mut set = IntersectionSet::new();

    set.push(sphere, -2.);
    set.push(sphere, -1.);

    assert!(set.closest_hit().is_none());
  }

  #[test]
  fn intersection_set_should_always_return_lowest_non_negative_hit() {
    let sphere = &Sphere::new().with_transform(Matrix4x4::translate(0., 0., -5.));

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
    let ray = Ray::new(point(0., 0., -5.), vec3(0., 0., 1.));

    let set = scene.intersect(ray);

    assert_eq!(set.len(), 4);
    assert_eq!(set[0].distance, 4.);
    assert_eq!(set[1].distance, 4.5);
    assert_eq!(set[2].distance, 5.5);
    assert_eq!(set[3].distance, 6.);
  }

  #[test]
  fn apply_lighting_to_an_intersection_from_outside() {
    let scene = create_test_scene();

    let ray = Ray::new(point(0., 0., -5.), vec3(0., 0., 1.));
    let object = scene.nodes[0].deref();
    let intersection = Intersection::new(object.deref(), 4.);

    let color = scene.apply_lighting(ray, intersection, 0);

    assert_eq!(color, rgb(0.38012764, 0.47515953, 0.28509575));
  }

  #[test]
  fn apply_lighting_to_an_intersection_from_inside() {
    let mut scene = create_test_scene();
    scene.lights[0] = PointLight::new(point(0., 0.25, 0.), rgb(1., 1., 1.));

    let ray = Ray::new(point(0., 0., 0.), vec3(0., 0., 1.));
    let object = scene.nodes[1].deref();
    let intersection = Intersection::new(object, 0.5);

    let color = scene.apply_lighting(ray, intersection, 0);

    assert_eq!(color, rgb(0.1, 0.1, 0.1));
  }

  #[test]
  fn background_color_is_used_when_ray_misses() {
    let mut scene = create_test_scene();
    let ray = Ray::new(point(0., 0., -5.), vec3(0., 1., 0.));

    scene.ambient_color = Color::RED;

    let color = scene.trace(ray);

    assert_eq!(color, Color::RED);
  }

  #[test]
  fn color_of_material_is_used_when_ray_hits() {
    let scene = create_test_scene();
    let ray = Ray::new(point(0., 0., -5.), vec3(0., 0., 1.));

    let color = scene.trace(ray);

    assert_eq!(color, rgb(0.38012764, 0.47515953, 0.28509575));
  }

  #[test]
  fn there_is_no_shadow_when_nothing_is_colinear_with_point_and_light() {
    let scene = create_test_scene();
    let point = point(0., 10., 10.);

    assert!(!scene.is_shadowed(point));
  }

  #[test]
  fn the_shadow_when_an_object_is_between_the_point_and_the_light() {
    let scene = create_test_scene();
    let point = point(10., -10., 10.);

    assert!(scene.is_shadowed(point));
  }

  #[test]
  fn there_is_no_shadow_when_an_object_is_behind_the_light() {
    let scene = create_test_scene();
    let point = point(-20., 20., -20.);

    assert!(!scene.is_shadowed(point));
  }

  #[test]
  fn there_is_no_shadow_when_an_object_is_behind_the_point() {
    let scene = create_test_scene();
    let point = point(-2., 2., -2.);

    assert!(!scene.is_shadowed(point));
  }

  #[test]
  fn apply_lighting_is_given_an_intersection_in_shadow() {
    let mut scene = Scene::new();

    scene.add_light(PointLight::new(point(0., 0., -10.), rgb(1., 1., 1.)));
    scene.add_object(Sphere::new());
    scene.add_object(Sphere::new().with_transform(Matrix4x4::translate(0., 0., 10.)));

    let ray = Ray::new(point(0., 0., 5.), vec3(0., 0., 1.));
    let intersection = Intersection::new(scene.nodes[1].deref(), 4.);

    let color = scene.apply_lighting(ray, intersection, 0);

    assert_eq!(color, rgb(0.1, 0.1, 0.1));
  }

  #[test]
  fn reflected_color_for_non_reflective_material() {
    let scene = create_test_scene();
    let ray = Ray::new(point(0., 0., 0.), vec3(0., 0., 1.));
    let object = scene.nodes[1].deref();

    let intersection = Intersection::new(object, -1.);

    let color = scene.reflected_color(ray, intersection, 0);

    assert_eq!(color, Color::BLACK);
  }

  #[test]
  fn reflected_color_for_reflective_material() {
    let mut scene = create_test_scene();

    scene.add_object(
      Plane::new(vec3(0., 1., 0.))
        .with_material(Material::default()
          .with_reflective(0.5))
        .with_transform(Matrix4x4::translate(0., -1., 0.)),
    );

    let ray = Ray::new(point(0., 0., -3.), vec3(0., -2f32.sqrt() / 2., 2f32.sqrt() / 2.));
    let object = scene.nodes[2].deref();

    let intersection = Intersection::new(object, 2f32.sqrt());

    let color = scene.reflected_color(ray, intersection, 0);

    assert_eq!(color, rgb(0.19007981, 0.23759975, 0.14255986));
  }

  /// Creates a default scene with two spheres a single light source.
  fn create_test_scene() -> Scene {
    let mut scene = Scene::new();

    scene.add_light(PointLight::new(vec3(-10., 10., -10.), Color::WHITE));

    scene.add_object(
      Sphere::new()
        .with_material(
          Material::default()
            .with_color(rgb(0.8, 1., 0.6))
            .with_diffuse(0.7)
            .with_specular(0.2)
        ),
    );

    scene.add_object(
      Sphere::new()
        .with_transform(Matrix4x4::scale(0.5, 0.5, 0.5))
    );

    scene
  }
}
