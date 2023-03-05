use super::*;

pub struct GameStateSystem {
    pub systems: HashMap<GameState, Vec<Box<dyn System>>>,
}

#[derive(PartialEq, Eq, Hash, Clone)]
pub enum GameState {
    MainMenu,
    Shop,
    Battle,
    Gallery,
    GameOver,
}

impl System for GameStateSystem {
    fn update(&mut self, world: &mut legion::World, resources: &mut Resources) {
        match resources.current_state {
            GameState::MainMenu => {
                resources.transition_state = GameState::Shop;
                // if !resources.down_keys.is_empty() {
                //     self.transition = GameState::Gallery;
                // }
            }
            GameState::Battle => {
                if resources.down_keys.contains(&geng::Key::R) {
                    resources.cassette.clear();
                    resources.rounds.next_round -= 1;
                    BattleSystem::run_battle(world, resources);
                }
                if resources.cassette.head > resources.cassette.length() + 2.0 {
                    BattleSystem::finish_battle(world, resources);
                }
            }
            GameState::Shop => {
                if resources.down_keys.contains(&geng::Key::Space) {
                    resources.transition_state = GameState::Battle;
                }

                if resources.down_keys.contains(&geng::Key::G) {
                    resources.transition_state = GameState::Gallery;
                }

                if resources.down_keys.contains(&geng::Key::O) {
                    resources.transition_state = GameState::GameOver;
                }
            }
            GameState::Gallery => {
                if resources.down_keys.contains(&geng::Key::G) {
                    resources.transition_state = GameState::Shop;
                }
            }
            GameState::GameOver => {
                if resources.down_keys.contains(&geng::Key::Enter) {
                    resources.transition_state = GameState::Shop;
                }
            }
        }
        self.systems
            .get_mut(&resources.current_state)
            .and_then(|systems| {
                Some(
                    systems
                        .iter_mut()
                        .for_each(|system| system.update(world, resources)),
                )
            });

        self.transition(world, resources);
    }

    fn draw(
        &self,
        world: &legion::World,
        resources: &mut Resources,
        framebuffer: &mut ugli::Framebuffer,
    ) {
        self.systems
            .get(&resources.current_state)
            .and_then(|systems| {
                Some(
                    systems
                        .iter()
                        .for_each(|system| system.draw(world, resources, framebuffer)),
                )
            });
    }

    fn ui<'a>(
        &'a mut self,
        cx: &'a ui::Controller,
        world: &mut legion::World,
        resources: &mut Resources,
    ) -> Box<dyn ui::Widget + 'a> {
        if let Some(widgets) = self
            .systems
            .get_mut(&resources.current_state)
            .and_then(|systems| {
                Some(
                    systems
                        .iter_mut()
                        .map(|system| system.ui(cx, world, resources))
                        .collect_vec(),
                )
            })
            .and_then(|widgets| Some(widgets))
        {
            if !widgets.is_empty() {
                return Box::new(geng::ui::stack(widgets));
            }
        }
        Box::new(ui::Void)
    }
}

impl GameStateSystem {
    pub fn new(state: GameState) -> Self {
        Self { systems: default() }
    }

    pub fn add_systems(&mut self, state: GameState, value: Vec<Box<dyn System>>) {
        let mut systems = self.systems.remove(&state).unwrap_or_default();
        systems.extend(value.into_iter());
        self.systems.insert(state, systems);
    }

    fn transition(&mut self, world: &mut legion::World, resources: &mut Resources) {
        if resources.current_state == resources.transition_state {
            return;
        }
        // transition from
        match resources.current_state {
            GameState::MainMenu => {}
            GameState::Shop => {
                resources.cassette.clear();
                ShopSystem::clear(world, resources);
            }
            GameState::Battle => {
                resources.cassette.clear();
                WorldSystem::set_var(world, VarName::IsBattle, &Var::Float(0.0));
                Event::BattleOver.send(resources, world);
            }
            GameState::Gallery => {
                resources.cassette.clear();
                resources.action_queue.clear();
                resources.camera.fov = resources.options.fov;
                WorldSystem::set_var(world, VarName::FieldPosition, &Var::Vec2(vec2(0.0, 0.0)));
            }
            GameState::GameOver => {
                resources.camera.fov = resources.options.fov;
                resources.camera.center = vec2::ZERO;
            }
        }

        //transition to
        match resources.transition_state {
            GameState::MainMenu => {}
            GameState::Battle => {
                WorldSystem::set_var(world, VarName::IsBattle, &Var::Float(1.0));
                BattleSystem::run_battle(world, resources);
            }
            GameState::Shop => {
                ShopSystem::init(world, resources);
            }
            GameState::Gallery => {
                WorldSystem::set_var(world, VarName::FieldPosition, &Var::Vec2(vec2(0.0, 20.0)));
                SlotSystem::clear_world(world);
            }
            GameState::GameOver => {
                resources.camera.fov = resources.options.fov * 0.5;
                resources.camera.center = SlotSystem::get_position(3, &Faction::Team);
                GameOverSystem::init(world, resources);
            }
        }

        resources.current_state = resources.transition_state.clone();
    }
}
