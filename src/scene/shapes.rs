//! Shape rendering.

pub use planes::*;
pub use spheres::*;

use crate::maths::{Matrix4x4, Ray, Vector};

mod planes;
mod spheres;

/// A shape in 3-space that can compute ray intersection and normals.
pub trait Shape {
  /// Computes the distances at which the given ray intersects the shape.
  fn intersect(&self, object_ray: Ray) -> Vec<f32>;

  /// Computes the normal vector at a given object point on the surface of the object.
  fn normal_at(&self, object_point: Vector, inverse_transform: Matrix4x4) -> Vector;
}