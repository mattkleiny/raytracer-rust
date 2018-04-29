//! A simple software-only scene ray-tracing implementation with Rust.

extern crate cgmath;
extern crate image;

use cgmath::*;
use image::GenericImage;
use std::f64::consts::PI;
use std::fs::File;

/// Minimum resolution for our floating-point comparisons.
const EPSILON: f64 = 1e-7;

/// Gamma constant for image conversion.
const GAMMA: f64 = 2.2;

/// The maximum number of recursive traces we can perform when rendering the image.
const MAX_TRACE_DEPTH: usize = 3;

type Point = Vector3<f64>;
type Vector = Vector3<f64>;

/// Clamps the given value between the given lower and upper bounds.
fn clamp<V: PartialOrd>(value: V, lower: V, upper: V) -> V {
	match () {
		_ if value > upper => upper,
		_ if value < lower => lower,
		_ => value
	}
}

/// Converts the given value to radians from degrees.
fn to_radians(degrees: f64) -> f64 {
	degrees * (PI / 180.0)
}

/// Defines a color in floating-point RGBA color space.
#[derive(Clone, Debug)]
struct Color {
	r: f64,
	g: f64,
	b: f64,
}

impl Color {
	const BLACK: Color = Color { r: 0.0, g: 0.0, b: 0.0 };
	const RED: Color = Color { r: 1.0, g: 0.0, b: 0.0 };
	const GREEN: Color = Color { r: 0.0, g: 1.0, b: 0.0 };
	const BLUE: Color = Color { r: 0.0, g: 0.0, b: 1.0 };
	const WHITE: Color = Color { r: 1.0, g: 1.0, b: 1.0 };

	/// Clamps all of the color's channels between (0.0 and 1.0).
	pub fn clamp(&self) -> Color {
		Color {
			r: clamp(self.r, 0.0, 1.0),
			g: clamp(self.g, 0.0, 1.0),
			b: clamp(self.b, 0.0, 1.0),
		}
	}
}

/// A bit-mapped image of pixels, convertible to/from PNG.
struct Image {
	width: u32,
	height: u32,
	pixels: Vec<Color>,
}

impl Image {
	/// Creates a new blank image with the given dimensions.
	pub fn new(width: u32, height: u32) -> Image {
		Image { width, height, pixels: vec![Color::BLACK; (width * height) as usize] }
	}

	/// Loads a .PNG image from the given path.
	pub fn load(path: &str) -> Image {
		fn decode_gamma(value: u8) -> f64 {
			(value as f64).powf(GAMMA)
		}

		// load the source image, prepare an in-memory buffered image
		let image = image::open(path).expect(&format!("Unable to load source image: {}", path));
		let (width, height) = image.dimensions();

		let mut result = Image::new(width, height);

		for y in 0..result.height {
			for x in 0..result.width {
				// sample source pixels; correct for gamma over conversion from u8 to f64.
				let source_pixel = image.get_pixel(x, y);
				let corrected_pixel = Color {
					r: decode_gamma(source_pixel.data[0]) * 255.0,
					g: decode_gamma(source_pixel.data[1]) * 255.0,
					b: decode_gamma(source_pixel.data[2]) * 255.0,
				};

				result.set(x, y, corrected_pixel);
			}
		}

		result
	}

	/// Retrieves the color at the given (x, y) image coordinates.
	pub fn get(&self, x: u32, y: u32) -> &Color {
		assert!(x < self.width);
		assert!(y < self.height);

		&self.pixels[(x + y * self.width) as usize]
	}

	/// Sets the color at the given (x, y) image coordinates.
	pub fn set(&mut self, x: u32, y: u32, color: Color) {
		assert!(x < self.width);
		assert!(y < self.height);

		self.pixels[(x + y * self.width) as usize] = color;
	}

	/// Saves the image in .PNG format to the given path.
	pub fn save(&self, path: &str) {
		fn encode_gamma(value: f64) -> u8 {
			(value).powf(1.0 / GAMMA) as u8
		}

		let mut image = image::ImageBuffer::new(self.width, self.height);

		for y in 0..self.height {
			for x in 0..self.width {
				let source_pixel = self.get(x, y);
				let corrected_pixel = image::Rgb([
					encode_gamma(source_pixel.r * 255.0),
					encode_gamma(source_pixel.g * 255.0),
					encode_gamma(source_pixel.b * 255.0),
				]);

				image[(x, y)] = corrected_pixel;
			}
		}

		image.save(path).expect(&format!("Unable to save image: {}", path));
	}
}

/// Defines a ray in floating point 3-space.
#[derive(Clone, Debug)]
struct Ray {
	origin: Point,
	direction: Vector,
}

impl Ray {
	/// Creates a ray reflected around the given intersection point with the given normal and incidence.
	pub fn create_reflection(normal: &Vector, incidence: &Vector, intersection: &Point, bias: f64) {
		unimplemented!()
	}
}

/// Encapsulates UV texture mapping coordinates.
#[derive(Clone, Debug)]
struct UV {
	pub u: f64,
	pub v: f64,
}

/// Defines the material properties of some object.
enum Material {
	/// A solid colored material.
	Solid {
		albedo: Color,
		reflectivity: f64,
	},
	/// A textured image material.
	Textured {
		image: Image,
		reflectivity: f64,
	},
}

impl Material {
	/// Samples the material at the given UV coordinates, returning it's color.
	pub fn sample(&self, uv: &UV) -> Color {
		unimplemented!()
	}
}

/// Defines a light in the scene.
enum Light {
	Directional {
		direction: Vector,
		emissive: Color,
		intensity: f64,
	},
	Spherical {
		position: Point,
		emissive: Color,
		intensity: f64,
	},
}

/// Defines a node in the scene.
trait SceneNode {
	/// Determines if the node intersects with the given ray, and returns the distance
	/// along the ray at which the intersection occurs.
	fn intersects(&self, ray: &Ray) -> Option<f64>;

	/// Calculates the normal on the surface of the object at the given point.
	fn calculate_normal(&self, point: &Point) -> Vector;

	/// Calculates the UV coordinates for the object's surface material at the given point.
	fn calculate_uv(&self, point: &Point) -> UV;

	/// Retrieves the material used by the node.
	fn material(&self) -> &Material;
}

/// Defines a sphere in the scene.
struct Sphere {
	center: Point,
	radius: f64,
	material: Material,
}

impl SceneNode for Sphere {
	fn intersects(&self, ray: &Ray) -> Option<f64> {
		unimplemented!()
	}

	fn calculate_normal(&self, point: &Point) -> Vector {
		unimplemented!()
	}

	fn calculate_uv(&self, point: &Point) -> UV {
		unimplemented!()
	}

	fn material(&self) -> &Material {
		&self.material
	}
}

/// Defines a plane in the scene.
struct Plane {
	origin: Point,
	normal: Vector,
	material: Material,
}

impl SceneNode for Plane {
	fn intersects(&self, ray: &Ray) -> Option<f64> {
		unimplemented!()
	}

	fn calculate_normal(&self, point: &Point) -> Vector {
		unimplemented!()
	}

	fn calculate_uv(&self, point: &Point) -> UV {
		unimplemented!()
	}

	fn material(&self) -> &Material {
		&self.material
	}
}

/// Defines the scene properties for the ray-tracing algorithm.
struct Scene {
	field_of_view: f64,
	background_color: Color,
	lights: Vec<Box<Light>>,
	nodes: Vec<Box<SceneNode>>,
}

impl Scene {
	/// Renders the scene to an image with the given dimensions.
	pub fn render(&self, width: u32, height: u32) -> Image {
		let mut image = Image::new(width, height);

		for y in 0..height {
			for x in 0..width {
				let camera_ray = self.project(x as f64, y as f64, width as f64, height as f64);
				let color = self.trace(&camera_ray, 0, MAX_TRACE_DEPTH);

				image.set(x, y, color.clamp());
			}
		}

		image
	}

	/// Samples the color at the resultant object by projecting a ray into the scene
	/// and following it along it's path of reflection/refraction.
	fn trace(&self, ray: &Ray, depth: usize, max_depth: usize) -> Color {
		// don't trace beyond a certain level of recursion; technically light attenuates
		// with each reflection but we don't model this property.
		if depth >= max_depth {
			return self.background_color.clone();
		}

		// project a ray into the scene for each pixel in our resultant image
		let intersection = self.find_intersecting_object(ray);

		// if we're able to locate a valid intersection for this ray
		if let Some(intersection) = intersection {
			let (node, distance) = intersection;
			let material = node.material();

			// calculate the hit point, normal and UV on the surface of the object
			let hit_point = ray.origin + ray.direction * distance;
			let surface_normal = node.calculate_normal(&hit_point);
			let surface_uv = node.calculate_uv(&hit_point);

			// apply lighting and shading to the object
			let mut color = self.apply_diffuse_shading(distance, &material, &hit_point, &surface_normal, &surface_uv);

			// TODO: apply reflectivity
			// TODO: apply refractivity

			return color;
		}

		// otherwise, sample the background color
		self.background_color.clone()
	}

	/// Applies lighting to an object's material by evaluating all the lights in the scene.
	fn apply_diffuse_shading(&self,
													 distance: f64,
													 material: &Material,
													 hit_point: &Point,
													 surface_normal: &Vector,
													 surface_uv: &UV) -> Color {
		let mut color = Color::BLACK;
		let albedo = material.sample(surface_uv);

		// walk through all lights in the scene
		for light in self.lights.iter() {
			unimplemented!()
		}

		color
	}

	/// Traces a ray into the scene, attempting to find the first intersecting object that it collides with.
	fn find_intersecting_object(&self, ray: &Ray) -> Option<(&Box<SceneNode>, f64)> {
		let mut distance = 999999999.0;
		let mut result: Option<(&Box<SceneNode>, f64)> = None;

		// walk through all nodes in the scene
		for node in self.nodes.iter() {
			let intersection = node.intersects(ray);

			// if our ray intersects with the node
			if intersection.is_some() {
				let hit_distance = intersection.unwrap();

				// and the intersection point is the closest we've located so far
				if hit_distance < distance {
					distance = hit_distance;
					result = Some((node, distance)) // then record the result
				}
			}
		}

		result
	}

	/// Projects a ray into the scene at the given coordinates.
	fn project(&self, x: f64, y: f64, width: f64, height: f64) -> Ray {
		assert!(width > height);

		let fov_adjustment = (to_radians(self.field_of_view) / 2.0).tan();
		let aspect_ratio = width / height;
		let sensor_x = ((((x + 0.5) / width) * 2.0 - 1.0) * aspect_ratio) * fov_adjustment;
		let sensor_y = (1.0 - ((y + 0.5) / height) * 2.0) * fov_adjustment;

		let direction = vec3(sensor_x, sensor_y, -1.0).normalize();

		Ray { origin: vec3(0.0, 0.0, 0.0), direction }
	}
}

impl Default for Scene {
	fn default() -> Self {
		Self {
			field_of_view: 90.0,
			background_color: Color::BLACK,
			lights: Vec::new(),
			nodes: Vec::new(),
		}
	}
}

/// Entry point for the ray-tracer.
fn main() {
	// build a simple test scene
	let scene = Scene {
		lights: vec!(
			Box::new(Light::Directional {
				direction: vec3(-1.0, -1.0, 0.0),
				emissive: Color::WHITE,
				intensity: 0.33,
			}),
			Box::new(Light::Directional {
				direction: vec3(1.0, -1.0, 0.0),
				emissive: Color::WHITE,
				intensity: 0.33,
			}),
			Box::new(Light::Spherical {
				position: vec3(0.0, 3.0, 0.0),
				emissive: Color::WHITE,
				intensity: 1.0,
			}),
		),
		nodes: vec!(
			Box::new(Sphere {
				center: vec3(5.0, -1.0, -15.0),
				radius: 2.0,
				material: Material::Solid {
					albedo: Color::BLUE,
					reflectivity: 0.3,
				},
			}),
			Box::new(Sphere {
				center: vec3(3.0, 0.0, -35.0),
				radius: 1.0,
				material: Material::Solid {
					albedo: Color::GREEN,
					reflectivity: 0.1,
				},
			}),
			Box::new(Sphere {
				center: vec3(-5.5, 0.0, -15.0),
				radius: 1.0,
				material: Material::Textured {
					image: Image::load("textures/checkerboard.png"),
					reflectivity: 0.1,
				},
			}),
			Box::new(Plane {
				origin: vec3(0.0, -4.2, 0.0),
				normal: vec3(0.0, -1.0, 0.0),
				material: Material::Solid {
					albedo: Color::WHITE,
					reflectivity: 0.1,
				},
			})
		),
		..Default::default()
	};

	// render the scene into an image
	let image = scene.render(1920, 1080);

	// and export as .PNG
	image.save("output.png");
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn image_should_export_to_png_correctly() {
		let mut image = Image::new(512, 512);

		for y in 0..image.height {
			for x in 0..image.width {
				image.set(x, y, Color {
					r: x as f64,
					g: y as f64,
					b: 0.0,
				});
			}
		}

		image.save("testoutput.png");
	}
}
