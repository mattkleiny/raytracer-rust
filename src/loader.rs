use serde::{Deserialize, Serialize};

use crate::graphics::{CheckerPattern, GradientPattern, RingPattern, StripedPattern};
use crate::maths::{Color, Matrix4x4, rgb, vec3, Vector};
use crate::scene::*;

type PackedTuple = [f64; 3];

impl From<PackedTuple> for Vector {
  fn from([x, y, z]: PackedTuple) -> Self {
    vec3(x, y, z)
  }
}

impl From<PackedTuple> for Color {
  fn from([r, g, b]: PackedTuple) -> Self {
    rgb(r, g, b)
  }
}

/// A serialized `Scene` that can be read from a file.
#[derive(Serialize, Deserialize)]
pub struct PackedScene {
  lights: Vec<PackedLight>,
  objects: Vec<PackedObject>,
}

#[derive(Serialize, Deserialize)]
struct PackedLight {
  position: PackedTuple,
  color: Option<PackedTuple>,
}

impl PackedLight {
  pub fn build(&self) -> PointLight {
    let [x, y, z] = self.position;
    let [r, g, b] = self.color.unwrap_or([1., 1., 1.]);

    PointLight::new(vec3(x, y, z), rgb(r, g, b))
  }
}

#[derive(Serialize, Deserialize)]
struct PackedObject {
  kind: PackedKind,
  position: Option<PackedTuple>,
  rotation: Option<PackedTuple>,
  scale: Option<PackedTuple>,
  material: Option<PackedMaterial>,
}

#[derive(Copy, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
enum PackedKind {
  Sphere,
  Plane,
}

impl PackedObject {
  pub fn build(&self) -> Box<dyn Traceable> {
    let mut transform = Matrix4x4::identity();
    let material = self.material.as_ref()
      .map(|packed| packed.build())
      .unwrap_or(Material::default());

    if let Some([x, y, z]) = self.position {
      transform = transform * Matrix4x4::translate(x, y, z);
    }

    if let Some([x, y, z]) = self.rotation {
      transform = transform * Matrix4x4::rotate_x(x);
      transform = transform * Matrix4x4::rotate_y(y);
      transform = transform * Matrix4x4::rotate_z(z);
    };

    if let Some([x, y, z]) = self.scale {
      transform = transform * Matrix4x4::scale(x, y, z);
    }

    match self.kind {
      PackedKind::Sphere => {
        Box::new(
          Sphere::new()
            .with_material(material)
            .with_transform(transform)
        )
      }
      PackedKind::Plane => {
        Box::new(
          Plane::new(vec3(0., 1., 0.))
            .with_material(material)
            .with_transform(transform)
        )
      }
    }
  }
}

#[derive(Serialize, Deserialize)]
struct PackedMaterial {
  texture: Option<PackedTexture>,
  ambient: Option<f64>,
  diffuse: Option<f64>,
  specular: Option<f64>,
  shininess: Option<f64>,
  transparency: Option<f64>,
  reflectivity: Option<f64>,
  refractivity: Option<f64>,
}

impl PackedMaterial {
  pub fn build(&self) -> Material {
    let texture = self.texture.unwrap_or(PackedTexture::Solid([1., 1., 1.]));
    let ambient = self.ambient.unwrap_or(0.1);
    let diffuse = self.diffuse.unwrap_or(0.9);
    let specular = self.specular.unwrap_or(0.9);
    let shininess = self.shininess.unwrap_or(200.);
    let transparency = self.transparency.unwrap_or(0.);
    let reflectivity = self.reflectivity.unwrap_or(0.);
    let refractivity = self.refractivity.unwrap_or(1.);

    Material {
      texture: texture.build(),
      ambient,
      diffuse,
      specular,
      shininess,
      transparency,
      reflectivity,
      refractivity,
    }
  }
}

#[derive(Copy, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
enum PackedTexture {
  Solid(PackedTuple),
  Checker(PackedTuple, PackedTuple),
  Gradient(PackedTuple, PackedTuple),
  Ring(PackedTuple, PackedTuple),
  Striped(PackedTuple, PackedTuple),
}

impl PackedTexture {
  pub fn build(&self) -> Texture {
    match *self {
      PackedTexture::Solid(color) => {
        Texture::Solid(color.into())
      }
      PackedTexture::Checker(from, to) => {
        let pattern = CheckerPattern::new(from.into(), to.into());

        Texture::Pattern(Box::new(pattern))
      }
      PackedTexture::Gradient(from, to) => {
        let pattern = GradientPattern::new(from.into(), to.into());

        Texture::Pattern(Box::new(pattern))
      }
      PackedTexture::Ring(from, to) => {
        let pattern = RingPattern::new(from.into(), to.into());

        Texture::Pattern(Box::new(pattern))
      }
      PackedTexture::Striped(from, to) => {
        let pattern = StripedPattern::new(from.into(), to.into());

        Texture::Pattern(Box::new(pattern))
      }
    }
  }
}

impl PackedScene {
  /// Loads the scene from the given YAML file.
  pub fn from_yaml_file(path: &str) -> anyhow::Result<Self> {
    let file = std::fs::File::open(path)?;
    let scene: Self = serde_yaml::from_reader(&file)?;

    Ok(scene)
  }

  /// Converts this packed scene into a usable `Scene`.
  pub fn build(&self) -> anyhow::Result<Scene> {
    let mut scene = Scene::new();

    for light in &self.lights {
      scene.add_light(light.build());
    }

    for object in &self.objects {
      scene.add_object_boxed(object.build());
    }

    Ok(scene)
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn packed_scene_can_load_from_yaml() {
    let packed = PackedScene::from_yaml_file("assets/scenes/test01.yaml").unwrap();

    assert_eq!(packed.lights.len(), 1);
    assert_eq!(packed.objects.len(), 4);
  }
}