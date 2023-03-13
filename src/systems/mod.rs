use self::button_system::ButtonSystem;

use super::*;
use geng::prelude::itertools::Itertools;

mod action_system;
mod battle_system;
mod button_system;
mod camera_system;
mod cassette_player_system;
mod context_system;
mod file_watcher_system;
mod gallery_system;
mod game_over_system;
mod game_state_system;
mod house_system;
mod logger;
mod mouse_system;
mod name_system;
mod pool_ui_system;
mod power_points_system;
mod shader_system;
mod shop_system;
mod slot_system;
mod stats_ui_system;
mod time_system;
mod unit_system;
mod vfx_system;
mod widgets;
mod world_system;

pub use action_system::*;
pub use battle_system::*;
pub use camera_system::*;
pub use cassette_player_system::*;
pub use context_system::*;
pub use file_watcher_system::*;
pub use gallery_system::*;
pub use game_over_system::*;
pub use game_state_system::*;
pub use house_system::*;
pub use logger::*;
pub use mouse_system::*;
pub use name_system::*;
pub use pool_ui_system::*;
pub use power_points_system::*;
pub use shader_system::*;
pub use shop_system::*;
pub use slot_system::*;
pub use stats_ui_system::*;
pub use time_system::*;
pub use unit_system::*;
pub use vfx_system::*;
pub use widgets::*;
pub use world_system::*;

pub trait System {
    fn update(&mut self, world: &mut legion::World, resources: &mut Resources);
    fn draw(
        &self,
        world: &legion::World,
        resources: &mut Resources,
        framebuffer: &mut ugli::Framebuffer,
    ) {
        #![allow(unused_variables)]
    }
    fn ui<'a>(
        &'a mut self,
        cx: &'a ui::Controller,
        world: &'a legion::World,
        resources: &'a Resources,
    ) -> Box<dyn ui::Widget + 'a> {
        #![allow(unused_variables)]
        Box::new(ui::Void)
    }
}

impl Game {
    pub fn create_systems(resources: &mut Resources) -> Vec<Box<dyn System>> {
        let mut fws = FileWatcherSystem::new();
        resources.load(&mut fws);

        let mut global_systems: Vec<Box<dyn System>> = Vec::default();
        let mut game_state = GameStateSystem::new(GameState::MainMenu);
        game_state.add_systems(GameState::MainMenu, vec![]);
        game_state.add_systems(
            GameState::Battle,
            vec![
                Box::new(SlotSystem::new()),
                Box::new(CassettePlayerSystem::new(false)),
                Box::new(ActionSystem::new()),
                Box::new(BattleSystem::new()),
            ],
        );
        game_state.add_systems(
            GameState::Shop,
            vec![
                Box::new(SlotSystem::new()),
                Box::new(ShopSystem::new()),
                Box::new(PoolUiSystem::new()),
                Box::new(CassettePlayerSystem::new(true)),
                Box::new(ActionSystem::new()),
            ],
        );
        game_state.add_systems(
            GameState::Gallery,
            vec![
                Box::new(GallerySystem::new()),
                Box::new(SlotSystem::new()),
                Box::new(CassettePlayerSystem::new(true)),
            ],
        );
        game_state.add_systems(
            GameState::GameOver,
            vec![
                Box::new(GameOverSystem::new()),
                Box::new(CassettePlayerSystem::new(true)),
                Box::new(SlotSystem::new()),
            ],
        );

        global_systems.push(Box::new(fws));
        global_systems.push(Box::new(TimeSystem::new()));
        global_systems.push(Box::new(CameraSystem::new()));
        global_systems.push(Box::new(ContextSystem::new()));
        global_systems.push(Box::new(ShaderSystem::new()));
        global_systems.push(Box::new(MouseSystem::new()));
        global_systems.push(Box::new(ButtonSystem::new()));
        global_systems.push(Box::new(game_state));
        global_systems
    }
}
