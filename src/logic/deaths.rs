use super::*;

impl Logic<'_> {
    pub fn kill(&mut self, id: Id) {
        let unit = self.model.units.get_mut(&id).unwrap();
        unit.stats.health = Health::new(0.0);
        let unit = self.model.units.get(&id).unwrap();

        for (effect, vars, status_id) in unit
            .all_statuses
            .iter()
            .flat_map(|status| (status.trigger(|trigger| matches!(trigger, StatusTrigger::Death))))
        {
            self.effects.push_front(QueuedEffect {
                effect,
                context: EffectContext {
                    caster: Some(unit.id),
                    from: Some(unit.id),
                    target: Some(unit.id),
                    vars,
                    status_id: Some(status_id),
                },
            });
        }

        for other in self.model.units.iter().filter(|other| other.id != unit.id) {
            for (effect, vars, status_id) in other.all_statuses.iter().flat_map(|status| {
                status.trigger(|trigger| match trigger {
                    StatusTrigger::Scavenge { who, range, clan } => {
                        who.matches(other.faction, unit.faction)
                            && clan.map(|clan| unit.clans.contains(&clan)).unwrap_or(true)
                            && distance_between_units(other, unit) > *range
                    }
                    _ => false,
                })
            }) {
                self.effects.push_front(QueuedEffect {
                    effect,
                    context: EffectContext {
                        caster: Some(other.id),
                        from: Some(other.id),
                        target: Some(unit.id),
                        vars,
                        status_id: Some(status_id),
                    },
                })
            }
        }
    }
    pub fn process_deaths(&mut self) {
        let ids = self.model.units.ids().copied().collect::<Vec<_>>();
        for id in ids {
            let unit = self.model.units.get(&id).unwrap();
            if unit.stats.health <= Health::ZERO {
                self.model.dead_units.insert(unit.clone());
                let unit_position = unit.position;
                self.update_positions(id, unit_position);
            }
        }
        self.model
            .units
            .retain(|unit| unit.stats.health > Health::ZERO);
    }
    fn update_positions(&mut self, unit_id: Id, unit_position: Position) {
        let mut move_vertically = false;
        for other in self.model.units.iter_mut().filter(|other| {
            other.id != unit_id
                && other.position.side == unit_position.side
                && other.position.x == unit_position.x
                && other.position.height > unit_position.height
        }) {
            // Move vertically
            move_vertically = true;
            other.position.height -= 1;
        }
        if !move_vertically {
            // Move horizontally
            self.model
                .units
                .iter_mut()
                .filter(|other| {
                    other.position.side == unit_position.side && other.position.x > unit_position.x
                })
                .for_each(|other| other.position.x -= 1);
        }
    }
}
