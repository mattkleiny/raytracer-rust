//! Sphere objects for use in scene rendering.

use crate::maths::{Matrix4x4, point, Ray, vec3, Vector};
use crate::scene::Material;

use super::{IntersectionSet, SceneObject};

/// A sphere in 3-space.
#[derive(Clone, Debug)]
pub struct Sphere {
  transform: Matrix4x4,
  material: Material,
}

impl Sphere {
  /// Creates a new sphere.
  pub fn new() -> Self {
    Self {
      transform: Matrix4x4::identity(),
      material: Material::default(),
    }
  }

  /// Sets the transform for this sphere.
  pub fn with_transform(self, transform: Matrix4x4) -> Self {
    Self {
      transform: self.transform * transform,
      ..self
    }
  }

  /// Sets the material for this sphere.
  pub fn with_material(self, material: Material) -> Self {
    Self { material, ..self }
  }
}

impl SceneObject for Sphere {
  fn material(&self) -> &Material {
    &self.material
  }

  fn intersect(&self, mut ray: Ray) -> IntersectionSet {
    if let Ok(inverse_transform) = self.transform.invert() {
      ray = inverse_transform * ray;
    }

    // standard ray sphere intersection
    let sphere_to_ray = ray.origin - point(0., 0., 0.);

    let a = ray.direction.dot(ray.direction);
    let b = 2. * sphere_to_ray.dot(ray.direction);
    let c = sphere_to_ray.dot(sphere_to_ray) - 1.;

    let discriminant = b * b - 4. * a * c;
    let mut results = IntersectionSet::new();

    if discriminant >= 0. {
      results.push(self, (-b - discriminant.sqrt()) / (2. * a));
      results.push(self, (-b + discriminant.sqrt()) / (2. * a));
    }

    results
  }

  fn normal_at(&self, world_point: Vector) -> Vector {
    if let Ok(inverse_transform) = self.transform.invert() {
      let object_point = inverse_transform * world_point;
      let object_normal = object_point - point(0., 0., 0.);
      let mut world_normal = inverse_transform.transpose() * object_normal;

      world_normal.w = 0.;

      world_normal.normalize()
    } else {
      vec3(0., 0., 0.)
    }
  }
}

#[cfg(test)]
mod tests {
  use crate::maths::{PI, point, vec3};

  use super::*;

  #[test]
  fn ray_should_intersect_sphere() {
    let ray = Ray::new(point(0., 0., -5.), vec3(0., 0., 1.));
    let sphere = Sphere::new();

    let set = sphere.intersect(ray);

    assert_eq!(set.len(), 2);
    assert_eq!(set[0].distance, 4.);
    assert_eq!(set[1].distance, 6.);
  }

  #[test]
  fn ray_should_intersect_sphere_at_tangent() {
    let ray = Ray::new(point(0., 1., -5.), vec3(0., 0., 1.));
    let sphere = Sphere::new();

    let set = sphere.intersect(ray);

    assert_eq!(set.len(), 2);
    assert_eq!(set[0].distance, 5.);
    assert_eq!(set[1].distance, 5.);
  }

  #[test]
  fn ray_should_miss_sphere() {
    let ray = Ray::new(point(0., 2., -5.), vec3(0., 0., 1.));
    let sphere = Sphere::new();

    let set = sphere.intersect(ray);

    assert_eq!(set.len(), 0);
  }

  #[test]
  fn ray_originates_inside_sphere() {
    let ray = Ray::new(point(0., 0., 0.), vec3(0., 0., 1.));
    let sphere = Sphere::new();

    let set = sphere.intersect(ray);

    assert_eq!(set.len(), 2);
    assert_eq!(set[0].distance, -1.);
    assert_eq!(set[1].distance, 1.);
  }

  #[test]
  fn ray_originates_in_front_of_sphere() {
    let ray = Ray::new(point(0., 0., 5.), vec3(0., 0., 1.));
    let sphere = Sphere::new();

    let set = sphere.intersect(ray);

    assert_eq!(set.len(), 2);
    assert_eq!(set[0].distance, -6.);
    assert_eq!(set[1].distance, -4.);
  }

  #[test]
  fn sphere_default_transformation() {
    let sphere = Sphere::new();

    assert_eq!(sphere.transform, Matrix4x4::IDENTITY);
  }

  #[test]
  fn scaled_sphere_intersection_with_ray() {
    let ray = Ray::new(point(0., 0., -5.), vec3(0., 0., 1.));
    let sphere = Sphere::new().with_transform(Matrix4x4::scale(2., 2., 2.));

    let set = sphere.intersect(ray);

    assert_eq!(set.len(), 2);
    assert_eq!(set[0].distance, 3.);
    assert_eq!(set[1].distance, 7.);
  }

  #[test]
  fn translated_sphere_intersection_with_ray() {
    let ray = Ray::new(point(0., 0., -5.), vec3(0., 0., 1.));
    let sphere = Sphere::new().with_transform(Matrix4x4::translate(5., 0., 0.));

    let set = sphere.intersect(ray);

    assert_eq!(set.len(), 0);
  }

  #[test]
  fn normal_on_sphere_on_x_axis() {
    let sphere = Sphere::new();
    let normal = sphere.normal_at(point(1., 0., 0.));

    assert_eq!(normal, vec3(1., 0., 0.));
  }

  #[test]
  fn normal_on_sphere_on_y_axis() {
    let sphere = Sphere::new();
    let normal = sphere.normal_at(point(0., 1., 0.));

    assert_eq!(normal, vec3(0., 1., 0.));
  }

  #[test]
  fn normal_on_sphere_on_z_axis() {
    let sphere = Sphere::new();
    let normal = sphere.normal_at(point(0., 0., 1.));

    assert_eq!(normal, vec3(0., 0., 1.));
  }

  #[test]
  fn normal_on_sphere_at_non_axial_point() {
    let sphere = Sphere::new();
    let point = point(3f32.sqrt() / 3., 3f32.sqrt() / 3., 3f32.sqrt() / 3.);
    let normal = sphere.normal_at(point);

    assert_eq!(
      normal,
      vec3(3f32.sqrt() / 3., 3f32.sqrt() / 3., 3f32.sqrt() / 3.)
    );
  }

  #[test]
  fn normal_on_sphere_is_normalised() {
    let sphere = Sphere::new();
    let point = point(3f32.sqrt() / 3., 3f32.sqrt() / 3., 3f32.sqrt() / 3.);
    let normal = sphere.normal_at(point);

    assert_eq!(normal.normalize(), normal);
  }

  #[test]
  fn normal_on_translated_sphere() {
    let sphere = Sphere::new().with_transform(Matrix4x4::translate(0., 1., 0.));

    let normal = sphere.normal_at(point(0., 1.70711, -0.70711));

    assert_eq!(normal, vec3(0., 0.70711, -0.70711));
  }

  #[test]
  fn normal_on_transformed_sphere() {
    let sphere = Sphere::new()
      .with_transform(Matrix4x4::scale(1., 0.5, 1.))
      .with_transform(Matrix4x4::rotate_z(PI / 5.));

    let normal = sphere.normal_at(point(0., 2f32.sqrt() / 2., -2f32.sqrt() / 2.));

    assert_eq!(normal, vec3(0., 0.97014, -0.24254));
  }
}
