//! Scene management abstractions.

use std::ops::{Deref, DerefMut};

pub use spheres::*;

use crate::maths::Ray;

mod spheres;


/// Allows an object to calculate intersection information with a Ray.
pub trait Object {
  /// Calculates the distances of intersection for the given ray.
  fn intersect(&self, ray: &Ray) -> IntersectSet;
}

/// A set of intersections for a particular object.K
pub struct IntersectSet<'a> {
  // TODO: push this into individual intersections, instead?
  object: &'a dyn Object,
  hits: smallvec::SmallVec<[f32; 4]>,
}

impl<'a> IntersectSet<'a> {
  /// Creates a new intersection set for the given object.
  pub fn new(object: &'a dyn Object) -> Self {
    Self {
      object,
      hits: smallvec::SmallVec::new(),
    }
  }

  /// Finds the closest hit intersection.
  pub fn closest_hit(&self) -> Option<f32> {
    if self.hits.len() == 0 {
      return None;
    }

    // find the closest intersection
    let mut closest = 0.;

    for t in &self.hits {
      let t = *t; // ugly hack
      if t > 0. && t > closest {
        closest = t;
      }
    }

    Some(closest)
  }
}

impl<'a> Deref for IntersectSet<'a> {
  type Target = smallvec::SmallVec<[f32; 4]>;

  fn deref(&self) -> &Self::Target {
    &self.hits
  }
}

impl<'a> DerefMut for IntersectSet<'a> {
  fn deref_mut(&mut self) -> &mut Self::Target {
    &mut self.hits
  }
}

#[cfg(test)]
mod tests {
  use crate::maths::point;

  use super::*;

  #[test]
  fn intersection_set_should_return_closest_hit() {
    let sphere = &Sphere::new(point(0., 0., -5.), 1.);
    let mut set = IntersectSet::new(sphere);

    set.push(1.);
    set.push(2.);

    assert_eq!(set.closest_hit(), Some(1.));
  }

  #[test]
  fn intersection_set_should_ignore_negative_t() {
    let sphere = &Sphere::new(point(0., 0., -5.), 1.);
    let mut set = IntersectSet::new(sphere);

    set.push(-1.);
    set.push(1.);

    assert_eq!(set.closest_hit(), Some(1.));
  }

  #[test]
  fn intersection_set_should_return_nothing_when_all_negative() {
    let sphere = &Sphere::new(point(0., 0., -5.), 1.);
    let mut set = IntersectSet::new(sphere);

    set.push(-2.);
    set.push(-1.);

    assert_eq!(set.closest_hit(), None);
  }

  #[test]
  fn intersection_set_should_always_return_lowest_non_negative_hit() {
    let sphere = &Sphere::new(point(0., 0., -5.), 1.);
    let mut set = IntersectSet::new(sphere);

    set.push(5.);
    set.push(7.);
    set.push(-3.);
    set.push(2.);

    assert_eq!(set.closest_hit(), Some(2.));
  }
}