use colored::{Color, ColoredString};

use super::*;

#[derive(Debug, AssetCollection, Resource)]
pub struct TestScenarios {
    #[asset(key = "test.scenarios", collection(typed, mapped))]
    pub tests_handles: HashMap<String, Handle<TestScenario>>,
}

#[derive(Deserialize, TypeUuid, TypePath, Debug, Clone)]
#[uuid = "d112970f-9d3f-412d-b7a3-25db4f52c6b8"]
pub struct TestScenario {
    pub left: PackedTeam,
    pub right: PackedTeam,
    pub condition: Expression,
}

impl TestScenario {
    pub fn run(self, world: &mut World) -> Result<bool> {
        let result = match SimulationPlugin::run(self.left, self.right, world) {
            Ok(_) => self.condition.get_bool(&Context::default(), world),
            Err(e) => Err(anyhow!("{e}")),
        };
        SimulationPlugin::clear(world);
        result
    }
}

pub struct TestPlugin;

impl Plugin for TestPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::BattleTest), Self::run_tests);
    }
}

impl TestPlugin {
    pub fn run_tests(world: &mut World) {
        // PersistentData::save_last_state(GameState::TestsLoading, world);
        let scenarios = Self::get_all_scenarios(world);
        let mut failure: Vec<ColoredString> = default();
        let mut success: Vec<ColoredString> = default();
        let path_color = Color::TrueColor {
            r: 50,
            g: 50,
            b: 50,
        };
        for (path, scenario) in scenarios {
            match scenario.run(world) {
                Ok(value) => debug!(
                    "Test run {}",
                    match value {
                        true => {
                            let str =
                                format!("{} {}", "Success".bold(), path.color(path_color)).green();
                            success.push(str.clone());
                            str
                        }
                        false => {
                            let str = format!(
                                "{} {}",
                                "Condition Failure".bold(),
                                path.color(path_color)
                            )
                            .red();
                            failure.push(str.clone());
                            str
                        }
                    }
                ),
                Err(err) => {
                    let str = format!(
                        "{} {}\n{}",
                        "Error".bold(),
                        path.color(path_color),
                        err.to_string()
                    )
                    .red();
                    failure.push(str.clone());
                    debug!("Test fail: {}", str.clone())
                }
            }
        }
        debug!(
            "Tests run complete with {} successes and {} failures.\n{}\n{}",
            success.len(),
            failure.len(),
            success.into_iter().join("\n"),
            failure.into_iter().join("\n"),
        );
        let mut events = world
            .get_resource_mut::<bevy::prelude::Events<bevy::app::AppExit>>()
            .unwrap();
        events.send(bevy::app::AppExit);
    }

    pub fn get_all_scenarios(world: &World) -> Vec<(String, TestScenario)> {
        world
            .get_resource::<TestScenarios>()
            .unwrap()
            .tests_handles
            .clone()
            .into_iter()
            .map(|(name, handle)| {
                (
                    name,
                    world
                        .get_resource::<Assets<TestScenario>>()
                        .unwrap()
                        .get(&handle)
                        .unwrap()
                        .clone(),
                )
            })
            .collect_vec()
    }
}
