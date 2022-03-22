use std::collections::VecDeque;

use super::*;

mod abilities;
mod actions;
mod alliances;
mod collisions;
mod deaths;
mod effects;
mod movement;
mod projectiles;
mod spawn;
mod statuses;
mod targeting;
mod time_bombs;
mod util;
mod waves;

pub use effects::*;
pub use util::*;

pub struct Logic<'a> {
    pub model: &'a mut Model,
    pub delta_time: Time,
    pub effects: VecDeque<QueuedEffect<Effect>>,
    pub pressed_keys: Vec<Key>,
    pub render: Option<&'a mut Render>,
}

impl<'a> Logic<'a> {
    pub fn initialize(model: &'a mut Model, config: &Config) {
        let mut logic = Self {
            model,
            delta_time: Time::new(0.0),
            effects: VecDeque::new(),
            pressed_keys: Vec::new(),
            render: None,
        };
        logic.spawn_player(config);
        logic.process();
    }
    pub fn process(&mut self) {
        self.process_statuses();
        self.process_time_bombs();
        self.process_spawns();
        self.process_abilities();
        self.process_movement();
        self.process_collisions();
        self.process_targeting();
        self.process_actions();
        self.process_cooldowns();
        self.process_projectiles();
        self.check_next_wave();
        while self.model.free_revives > 0 {
            if let Some(unit) = self
                .model
                .dead_units
                .iter()
                .filter(|unit| unit.faction == Faction::Player)
                .next()
            {
                self.effects.push_back(QueuedEffect {
                    effect: Effect::Revive(Box::new(ReviveEffect {
                        hp: DamageValue::relative(100.0),
                    })),
                    context: EffectContext {
                        caster: None,
                        from: None,
                        target: Some(unit.id),
                    },
                });
                self.model.free_revives -= 1;
            } else {
                break;
            }
        }
        self.process_effects();
        self.process_deaths();
    }
    fn process_units(&mut self, mut f: impl FnMut(&mut Self, &mut Unit)) {
        let ids: Vec<Id> = self.model.units.ids().copied().collect();
        for id in ids {
            let mut unit = self.model.units.remove(&id).unwrap();
            f(self, &mut unit);
            self.model.units.insert(unit);
        }
    }
    fn spawn_player(&mut self, config: &Config) {
        let mut to_spawn = config
            .player
            .iter()
            .map(|unit| (unit, self.model.unit_templates[unit].clone()))
            .collect::<Vec<_>>();

        let mut alliance_counts = HashMap::new();
        for (_, unit) in &to_spawn {
            for alliance in &unit.alliances {
                *alliance_counts.entry(*alliance).or_default() += 1;
            }
        }

        // Apply effects
        for (_, unit) in &mut to_spawn {
            for alliance in unit.alliances.clone() {
                alliance.apply(unit, alliance_counts[&alliance]);
            }
        }

        // Spawn
        for (unit_type, template) in to_spawn {
            self.spawn_template(unit_type, template, Faction::Player, Vec2::ZERO);
        }

        if alliance_counts
            .get(&Alliance::Healers)
            .copied()
            .unwrap_or(0)
            >= 6
        {
            self.model.free_revives += 1;
        }
    }
}

impl Game {
    pub fn update(&mut self, delta_time: Time) {
        let mut logic = Logic {
            model: &mut self.model,
            delta_time,
            effects: VecDeque::new(),
            pressed_keys: mem::take(&mut self.pressed_keys),
            render: Some(&mut self.render),
        };
        logic.process();
    }
}
