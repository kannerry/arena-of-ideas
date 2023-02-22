use std::collections::hash_map::Entry;

use geng::prelude::{itertools::Itertools, ugli::SingleUniform};
use legion::EntityStore;

use super::*;

pub struct ShaderSystem {}

impl System for ShaderSystem {
    fn update(&mut self, _world: &mut legion::World, resources: &mut Resources) {
        resources.frame_shaders.clear();
    }

    fn draw(
        &self,
        world: &legion::World,
        resources: &mut Resources,
        framebuffer: &mut ugli::Framebuffer,
    ) {
        self.draw_all_shaders(world, resources, framebuffer);
    }
}

impl ShaderSystem {
    pub fn new() -> Self {
        Self {}
    }

    /// Get Shader component and merge Context into it's vars if any
    pub fn get_entity_shader(world: &legion::World, entity: legion::Entity) -> Shader {
        let mut shader = world
            .entry_ref(entity)
            .expect("Failed to find Entry")
            .get_component::<Shader>()
            .unwrap()
            .clone();
        let context = Context::construct_context(&entity, world);
        shader.parameters.uniforms = shader
            .parameters
            .uniforms
            .merge(&context.vars.clone().into());
        shader
    }

    pub fn draw_all_shaders(
        &self,
        world: &legion::World,
        resources: &mut Resources,
        framebuffer: &mut ugli::Framebuffer,
    ) {
        // Get all Shader components from World for drawing
        let world_shaders = <(&Shader, &EntityComponent)>::query()
            .filter(!component::<UnitComponent>())
            .iter(world)
            .map(|(_, entity)| Self::get_entity_shader(world, entity.entity))
            .collect_vec();

        let shaders = [
            world_shaders,
            resources.cassette.get_shaders(resources.mouse_pos),
            resources.frame_shaders.clone(),
        ]
        .concat();
        let mut shaders_by_layer: HashMap<ShaderLayer, Vec<Shader>> = HashMap::default();
        let emtpy_vec: Vec<Shader> = Vec::new();
        for shader in shaders {
            let layer = &shader.layer;
            let vec = match shaders_by_layer.entry(layer.clone()) {
                Entry::Occupied(o) => o.into_mut(),
                Entry::Vacant(v) => v.insert(emtpy_vec.clone()),
            };
            vec.push(shader);
        }
        for (_layer, shaders) in shaders_by_layer.iter().sorted_by_key(|entry| entry.0) {
            for shader in shaders.iter().sorted_by_key(|shader| shader.order) {
                let uniforms = ugli::uniforms!(
                    u_global_time: resources.game_time,
                    u_game_time: resources.cassette.head,
                );
                let texts = shader
                    .parameters
                    .uniforms
                    .0
                    .iter()
                    .filter_map(|(key, uniform)| match uniform {
                        ShaderUniform::String((font, text)) => {
                            Some((*font, text, key, format!("{}_size", key)))
                        }
                        _ => None,
                    })
                    .collect_vec();
                resources.fonts.load_textures(
                    texts
                        .iter()
                        .map(|(font, text, _, _)| (*font, *text))
                        .collect_vec(),
                );
                let mut texture_uniforms = SingleUniformVec::default();
                let mut texture_size_uniforms = SingleUniformVec::default();
                for (font, text, key, size_key) in texts.iter() {
                    let texture = resources.fonts.get_texture(*font, text);
                    texture_uniforms.0.push(SingleUniform::new(key, texture));
                    texture_size_uniforms.0.push(SingleUniform::new(
                        &size_key,
                        texture.and_then(|texture| Some(texture.size().map(|x| x as f32))),
                    ));
                }
                Self::draw_shader(
                    shader,
                    framebuffer,
                    &resources.geng,
                    &resources.camera,
                    &resources.shader_programs,
                    (texture_uniforms, texture_size_uniforms, uniforms),
                );
            }
        }
    }

    fn draw_shader<U>(
        shader: &Shader,
        framebuffer: &mut ugli::Framebuffer,
        geng: &Geng,
        camera: &geng::Camera2d,
        shader_programs: &ShaderPrograms,
        uniforms: U,
    ) where
        U: ugli::Uniforms,
    {
        let mut chain = Some(Box::new(shader.clone()));
        let shader_uniforms = &mut default::<ShaderUniforms>();

        while let Some(shader) = chain {
            let program = shader_programs.get_program(&static_path().join(&shader.path));
            shader_uniforms.merge_mut(&shader.parameters.uniforms, true);
            let parameters = ShaderParameters {
                uniforms: shader_uniforms.clone(),
                ..shader.parameters.clone()
            };

            let mut instances_arr: ugli::VertexBuffer<Instance> =
                ugli::VertexBuffer::new_dynamic(geng.ugli(), Vec::new());
            instances_arr.resize(shader.parameters.instances, Instance {});
            let uniforms = (
                geng::camera2d_uniforms(camera, framebuffer.size().map(|x| x as f32)),
                parameters,
                &uniforms,
            );
            let quad = Self::get_quad(shader.parameters.vertices, &geng);
            ugli::draw(
                framebuffer,
                &program,
                ugli::DrawMode::TriangleStrip,
                ugli::instanced(&quad, &instances_arr),
                uniforms,
                ugli::DrawParameters {
                    blend_mode: Some(ugli::BlendMode::straight_alpha()),
                    ..default()
                },
            );
            chain = shader.chain.clone();
        }
    }

    pub fn get_quad(vertices: usize, geng: &Geng) -> ugli::VertexBuffer<draw_2d::Vertex> {
        let vert_count = vertices;
        let mut vertices = vec![draw_2d::Vertex {
            a_pos: vec2(-1.0, -1.0),
        }];
        for i in 0..vert_count {
            vertices.push(draw_2d::Vertex {
                a_pos: vec2((i as f32 / vert_count as f32) * 2.0 - 1.0, 1.0),
            });
            vertices.push(draw_2d::Vertex {
                a_pos: vec2(((i + 1) as f32 / vert_count as f32) * 2.0 - 1.0, -1.0),
            });
        }

        vertices.push(draw_2d::Vertex {
            a_pos: vec2(1.0, 1.0),
        });

        ugli::VertexBuffer::new_dynamic(geng.ugli(), vertices)
    }
}

#[derive(ugli::Vertex, Debug, Clone)]
pub struct Instance {}
