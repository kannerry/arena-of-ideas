use rand::seq::IteratorRandom;
use spacetimedb_sdk::identity::credentials;

use super::*;
pub struct MainMenuPlugin;

#[derive(Resource, Default)]
struct MainMenuState {
    connected: bool,
}

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (Self::ui, Self::update).run_if(in_state(GameState::MainMenu)),
        )
        .init_resource::<MainMenuState>();
    }
}

impl MainMenuPlugin {
    fn update(world: &mut World) {
        if LoginPlugin::is_connected() {
            if let Ok(creds) = credentials() {
                let mut state = world.resource_mut::<MainMenuState>();
                if !state.connected {
                    state.connected = true;
                    Self::handle_connection(creds, world);
                }
            }
        }
    }

    fn handle_connection(creds: Credentials, world: &mut World) {
        LoginPlugin::save_credentials(creds, world)
    }

    fn ui(world: &mut World) {
        let ctx = &egui_context(world);

        window("MAIN MENU")
            .set_width(400.0)
            .anchor(Align2::CENTER_CENTER, [0.0, 0.0])
            .show(ctx, |ui| {
                frame(ui, |ui| {
                    if LoginPlugin::is_connected() {
                        let identity = identity().unwrap();
                        let has_own = TableTower::filter_by_creator(identity.clone())
                            .next()
                            .is_some();
                        ui.set_enabled(has_own);
                        if ui
                            .button("RANDOM TOWER")
                            .on_hover_text("Play tower that belongs to other random player")
                            .on_disabled_hover_text(
                                "Create at least one tower, click New Tower and beat 3+ levels",
                            )
                            .clicked()
                        {
                            if let Some(tower) = TableTower::iter()
                                .filter(|l| l.owner != identity)
                                .choose(&mut thread_rng())
                            {
                                let team = match tower.status {
                                    module_bindings::TowerStatus::Fresh(team)
                                    | module_bindings::TowerStatus::Beaten(team) => {
                                        ron::from_str::<PackedTeam>(&team).unwrap()
                                    }
                                };
                                let save = Save {
                                    mode: GameMode::RandomTower { tower_id: tower.id },
                                    climb: TowerClimb {
                                        team: default(),
                                        levels: tower.levels,
                                        defeated: default(),
                                        shop: ShopState::new(world),
                                        owner_team: Some(team),
                                    },
                                };
                                save.save(world).unwrap();
                                GameState::change(GameState::Shop, world);
                            }
                        }
                    } else {
                        ui.label("DISCONNECTED");
                        if ui.button("CONNECT").clicked() {
                            LoginPlugin::connect(world);
                        }
                    }
                });

                frame(ui, |ui| {
                    let enabled = Save::get(world).is_ok();
                    ui.set_enabled(enabled);
                    let btn = if enabled {
                        ui.button_primary("CONTINUE")
                    } else {
                        ui.button("CONTINUE")
                    };
                    if btn.clicked() {
                        GameState::change(GameState::Shop, world);
                    }
                });
                frame(ui, |ui| {
                    if ui
                        .button("NEW TOWER")
                        .on_hover_text(
                            "Generate new levels infinitely until defeat.
New levels generated considering your teams strength",
                        )
                        .clicked()
                    {
                        Save {
                            mode: GameMode::NewTower,
                            climb: TowerClimb {
                                levels: Options::get_initial_tower(world).levels.clone(),
                                shop: ShopState::new(world),
                                team: default(),
                                owner_team: default(),
                                defeated: default(),
                            },
                        }
                        .save(world)
                        .unwrap();
                        GameState::change(GameState::Shop, world);
                    }
                });

                frame(ui, |ui| {
                    if ui.button("HERO GALLERY").clicked() {
                        GameState::change(GameState::HeroGallery, world);
                    }
                });
                if cfg!(debug_assertions) {
                    frame(ui, |ui| {
                        ui.columns(3, |ui| {
                            ui[0].vertical_centered_justified(|ui| {
                                if ui.button("CUSTOM").clicked() {
                                    GameState::change(GameState::CustomBattle, world);
                                }
                            });
                            ui[1].vertical_centered_justified(|ui| {
                                if ui.button("EDITOR").clicked() {
                                    GameState::change(GameState::HeroEditor, world);
                                }
                            });
                            ui[2].vertical_centered_justified(|ui| {
                                if ui.button("TESTS").clicked() {
                                    GameState::change(GameState::TestsLoading, world);
                                }
                            });
                        })
                    });
                }
            });
    }
}
