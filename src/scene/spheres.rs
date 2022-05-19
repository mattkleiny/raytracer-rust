//! Sphere objects for use in scene rendering.

use crate::maths::{Ray, Tuple};

use super::{IntersectSet, Object};

/// A sphere in 3-space.
#[derive(Copy, Clone, Debug)]
pub struct Sphere {
  center: Tuple,
  radius: f32,
}

impl Sphere {
  /// Creates a new sphere.
  pub const fn new(center: Tuple, radius: f32) -> Self {
    Self { center, radius }
  }
}

impl Object for Sphere {
  fn intersect(&self, ray: &Ray) -> IntersectSet {
    let sphere_to_ray = ray.origin - self.center;
    let radius_squared = self.radius * self.radius;

    let a = ray.direction.dot(&ray.direction);
    let b = 2. * sphere_to_ray.dot(&ray.direction);
    let c = sphere_to_ray.dot(&sphere_to_ray) - radius_squared;

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

    let set = sphere.intersect(&ray);

    assert_eq!(set.len(), 2);
    assert_eq!(set[0], 4.);
    assert_eq!(set[1], 6.);
  }

  #[test]
  fn ray_should_intersect_sphere_at_tangent() {
    let ray = Ray::new(point(0., 1., -5.), vec(0., 0., 1.));
    let sphere = Sphere::new(point(0., 0., 0.), 1.);

    let set = sphere.intersect(&ray);

    assert_eq!(set.len(), 2);
    assert_eq!(set[0], 5.);
    assert_eq!(set[1], 5.);
  }

  #[test]
  fn ray_should_miss_sphere() {
    let ray = Ray::new(point(0., 2., -5.), vec(0., 0., 1.));
    let sphere = Sphere::new(point(0., 0., 0.), 1.);

    let set = sphere.intersect(&ray);

    assert_eq!(set.len(), 0);
  }

  #[test]
  fn ray_originates_inside_sphere() {
    let ray = Ray::new(point(0., 0., 0.), vec(0., 0., 1.));
    let sphere = Sphere::new(point(0., 0., 0.), 1.);

    let set = sphere.intersect(&ray);

    assert_eq!(set.len(), 2);
    assert_eq!(set[0], -1.);
    assert_eq!(set[1], 1.);
  }

  #[test]
  fn ray_originates_in_front_of_sphere() {
    let ray = Ray::new(point(0., 0., 5.), vec(0., 0., 1.));
    let sphere = Sphere::new(point(0., 0., 0.), 1.);

    let set = sphere.intersect(&ray);

    assert_eq!(set.len(), 2);
    assert_eq!(set[0], -6.);
    assert_eq!(set[1], -4.);
  }
}