//! Sphere objects for use in scene rendering.

use crate::maths::{Matrix4x4, point, Ray, Tuple};

use super::{IntersectSet, Object};

/// A sphere in 3-space.
#[derive(Copy, Clone, Debug)]
pub struct Sphere {
  transform: Matrix4x4,
}

impl Sphere {
  /// Creates a new sphere.
  pub fn new(center: Tuple, radius: f32) -> Self {
    let translation = Matrix4x4::translate(center.x, center.y, center.z);
    let matrix = Matrix4x4::scale(radius, radius, radius);

    Self { transform: translation * matrix }
  }
}

impl Object for Sphere {
  fn intersect(&self, mut ray: Ray) -> IntersectSet {
    if let Ok(inverse_transform) = self.transform.invert() {
      ray = inverse_transform * ray;
    }

    let sphere_to_ray = ray.origin - point(0., 0., 0.);

    let a = ray.direction.dot(&ray.direction);
    let b = 2. * sphere_to_ray.dot(&ray.direction);
    let c = sphere_to_ray.dot(&sphere_to_ray) - 1.;

    let discriminant = b * b - 4. * a * c;
    let mut results = IntersectSet::new(self);

    if discriminant >= 0. {
      results.push((-b - discriminant.sqrt()) / (2. * a));
      results.push((-b + discriminant.sqrt()) / (2. * a));
    }

    results
  }
}

#[cfg(test)]
mod tests {
  use crate::maths::{point, vec};

  use super::*;

  #[test]
  fn ray_should_intersect_sphere() {
    let ray = Ray::new(point(0., 0., -5.), vec(0., 0., 1.));
    let sphere = Sphere::new(point(0., 0., 0.), 1.);

    let set = sphere.intersect(ray);

    assert_eq!(set.len(), 2);
    assert_eq!(set[0], 4.);
    assert_eq!(set[1], 6.);
  }

  #[test]
  fn ray_should_intersect_sphere_at_tangent() {
    let ray = Ray::new(point(0., 1., -5.), vec(0., 0., 1.));
    let sphere = Sphere::new(point(0., 0., 0.), 1.);

    let set = sphere.intersect(ray);

    assert_eq!(set.len(), 2);
    assert_eq!(set[0], 5.);
    assert_eq!(set[1], 5.);
  }

  #[test]
  fn ray_should_miss_sphere() {
    let ray = Ray::new(point(0., 2., -5.), vec(0., 0., 1.));
    let sphere = Sphere::new(point(0., 0., 0.), 1.);

    let set = sphere.intersect(ray);

    assert_eq!(set.len(), 0);
  }

  #[test]
  fn ray_originates_inside_sphere() {
    let ray = Ray::new(point(0., 0., 0.), vec(0., 0., 1.));
    let sphere = Sphere::new(point(0., 0., 0.), 1.);

    let set = sphere.intersect(ray);

    assert_eq!(set.len(), 2);
    assert_eq!(set[0], -1.);
    assert_eq!(set[1], 1.);
  }

  #[test]
  fn ray_originates_in_front_of_sphere() {
    let ray = Ray::new(point(0., 0., 5.), vec(0., 0., 1.));
    let sphere = Sphere::new(point(0., 0., 0.), 1.);

    let set = sphere.intersect(ray);

    assert_eq!(set.len(), 2);
    assert_eq!(set[0], -6.);
    assert_eq!(set[1], -4.);
  }

  #[test]
  fn sphere_default_transformation() {
    let sphere = Sphere::new(point(0., 0., 0.), 1.);

    assert_eq!(sphere.transform, Matrix4x4::IDENTITY);
  }

  #[test]
  fn scaled_sphere_intersection_with_ray() {
    let ray = Ray::new(point(0., 0., -5.), vec(0., 0., 1.));
    let sphere = Sphere::new(point(0., 0., 0.), 2.);

    let set = sphere.intersect(ray);

    assert_eq!(set.len(), 2);
    assert_eq!(set[0], 3.);
    assert_eq!(set[1], 7.);
  }

  #[test]
  fn translated_sphere_intersection_with_ray() {
    let ray = Ray::new(point(0., 0., -5.), vec(0., 0., 1.));
    let sphere = Sphere::new(point(5., 0., 0.), 1.);

    let set = sphere.intersect(ray);

    assert_eq!(set.len(), 0);
  }
}