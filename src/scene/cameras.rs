use crate::graphics::Canvas;
use crate::maths::{Matrix4x4, point, Ray};
use crate::scene::Scene;

/// A camera for orientating a view transform.
#[derive(Clone)]
pub struct Camera {
  width: u32,
  height: u32,
  half_width: f32,
  half_height: f32,
  field_of_view: f32,
  pixel_size: f32,
  pub transform: Matrix4x4,
}

impl Camera {
  /// Creates a new camera with the given dimensions.
  pub fn new(width: u32, height: u32, field_of_view: f32) -> Self {
    let half_view = (field_of_view / 2.).tan();
    let aspect = width as f32 / height as f32;

    let half_width;
    let half_height;

    if aspect >= 1. {
      half_width = half_view;
      half_height = half_width / aspect;
    } else {
      half_width = half_view * aspect;
      half_height = half_view;
    }

    Self {
      width,
      height,
      half_width,
      half_height,
      field_of_view,
      pixel_size: (half_width * 2.) / width as f32,
      transform: Matrix4x4::identity(),
    }
  }

  /// Creates a ray for the given pixel (x, y) on the camera.
  pub fn ray_for_pixel(&self, x: usize, y: usize) -> Ray {
    let x_offset = (x as f32 + 0.5) * self.pixel_size;
    let y_offset = (y as f32 + 0.5) * self.pixel_size;

    let world_x = self.half_width - x_offset;
    let world_y = self.half_height - y_offset;

    let inverse = self.transform.invert().expect("Failed to invert camera transform");

    let pixel = inverse * point(world_x, world_y, -1.);
    let origin = inverse * point(0., 0., 0.);
    let direction = (pixel - origin).normalize();

    Ray::new(origin, direction)
  }

  /// Renders an image of the given scene through the lens of the camera.
  pub fn render(&self, scene: &Scene) -> Canvas {
    let mut canvas = Canvas::new(self.width, self.height);

    for y in 0..self.height as usize {
      for x in 0..self.width as usize {
        let ray = self.ray_for_pixel(x, y);
        let color = scene.sample(ray);

        canvas.set_pixel(x, y, color);
      }
    }

    canvas
  }
}

#[cfg(test)]
mod tests {
  use crate::maths::{PI, vec3};

  use super::*;

  #[test]
  fn pixel_size_for_horizontal_canvas() {
    let camera = Camera::new(200, 125, PI / 2.);

    assert_eq!(camera.pixel_size, 0.01);
  }

  #[test]
  fn pixel_size_for_vertical_camera() {
    let camera = Camera::new(125, 200, PI / 2.);

    assert_eq!(camera.pixel_size, 0.01);
  }

  #[test]
  fn construct_ray_through_center_of_camera() {
    let camera = Camera::new(201, 101, PI / 2.);
    let ray = camera.ray_for_pixel(100, 50);

    assert_eq!(ray.origin, point(0., 0., 0.));
    assert_eq!(ray.direction, vec3(0., 0., -1.));
  }

  #[test]
  fn construct_ray_through_corner_of_camera() {
    let camera = Camera::new(201, 101, PI / 2.);
    let ray = camera.ray_for_pixel(0, 0);

    assert_eq!(ray.origin, point(0., 0., 0.));
    assert_eq!(ray.direction, vec3(0.66519, 0.33259, -0.66851));
  }

  #[test]
  fn construct_ray_when_camera_is_transformed() {
    let mut camera = Camera::new(201, 101, PI / 2.);
    camera.transform = Matrix4x4::rotate_y(PI / 4.) * Matrix4x4::translate(0., -2., 5.);
    let ray = camera.ray_for_pixel(100, 50);

    assert_eq!(ray.origin, point(0., 2., -5.));
    assert_eq!(ray.direction, vec3(2f32.sqrt() / 2., 0., -2f32.sqrt() / 2.));
  }
}