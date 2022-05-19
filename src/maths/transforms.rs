//! Transformation matrices for vectors and points.

use super::Matrix4x4;

impl Matrix4x4 {
  /// Creates a new translation matrix.
  pub fn translate(x: f32, y: f32, z: f32) -> Self {
    Self::create(&[
      1.0, 0.0, 0.0, x,
      0.0, 1.0, 0.0, y,
      0.0, 0.0, 1.0, z,
      0.0, 0.0, 0.0, 1.0,
    ])
  }

  /// Creates a new scale matrix.
  pub fn scale(x: f32, y: f32, z: f32) -> Self {
    Self::create(&[
      x, 0.0, 0.0, 0.0,
      0.0, y, 0.0, 0.0,
      0.0, 0.0, z, 0.0,
      0.0, 0.0, 0.0, 1.0,
    ])
  }

  /// Creates a new rotation matrix about the X axis.
  pub fn rotate_x(r: f32) -> Self {
    Self::create(&[
      1.0, 0.0, 0.0, 0.0,
      0.0, r.cos(), -r.sin(), 0.0,
      0.0, r.sin(), r.cos(), 0.0,
      0.0, 0.0, 0.0, 1.0,
    ])
  }

  /// Creates a new rotation matrix about the Y axis.
  pub fn rotate_y(r: f32) -> Self {
    Self::create(&[
      r.cos(), 0.0, r.sin(), 0.0,
      0.0, 1.0, 0.0, 0.0,
      -r.sin(), 0.0, r.cos(), 0.0,
      0.0, 0.0, 0.0, 1.0,
    ])
  }

  /// Creates a new rotation matrix about the Z axis.
  pub fn rotate_z(r: f32) -> Self {
    Self::create(&[
      r.cos(), -r.sin(), 0.0, 0.0,
      r.sin(), r.cos(), 0.0, 0.0,
      0.0, 0.0, 1.0, 0.0,
      0.0, 0.0, 0.0, 1.0,
    ])
  }

  /// Creates a new shearing matrix with the given proportions.
  pub fn shear(x1: f32, x2: f32, y1: f32, y2: f32, z1: f32, z2: f32) -> Self {
    Self::create(&[
      1.0, x1, x2, 0.0,
      y1, 1.0, y2, 0.0,
      z1, z2, 1.0, 0.0,
      0.0, 0.0, 0.0, 1.0,
    ])
  }
}

#[cfg(test)]
mod tests {
  use crate::maths::{point, vec3};

  use super::*;

  #[test]
  fn translation_should_transform_point() {
    let transform = Matrix4x4::translate(5.0, -3.0, 2.0);
    let p = point(-3.0, 4.0, 5.0);

    assert_eq!(transform * p, point(2.0, 1.0, 7.0));
  }

  #[test]
  fn inverse_translation_should_transform_point() {
    let transform = Matrix4x4::translate(5.0, -3.0, 2.0);
    let inverse = transform.invert().expect("Failed to invert");

    let p = point(-3.0, 4.0, 5.0);

    assert_eq!(inverse * p, point(-8.0, 7.0, 3.0));
  }

  #[test]
  fn translation_does_not_affect_vectors() {
    let transform = Matrix4x4::translate(5.0, -3.0, 2.0);
    let v = vec3(3.0, 4.0, 5.0);

    assert_eq!(transform * v, v);
  }

  #[test]
  fn scale_should_transform_point() {
    let transform = Matrix4x4::scale(2., 3., 4.);
    let p = point(-4., 6., 8.);

    assert_eq!(transform * p, point(-8., 18., 32.));
  }

  #[test]
  fn scale_should_transform_vector() {
    let transform = Matrix4x4::scale(2., 3., 4.);
    let p = vec3(-4., 6., 8.);

    assert_eq!(transform * p, vec3(-8., 18., 32.));
  }

  #[test]
  fn inverse_scale_should_transform_point() {
    let transform = Matrix4x4::scale(2., 3., 4.);
    let inverse = transform.invert().expect("Failed to invert");

    let p = point(-4., 6., 8.);

    assert_eq!(inverse * p, point(-2., 2., 2.));
  }

  #[test]
  fn scale_should_reflect_point() {
    let transform = Matrix4x4::scale(-1., 1., 1.);
    let p = point(2., 3., 4.);

    assert_eq!(transform * p, point(-2., 3., 4.));
  }

  #[test]
  fn rotate_around_x_axis() {
    let p = point(0., 1., 0.);

    let half_quarter = Matrix4x4::rotate_x(std::f32::consts::PI / 4.);
    let full_quarter = Matrix4x4::rotate_x(std::f32::consts::PI / 2.);

    assert_eq!(half_quarter * p, point(0., 2f32.sqrt() / 2., 2f32.sqrt() / 2.));
    assert_eq!(full_quarter * p, point(0., 0., 1.));
  }

  #[test]
  fn inverse_rotate_around_x_axis() {
    let p = point(0., 1., 0.);

    let half_quarter = Matrix4x4::rotate_x(std::f32::consts::PI / 4.);
    let inverse = half_quarter.invert().expect("Failed to invert");

    assert_eq!(inverse * p, point(0., 2f32.sqrt() / 2., -2f32.sqrt() / 2.));
  }

  #[test]
  fn rotate_around_y_axis() {
    let p = point(0., 0., 1.);

    let half_quarter = Matrix4x4::rotate_y(std::f32::consts::PI / 4.);
    let full_quarter = Matrix4x4::rotate_y(std::f32::consts::PI / 2.);

    assert_eq!(half_quarter * p, point(2f32.sqrt() / 2., 0., 2f32.sqrt() / 2.));
    assert_eq!(full_quarter * p, point(1., 0., 0.));
  }

  #[test]
  fn rotate_around_z_axis() {
    let p = point(0., 1., 0.);

    let half_quarter = Matrix4x4::rotate_z(std::f32::consts::PI / 4.);
    let full_quarter = Matrix4x4::rotate_z(std::f32::consts::PI / 2.);

    assert_eq!(half_quarter * p, point(-2f32.sqrt() / 2., 2f32.sqrt() / 2., 0.));
    assert_eq!(full_quarter * p, point(-1., 0., 0.));
  }

  #[test]
  fn shearing_should_move_x_in_proportion_to_y() {
    let transform = Matrix4x4::shear(1., 0., 0., 0., 0., 0.);
    let p = point(2., 3., 4.);

    assert_eq!(transform * p, point(5., 3., 4.));
  }

  #[test]
  fn shearing_should_move_x_in_proportion_to_z() {
    let transform = Matrix4x4::shear(0., 1., 0., 0., 0., 0.);
    let p = point(2., 3., 4.);

    assert_eq!(transform * p, point(6., 3., 4.));
  }

  #[test]
  fn shearing_should_move_y_in_proportion_to_x() {
    let transform = Matrix4x4::shear(0., 0., 1., 0., 0., 0.);
    let p = point(2., 3., 4.);

    assert_eq!(transform * p, point(2., 5., 4.));
  }

  #[test]
  fn shearing_should_move_y_in_proportion_to_z() {
    let transform = Matrix4x4::shear(0., 0., 0., 1., 0., 0.);
    let p = point(2., 3., 4.);

    assert_eq!(transform * p, point(2., 7., 4.));
  }

  #[test]
  fn shearing_should_move_z_in_proportion_to_x() {
    let transform = Matrix4x4::shear(0., 0., 0., 0., 1., 0.);
    let p = point(2., 3., 4.);

    assert_eq!(transform * p, point(2., 3., 6.));
  }

  #[test]
  fn shearing_should_move_z_in_proportion_to_y() {
    let transform = Matrix4x4::shear(0., 0., 0., 0., 0., 1.);
    let p = point(2., 3., 4.);

    assert_eq!(transform * p, point(2., 3., 7.));
  }

  #[test]
  fn individual_transforms_are_applied_in_sequence() {
    let p = point(1., 0., 1.);

    let a = Matrix4x4::rotate_x(std::f32::consts::PI / 2.);
    let b = Matrix4x4::scale(5., 5., 5.);
    let c = Matrix4x4::translate(10., 5., 7.);

    let p2 = a * p;
    assert_eq!(p2, point(1., -1., 0.));

    let p3 = b * p2;
    assert_eq!(p3, point(5., -5., 0.));

    let p4 = c * p3;
    assert_eq!(p4, point(15., 0., 7.));
  }

  #[test]
  fn chained_transformations_are_applied_in_reverse_order() {
    let p = point(1., 0., 1.);

    let a = Matrix4x4::rotate_x(std::f32::consts::PI / 2.);
    let b = Matrix4x4::scale(5., 5., 5.);
    let c = Matrix4x4::translate(10., 5., 7.);

    let transform = c * b * a;

    assert_eq!(transform * p, point(15., 0., 7.));
  }
}