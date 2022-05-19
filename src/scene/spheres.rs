//! Sphere objects for use in scene rendering.

use crate::maths::{Matrix4x4, point, Ray, Tuple, vec};

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

  /// Sets the transform for this sphere.
  pub fn set_transform(&mut self, transform: Matrix4x4) {
    self.transform = transform;
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

  fn normal_at(&self, world_point: Tuple) -> Tuple {
    if let Ok(inverse_transform) = self.transform.invert() {
      let object_point = inverse_transform * world_point;
      let object_normal = object_point - point(0., 0., 0.);
      let mut world_normal = inverse_transform.transpose() * object_normal;

      world_normal.w = 0.;

      world_normal.normalize()
    }else {
      vec(0., 0., 0.)
    }
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

  #[test]
  fn normal_on_sphere_on_x_axis() {
    let sphere = Sphere::new(point(0., 0., 0.), 1.);
    let normal = sphere.normal_at(point(1., 0., 0.));

    assert_eq!(normal, vec(1., 0., 0.));
  }

  #[test]
  fn normal_on_sphere_on_y_axis() {
    let sphere = Sphere::new(point(0., 0., 0.), 1.);
    let normal = sphere.normal_at(point(0., 1., 0.));

    assert_eq!(normal, vec(0., 1., 0.));
  }

  #[test]
  fn normal_on_sphere_on_z_axis() {
    let sphere = Sphere::new(point(0., 0., 0.), 1.);
    let normal = sphere.normal_at(point(0., 0., 1.));

    assert_eq!(normal, vec(0., 0., 1.));
  }

  #[test]
  fn normal_on_sphere_at_non_axial_point() {
    let sphere = Sphere::new(point(0., 0., 0.), 1.);
    let point = point(3f32.sqrt() / 3., 3f32.sqrt() / 3., 3f32.sqrt() / 3.);
    let normal = sphere.normal_at(point);

    assert_eq!(normal, vec(3f32.sqrt() / 3., 3f32.sqrt() / 3., 3f32.sqrt() / 3.));
  }

  #[test]
  fn normal_on_sphere_is_normalised() {
    let sphere = Sphere::new(point(0., 0., 0.), 1.);
    let point = point(3f32.sqrt() / 3., 3f32.sqrt() / 3., 3f32.sqrt() / 3.);
    let normal = sphere.normal_at(point);

    assert_eq!(normal.normalize(), normal);
  }

  #[test]
  fn normal_on_translated_sphere() {
    let sphere = Sphere::new(point(0., 1., 0.), 1.);
    let normal = sphere.normal_at(point(0., 1.70711, -0.70711));

    assert_eq!(normal, vec(0., 0.70711, -0.70711));
  }

  #[test]
  fn normal_on_transformed_sphere() {
    let mut sphere = Sphere::new(point(0., 0., 0.), 1.);

    let transform =
        Matrix4x4::scale(1., 0.5, 1.) *
            Matrix4x4::rotate_z(std::f32::consts::PI / 5.);

    sphere.set_transform(transform);

    let normal = sphere.normal_at(point(0., 2f32.sqrt() / 2., -2f32.sqrt() / 2.));

    assert_eq!(normal, vec(0., 0.97014, -0.24254));
  }
}