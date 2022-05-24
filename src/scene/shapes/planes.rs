//! Plane objects for use in scene rendering.

use crate::maths::{Matrix4x4, Ray, Vector};
use crate::scene::SceneNode;

use super::Shape;

/// A plane in 3-space.
#[derive(Clone, Debug)]
pub struct Plane {
  pub normal: Vector,
}

impl Plane {
  /// Constructs a new plane node
  pub fn new(normal: Vector) -> SceneNode<Self> {
    SceneNode::new(Self { normal })
  }
}

impl Shape for Plane {
  fn intersect(&self, object_ray: Ray) -> Vec<f64> {
    if object_ray.direction.y.abs() < 0.00001 {
      vec![]
    } else {
      vec![-object_ray.origin.y / object_ray.direction.y]
    }
  }

  fn normal_at(&self, _object_point: Vector, _inverse_transform: Matrix4x4) -> Vector {
    self.normal
  }
}

#[cfg(test)]
mod tests {
  use crate::maths::{point, vec3};
  use crate::scene::Traceable;

  use super::*;

  #[test]
  fn the_normal_of_a_plane_is_constant_everywhere() {
    let plane = Plane::new(vec3(0., 1., 0.));

    let n1 = plane.normal_at(point(0., 0., 0.));
    let n2 = plane.normal_at(point(10., 0., 0.));
    let n3 = plane.normal_at(point(-5., 0., 0.));

    assert_eq!(n1, vec3(0., 1., 0.));
    assert_eq!(n2, vec3(0., 1., 0.));
    assert_eq!(n3, vec3(0., 1., 0.));
  }

  #[test]
  fn intersect_with_a_ray_parallel_to_the_plane() {
    let plane = Plane::new(vec3(0., 1., 0.));
    let ray = Ray::new(point(0., 10., 0.), vec3(0., 0., 1.));

    let set = plane.intersect(ray);

    assert_eq!(set.len(), 0);
  }

  #[test]
  fn intersect_with_a_coplanar_ray() {
    let plane = Plane::new(vec3(0., 1., 0.));
    let ray = Ray::new(point(0., 0., 0.), vec3(0., 0., 1.));

    let set = plane.intersect(ray);

    assert_eq!(set.len(), 0);
  }

  #[test]
  fn intersect_with_ray_from_above() {
    let plane = Plane::new(vec3(0., 1., 0.));
    let ray = Ray::new(point(0., 1., 0.), vec3(0., -1., 0.));

    let set = plane.intersect(ray);

    assert_eq!(set.len(), 1);
    assert_eq!(set[0].distance, 1.);
  }

  #[test]
  fn intersect_with_ray_from_below() {
    let plane = Plane::new(vec3(0., 1., 0.));
    let ray = Ray::new(point(0., -1., 0.), vec3(0., 1., 0.));

    let set = plane.intersect(ray);

    assert_eq!(set.len(), 1);
    assert_eq!(set[0].distance, 1.);
  }
}