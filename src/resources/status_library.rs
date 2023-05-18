use geng::prelude::itertools::Itertools;

use super::*;

#[derive(Default)]
pub struct StatusLibrary {
    register: HashMap<String, Status>,
}

impl StatusLibrary {
    pub fn register(name: &str, status: Status, resources: &mut Resources) {
        if let Some(description) = status.description.as_ref() {
            resources
                .definitions
                .insert(name.to_owned(), status.color, description.to_owned());
        }
        resources
            .status_library
            .register
            .insert(name.to_owned(), status);
    }

    pub fn get<'a>(name: &str, resources: &'a Resources) -> &'a Status {
        resources
            .status_library
            .register
            .get(name)
            .expect(&format!("Status not found in library {name}"))
    }

    pub fn get_trigger(name: &str, resources: &Resources) -> Trigger {
        Self::get(name, resources).trigger.clone()
    }

    pub fn add_triggers(
        statuses: Vec<(String, i32)>,
        resources: &Resources,
    ) -> Vec<(String, (Trigger, i32))> {
        Vec::from_iter(statuses.into_iter().map(|(name, charges)| {
            let trigger = Self::get_trigger(&name, resources);
            (name, (trigger, charges))
        }))
    }

    pub fn get_context_shaders(
        context: &Context,
        world: &legion::World,
        resources: &Resources,
    ) -> Vec<Shader> {
        let mut index = -1;
        let statuses = context.collect_statuses(world);
        statuses
            .into_iter()
            .map(|(name, _)| Self::get(&name, resources))
            .filter_map(|status| match status.shader.as_ref() {
                Some(shader) => {
                    let shader = shader.clone();
                    index += 1;
                    Some(
                        shader.merge_uniforms(
                            &hashmap! {
                                "u_color" => ShaderUniform::Color(status.color),
                                "u_index" => ShaderUniform::Int(index as i32),
                            }
                            .into(),
                            true,
                        ),
                    )
                }
                None => None,
            })
            .collect_vec()
    }
}
