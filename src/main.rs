//! A simple software-only scene ray-tracing implementation with Rust.

extern crate cgmath;
extern crate image;

use cgmath::*;
use image::GenericImage;
use std::f64::consts::PI;
use std::fs::File;

/// Minimum resolution for our floating-point comparisons.
const EPSILON: f64 = 1e-7;

/// Gamma constant for RGBA conversion.
const GAMMA: f64 = 2.2;

/// The maximum number of recursive traces we can perform when rendering the image.
const MAX_TRACE_DEPTH: usize = 3;

// We use cgmath for it's excellent vector types.
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

fn encode_gamma(value: f64) -> u8 {
	(value).powf(1.0 / GAMMA) as u8
}

fn decode_gamma(value: u8) -> f64 {
	(value as f64).powf(GAMMA)
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

	pub fn new(r: f64, g: f64, b: f64) -> Color {
		Color { r, g, b }
	}

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
	width: usize,
	height: usize,
	pixels: Vec<Color>,
}

impl Image {
	pub fn new(width: usize, height: usize) -> Image {
		Image { width, height, pixels: vec![Color::BLACK; width * height] }
	}

	/// Loads a .PNG image from the given path.
	pub fn load(path: &str) -> Image {
		unimplemented!()
	}

	/// Retrieves the color at the given (x, y) image coordinates.
	pub fn get(&self, x: usize, y: usize) -> &Color {
		assert!(x < self.width);
		assert!(y < self.height);

		&self.pixels[x + y * self.width]
	}

	/// Sets the color at the given (x, y) image coordinates.
	pub fn set(&mut self, x: usize, y: usize, color: Color) {
		assert!(x < self.width);
		assert!(y < self.height);

		self.pixels[x + y * self.width] = color;
	}

	/// Saves the image in .PNG format to the given path.
	pub fn save(&self, path: &str) {
		unimplemented!()
	}
}

/// Defines a ray in floating point 3-space.
#[derive(Clone, Debug)]
struct Ray {
	origin: Point,
	direction: Vector,
}

impl Ray {
	pub fn new(origin: Point, direction: Vector) -> Ray {
		Ray { origin, direction }
	}

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

impl Sphere {
	pub fn new(center: Point, radius: f64, material: Material) -> Sphere {
		Sphere { center, radius, material }
	}
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

impl Plane {
	pub fn new(origin: Point, normal: Vector, material: Material) -> Plane {
		Plane { origin, normal, material }
	}
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
	pub fn render(&self, width: usize, height: usize) -> Image {
		let mut image = Image::new(width, height);

		for y in 0..height {
			for x in 0..width {
				let camera_ray = self.project(x, y, width, height);
				let color = self.trace(&camera_ray, 0, MAX_TRACE_DEPTH);

				image.set(x, y, color.clamp());
			}
		}

		image
	}

	/// Projects a ray into the scene at the given coordinates.
	fn project(&self, x: usize, y: usize, width: usize, height: usize) -> Ray {
		unimplemented!()
	}

	/// Samples the color at the resultant object by projecting a ray into the scene
	/// and following it along it's path of reflection/refraction.
	fn trace(&self, ray: &Ray, depth: usize, max_depth: usize) -> Color {
		// don't trace beyond a certain level of recursion; technically light attenuates
		// with each reflection but we don't model this property.
		if depth >= max_depth {
			return self.background_color.clone();
		}

		unimplemented!()
	}

	/// Traces a ray into the scene, attempting to find the first intersecting object that it collides with
	fn find_intersection_object(&self, ray: &Ray) -> Option<(&SceneNode, f64)> {
		unimplemented!()
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

	let image = scene.render(1920, 1080);

	image.save("output.png");
}
