use bevy::render::{mesh::MeshVertexBufferLayout, render_resource::RenderPipelineDescriptor};
use strum_macros::Display;

use super::*;

#[derive(AsBindGroup, TypeUuid, TypePath, Debug, Clone, Default)]
#[uuid = "ec09cb82-5a6b-43cd-ab8a-56d0979f7cc4"]
#[bind_group_data(CustomMaterialKey)]
pub struct SdfShapeMaterial {
    #[uniform(0)]
    pub color: Color,
    #[uniform(0)]
    pub size: Vec2,
    pub shape: Shape,
}

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, Display, Serialize, Deserialize)]
pub enum Shape {
    #[default]
    Rectangle,
    Circle,
}

impl Shape {
    pub fn def(&self) -> String {
        self.to_string().to_uppercase()
    }
    pub fn mesh(&self, size: Vec2) -> Mesh {
        match self {
            Shape::Rectangle => Mesh::from(shape::Quad::new(size)),
            Shape::Circle => Mesh::from(shape::Circle::new(size.x)),
        }
    }
}

impl Material2d for SdfShapeMaterial {
    fn fragment_shader() -> bevy::render::render_resource::ShaderRef {
        "shaders/sdf_shape.wgsl".into()
    }

    fn specialize(
        descriptor: &mut RenderPipelineDescriptor,
        _: &MeshVertexBufferLayout,
        key: bevy::sprite::Material2dKey<Self>,
    ) -> __private::Result<(), bevy::render::render_resource::SpecializedMeshPipelineError> {
        let fragment = descriptor.fragment.as_mut().unwrap();
        fragment
            .shader_defs
            .push(key.bind_group_data.shape.def().into());
        Ok(())
    }
}

#[derive(Eq, PartialEq, Hash, Clone, Copy)]
pub struct CustomMaterialKey {
    shape: Shape,
}

impl From<&SdfShapeMaterial> for CustomMaterialKey {
    fn from(material: &SdfShapeMaterial) -> Self {
        Self {
            shape: material.shape,
        }
    }
}
