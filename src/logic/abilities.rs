use super::*;

impl Game {
    pub fn process_abilities(&mut self) {
        for unit in &mut self.units {
            if let Some(time) = &mut unit.ability_cooldown {
                *time -= self.delta_time;
                if *time < Time::new(0.0) {
                    unit.ability_cooldown = None;
                }
            }
        }
        for key in self.pressed_keys.drain(..) {
            for unit in &mut self.units {
                let template = &self.assets.units[&unit.unit_type];
                if unit.ability_cooldown.is_some() {
                    continue;
                }
                if unit.faction != Faction::Player {
                    continue;
                }
                if unit
                    .statuses
                    .iter()
                    .any(|status| matches!(status, Status::Freeze))
                {
                    continue;
                }
                let ability = match template.abilities.get(&key) {
                    Some(ability) => ability,
                    None => continue,
                };
                unit.ability_cooldown = Some(ability.cooldown);
                for effect in &ability.effects {
                    self.effects.push(QueuedEffect {
                        effect: effect.clone(),
                        caster: Some(unit.id),
                        target: Some(unit.id),
                    });
                }
            }
        }
    }
}