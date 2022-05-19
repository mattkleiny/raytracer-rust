//! Standard mathematics and related linear algebra for implementing our ray tracer.

pub use tuples::*;

mod tuples;

/// Determines that two values are approximately equal.
pub trait ApproxEq<Rhs = Self> {
  fn is_approx(&self, rhs: Rhs) -> bool;
}

impl ApproxEq for f32 {
  fn is_approx(&self, rhs: Self) -> bool {
    (self - rhs).abs() < f32::EPSILON
  }
}
