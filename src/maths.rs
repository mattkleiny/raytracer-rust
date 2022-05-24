//! Standard mathematical tools.

pub use std::f64::consts::PI as PI;

pub use colors::*;
pub use matrices::*;
pub use rays::*;
pub use transforms::*;
pub use vectors::*;

mod colors;
mod matrices;
mod rays;
mod transforms;
mod vectors;

pub const EPSILON: f64 = 0.00001;

/// Determines that two values are approximately equal.
pub trait ApproxEq<Rhs = Self> {
  fn is_approx(&self, rhs: Rhs) -> bool;
}

impl ApproxEq for f64 {
  fn is_approx(&self, rhs: Self) -> bool {
    (self - rhs).abs() < EPSILON
  }
}
