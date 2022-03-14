use super::*;

impl Logic<'_> {
    pub fn process_deaths(&mut self) {
        self.process_units(Self::process_unit_death);
        for unit in &self.model.units {
            if unit.hp <= Health::new(0.0) {
                self.model.dead_units.insert(unit.clone());
            }
        }
        self.model.units.retain(|unit| unit.hp > Health::new(0.0));
    }
    fn process_unit_death(&mut self, unit: &mut Unit) {
        if unit.hp <= Health::new(0.0) {
            for trigger in &unit.triggers {
                if let UnitTrigger::Death(effect) = trigger {
                    self.effects.push_back(QueuedEffect {
                        effect: effect.clone(),
                        context: EffectContext {
                            caster: Some(unit.id),
                            from: Some(unit.id),
                            target: Some(unit.id),
                        },
                    });
                }
            }
        }
    }
}
