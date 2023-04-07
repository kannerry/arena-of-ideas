use super::*;

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct Options {
    pub fov: f32,
    pub rewind_speed: f32,
    pub rewind_add_speed: f32,
    pub initial_shop_fill: usize,
    pub start_floor: usize,
    pub log: HashMap<LogContext, bool>,
    pub shaders: Shaders,
    pub images: Images,
    pub colors: Colors,
    pub floats: Floats,
    pub custom_game: CustomGameConfig,
    pub walkthrough: bool,
    pub player_team_name: String,
}

impl FileWatcherLoader for Options {
    fn loader(resources: &mut Resources, _: &PathBuf, _: &mut FileWatcherSystem) {
        resources.options = Self::load();
        resources.transition_state = GameState::MainMenu;
    }
}

impl Options {
    pub fn load() -> Options {
        futures::executor::block_on(load_json(static_path().join("options.json"))).unwrap()
    }
}

#[derive(Deserialize, Debug)]
pub struct Shaders {
    pub unit: Shader,
    pub field: Shader,
    pub stats: Shader,
    pub strike: Shader,
    pub text: Shader,
    pub battle_timer: Shader,
    pub curve: Shader,
    pub name: Shader,
    pub slot: Shader,
    pub slot_price: Shader,
    pub money_indicator: Shader,
    pub status_panel: Shader,
    pub definitions_panel: Shader,
    pub status_panel_text: Shader,
    pub definitions_panel_text: Shader,
    pub definitions_panel_title: Shader,
    pub button: Shader,
    pub icon: Shader,
    pub corner_button: Shader,
    pub team_name: Shader,
    pub team_name_intro: Shader,
    pub tape_indicator: Shader,
    pub shop_sell_field: Shader,
    pub battle_score_indicator: Shader,
}

#[derive(Deserialize, Debug)]
pub struct Images {
    pub eye_icon: Image,
    pub money_icon: Image,
    pub pool_icon: Image,
    pub play_icon: Image,
    pub pause_icon: Image,
    pub rewind_forward_icon: Image,
    pub rewind_backward_icon: Image,
    pub refresh_icon: Image,
}

#[derive(Deserialize, Debug)]
pub struct Colors {
    pub faction_colors: HashMap<Faction, Rgba<f32>>,
    pub inactive: Rgba<f32>,
    pub stats_attack_color: Rgba<f32>,
    pub stats_hp_color: Rgba<f32>,
    pub corner_button_color: Rgba<f32>,
    pub corner_button_icon_color: Rgba<f32>,
    pub btn_normal: Rgba<f32>,
    pub btn_active: Rgba<f32>,
    pub btn_hovered: Rgba<f32>,
    pub damage_text: Rgba<f32>,
    pub text_add_color: Rgba<f32>,
    pub text_remove_color: Rgba<f32>,
}

#[derive(Deserialize, Debug)]
pub struct CustomGameConfig {
    pub enable: bool,
    pub light: Option<ReplicatedTeam>,
    pub dark: Option<ReplicatedTeam>,
}

#[derive(Deserialize, Debug)]
pub struct Floats {
    pub slots_battle_team_position: vec2<f32>,
    pub slots_shop_team_position: vec2<f32>,
    pub slots_striker_position: vec2<f32>,
    pub slots_battle_team_spacing: vec2<f32>,
    pub slots_shop_spacing: vec2<f32>,
    pub slots_battle_team_scale: f32,
    pub slots_striker_scale: f32,
}
