#![allow(dead_code, unused_mut, unused_imports, unused_variables)]
#![deny(unconditional_recursion)]

use geng::prelude::*;

mod assets;
mod logic;
mod model;
mod render;

use assets::*;
use logic::*;
use model::*;

type Health = R32;
type Time = R32;
type Coord = R32;
type Id = i64;

pub struct Game {
    next_id: Id,
    assets: Assets,
    geng: Geng,
    camera: geng::Camera2d,
    delta_time: Time,
    units: Collection<Unit>,
    spawning_units: Collection<Unit>,
    projectiles: Collection<Projectile>,
}

impl Game {
    pub fn new(geng: &Geng, assets: Assets) -> Self {
        let mut game = Self {
            next_id: 0,
            assets,
            geng: geng.clone(),
            camera: geng::Camera2d {
                center: vec2(0.0, 0.0),
                rotation: 0.0,
                fov: 10.0,
            },
            delta_time: Time::new(0.0),
            units: Collection::new(),
            spawning_units: Collection::new(),
            projectiles: Collection::new(),
        };
        for unit_type in &game.assets.config.player.clone() {
            let template = game.assets.units[unit_type].clone();
            game.spawn_unit(&template, Faction::Player, Vec2::ZERO);
        }
        game
    }
}

impl geng::State for Game {
    fn update(&mut self, delta_time: f64) {
        self.update(Time::new(delta_time as _));
    }
    fn draw(&mut self, framebuffer: &mut ugli::Framebuffer) {
        self.draw(framebuffer);
    }
}

fn main() {
    logger::init().unwrap();
    geng::setup_panic_handler();
    let geng = Geng::new("Arena of Ideas");
    geng::run(
        &geng,
        geng::LoadingScreen::new(
            &geng,
            geng::EmptyLoadingScreen,
            <Assets as geng::LoadAsset>::load(&geng, static_path().to_str().unwrap()),
            {
                let geng = geng.clone();
                move |assets| {
                    let assets = assets.expect("Failed to load assets");
                    Game::new(&geng, assets)
                }
            },
        ),
    );
}
