//! Standard mathematics and related linear algebra for implementing our ray tracer.

pub use colors::*;
pub use matrices::*;
pub use rays::*;
pub use transforms::*;
pub use tuples::*;

mod colors;
mod matrices;
mod rays;
mod transforms;
mod tuples;

const EPSILON: f32 = 0.00001;

/// Determines that two values are approximately equal.
pub trait ApproxEq<Rhs = Self> {
  fn is_approx(&self, rhs: Rhs) -> bool;
}

impl ApproxEq for f32 {
  fn is_approx(&self, rhs: Self) -> bool {
    (self - rhs).abs() < EPSILON
  }
}