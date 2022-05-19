//! Scene management abstractions.

use std::ops::{Deref, DerefMut};

pub use spheres::*;

use crate::maths::Ray;

mod spheres;


/// Allows an object to calculate intersection information with a Ray.
pub trait Object {
  /// Calculates the distances of intersection for the given ray.
  fn intersect(&self, ray: &Ray) -> IntersectionSet;
}

/// A set of intersections for a particular object.K
pub struct IntersectionSet<'a> {
  // TODO: push this into individual intersections, instead?
  object: &'a dyn Object,
  points: smallvec::SmallVec<[f32; 4]>,
}

impl<'a> IntersectionSet<'a> {
  /// Creates a new intersection set for the given object.
  pub fn new(object: &'a dyn Object) -> Self {
    Self {
      object,
      points: smallvec::SmallVec::new(),
    }
  }
}

impl<'a> Deref for IntersectionSet<'a> {
  type Target = smallvec::SmallVec<[f32; 4]>;

  fn deref(&self) -> &Self::Target {
    &self.points
  }
}

impl<'a> DerefMut for IntersectionSet<'a> {
  fn deref_mut(&mut self) -> &mut Self::Target {
    &mut self.points
  }
}