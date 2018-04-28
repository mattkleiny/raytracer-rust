extern crate cgmath;
extern crate image;

use cgmath::*;
use std::f64::consts::PI;

const EPSILON: f64 = 1e-7;
const GAMMA: f64 = 2.2;

const MAX_TRACE_DEPTH: usize = 3;

type Point = Vector2<f64>;
type Vector = Vector2<f64>;

fn clamp(value: f64, lower: f64, upper: f64) -> f64 {
	match () {
		_ if value > upper => upper,
		_ if value < lower => lower,
		_ => value
	}
}

fn to_radians(degrees: f64) -> f64 {
	degrees * (PI / 180.0)
}

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

	pub fn clamp(&self) -> Color {
		Color {
			r: clamp(self.r, 0.0, 1.0),
			g: clamp(self.g, 0.0, 1.0),
			b: clamp(self.b, 0.0, 1.0),
		}
	}
}

struct Image {
	width: usize,
	height: usize,
	pixels: Vec<Color>,
}

impl Image {
	pub fn new(width: usize, height: usize) -> Image {
		Image { width, height, pixels: vec![Color::BLACK; width * height] }
	}

	pub fn load(path: &str) -> Image {
		unimplemented!()
	}

	pub fn get(&self, x: usize, y: usize) -> &Color {
		assert!(x < self.width);
		assert!(y < self.height);

		&self.pixels[x + y * self.width]
	}

	pub fn set(&mut self, x: usize, y: usize, color: Color) {
		assert!(x < self.width);
		assert!(y < self.height);

		self.pixels[x + y * self.width] = color;
	}

	pub fn save(&self, path: &str) {
		unimplemented!()
	}
}

#[derive(Clone, Debug)]
struct Ray {
	origin: Point,
	direction: Vector,
}

impl Ray {
	pub fn new(origin: Point, direction: Vector) -> Ray {
		Ray { origin, direction }
	}

	pub fn create_reflection(normal: &Vector, incidence: &Vector, intersection: &Point, bias: f64) {
		unimplemented!()
	}
}

#[derive(Clone, Debug)]
struct UV {
	pub u: f64,
	pub v: f64,
}

enum Material {
	Solid {
		albedo: Color,
		reflectivity: f64,
	},
	Textured {
		image: &'static str,
		reflectivity: f64,
	},
}

impl Material {
	pub fn sample(&self, uv: &UV) -> Color {
		unimplemented!()
	}
}

enum Light {
	Directional,
	Spherical,
}

trait Node {
	fn intersects(&self, ray: &Ray) -> Option<f64>;
	fn calculate_normal(&self, point: &Point) -> Vector;
	fn calculate_uv(&self, point: &Point) -> UV;
	fn material(&self) -> &Material;
}

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

impl Node for Sphere {
	fn intersects(&self, ray: &Ray) -> Option<f64> {
		unimplemented!()
	}

	fn calculate_normal(&self, point: &Vector2<f64>) -> Vector2<f64> {
		unimplemented!()
	}

	fn calculate_uv(&self, point: &Vector2<f64>) -> UV {
		unimplemented!()
	}

	fn material(&self) -> &Material {
		&self.material
	}
}

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

impl Node for Plane {
	fn intersects(&self, ray: &Ray) -> Option<f64> {
		unimplemented!()
	}

	fn calculate_normal(&self, point: &Vector2<f64>) -> Vector2<f64> {
		unimplemented!()
	}

	fn calculate_uv(&self, point: &Vector2<f64>) -> UV {
		unimplemented!()
	}

	fn material(&self) -> &Material {
		&self.material
	}
}

struct Scene {
	field_of_view: f64,
	background_color: Color,
	lights: Vec<Box<Light>>,
	nodes: Vec<Box<Node>>,
}

impl Scene {
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

	fn project(&self, x: usize, y: usize, width: usize, height: usize) -> Ray {
		unimplemented!()
	}

	fn trace(&self, ray: &Ray, depth: usize, max_depth: usize) -> Color {
		unimplemented!()
	}
}

struct SceneBuilder {
	field_of_view: f64,
	background_color: Color,
	lights: Vec<Box<Light>>,
	nodes: Vec<Box<Node>>,
}

impl SceneBuilder {
	pub fn new() -> SceneBuilder {
		SceneBuilder {
			field_of_view: 90.0,
			background_color: Color::BLACK,
			lights: Vec::new(),
			nodes: Vec::new(),
		}
	}

	pub fn set_field_of_view(&mut self, fov: f64) -> &mut SceneBuilder {
		self.field_of_view = fov;
		self
	}

	pub fn set_background_color(&mut self, color: Color) -> &mut SceneBuilder {
		self.background_color = color;
		self
	}

	pub fn add_light(&mut self, light: Box<Light>) -> &mut SceneBuilder {
		self.lights.push(light);
		self
	}

	pub fn add_node(&mut self, node: Box<Node>) -> &mut SceneBuilder {
		self.nodes.push(node);
		self
	}

	pub fn build(&self) -> Scene {
		unimplemented!()
	}
}

fn main() {
	let scene = SceneBuilder::new()
			.set_field_of_view(75.0)
			.set_background_color(Color::WHITE)
			.build();

	let image = scene.render(1920, 1080);

	image.save("output.png");
}
