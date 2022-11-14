use super::*;

impl Logic {
    pub fn kill(&mut self, id: Id, context: EffectContext) {
        let unit = self.model.units.get_mut(&id);
        if unit.is_none() {
            return;
        }
        let unit = unit.unwrap();
        unit.permanent_stats.health = 0;
        let unit = self.model.units.get(&id).unwrap();

        for other in self.model.units.iter().filter(|other| other.id != unit.id) {
            for (effect, trigger, vars, status_id, color) in
                other.all_statuses.iter().flat_map(|status| {
                    status.trigger(|trigger| match trigger {
                        StatusTrigger::Scavenge { who, range, clan } => {
                            who.matches(other.faction, unit.faction)
                                && clan.map(|clan| unit.clans.contains(&clan)).unwrap_or(true)
                                && distance_between_units(other, unit) > *range
                        }
                        StatusTrigger::DetectDeath { condition } => {
                            Self::check_condition(&self.model, condition, &context)
                        }
                        _ => false,
                    })
                })
            {
                self.effects.push_back(
                    EffectContext {
                        owner: other.id,
                        creator: unit.id,
                        target: unit.id,
                        status_id: Some(status_id),
                        vars: context.vars.clone(),
                        queue_id: context.queue_id.clone(),
                        color,
                        ..context
                    },
                    effect,
                )
            }
        }
    }
    pub fn process_deaths(&mut self) {
        let ids = self.model.units.ids().copied().collect::<Vec<_>>();
        for id in ids {
            let unit = self.model.units.get(&id).unwrap();
            if unit.permanent_stats.health <= 0 {
                self.model.dead_units.insert(unit.clone());
                for (effect, trigger, vars, status_id, color) in
                    unit.all_statuses.iter().flat_map(|status| {
                        status.trigger(|trigger| matches!(trigger, StatusTrigger::Death))
                    })
                {
                    let context = EffectContext {
                        owner: unit.id,
                        creator: unit.id,
                        target: unit.id,
                        vars,
                        status_id: Some(status_id),
                        color,
                        queue_id: None,
                    };
                    self.effects.push_back(context, effect);
                }
                let unit_position = unit.clone().position;
                self.update_positions(unit_position);
            }
        }

        self.model
            .units
            .retain(|unit| unit.permanent_stats.health > 0);
    }
    fn update_positions(&mut self, unit_position: Position) {
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
