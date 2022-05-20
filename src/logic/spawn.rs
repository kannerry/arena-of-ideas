use super::*;

impl Logic<'_> {
    /// Spawns the unit and returns its id
    pub fn spawn_unit(
        &mut self,
        unit_type: &UnitType,
        faction: Faction,
        position: Vec2<Coord>,
    ) -> Id {
        let mut template = &self.model.unit_templates[unit_type];
        let id = self.model.next_id;

        let mut unit = Unit::new(&template, id, unit_type.clone(), faction, position);
        for clan in &template.clans {
            let clan_members = *self
                .model
                .config
                .clans
                .get(clan)
                .expect(&format!("Failed to find clan members of {clan:?}"));
            clan.apply_effects(&mut unit, &self.model.clan_effects, clan_members);
        }

        self.model.next_id += 1;
        self.model.spawning_units.insert(unit);
        id
    }
    pub fn process_spawns(&mut self) {
        let mut new_units = Vec::new();
        for unit in &mut self.model.spawning_units {
            if let Some(time) = &mut unit.spawn_animation_time_left {
                *time -= self.delta_time;
                if *time <= Time::new(0.0) {
                    unit.spawn_animation_time_left = None;
                    new_units.push(unit.clone());
                }
            }
        }
        for mut unit in new_units {
            for status in &unit.all_statuses {
                if let Status::OnSpawn(status) = status {
                    self.effects.push_back(QueuedEffect {
                        effect: status.effect.clone(),
                        context: EffectContext {
                            caster: Some(unit.id),
                            from: Some(unit.id),
                            target: Some(unit.id),
                            vars: default(),
                        },
                    });
                }
            }
            self.model.units.insert(unit);
        }
        self.model
            .spawning_units
            .retain(|unit| unit.spawn_animation_time_left.is_some());
    }
}
