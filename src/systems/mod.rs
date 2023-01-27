use self::time_system::TimeSystem;

use super::*;

mod action_system;
mod file_watcher_system;
mod game_state_system;
mod shader_system;
mod time_system;

pub use action_system::*;
pub use file_watcher_system::*;
pub use game_state_system::*;
use geng::prelude::itertools::Itertools;
pub use shader_system::*;

pub trait System {
    fn update(&mut self, world: &mut legion::World, resources: &mut Resources);
    fn draw(
        &self,
        world: &legion::World,
        resources: &Resources,
        framebuffer: &mut ugli::Framebuffer,
    );
}

impl Game {
    pub fn create_active_systems(resources: &mut Resources) -> Vec<Box<dyn System>> {
        let mut fws = FileWatcherSystem::new();
        resources.load(&mut fws);

        let mut systems: Vec<Box<dyn System>> = Vec::default();
        systems.push(Box::new(GameStateSystem::new(GameState::MainMenu)));
        systems.push(Box::new(ShaderSystem::new()));
        systems.push(Box::new(fws));
        systems.push(Box::new(TimeSystem::new()));
        systems.push(Box::new(ActionSystem::new()));
        systems
    }

    pub fn init_world(resources: &mut Resources, world: &mut legion::World) {
        Self::create_units_from_templates(resources, world);
    }

    pub fn create_units_from_templates(resources: &mut Resources, world: &mut legion::World) {
        for template in resources.unit_templates.values() {
            let entity = world.push((Position(Vec2::ZERO),));
            let mut entry = world.entry(entity).unwrap();
            template.0.iter().for_each(|component| {
                component.add_to_entry(
                    &mut entry,
                    &entity,
                    &mut resources.defined_statuses,
                    &mut resources.active_statuses,
                )
            });
        }
    }
}
