use super::*;

impl Game {
    pub fn process_aoe_effect(
        &mut self,
        QueuedEffect {
            target,
            caster,
            effect,
        }: QueuedEffect<AoeEffect>,
    ) {
        let caster = caster
            .and_then(|id| self.units.get(&id).or(self.dead_units.get(&id)))
            .expect("Caster not found");
        let caster_faction = caster.faction;
        let center = target
            .and_then(|id| {
                self.units
                    .get(&id)
                    .map(|unit| unit.position)
                    .or(self.dead_time_bombs.get(&id).map(|bomb| bomb.position))
            })
            .expect("Target not found");
        self.render.add_text(center, "AOE", Color::RED);
        for unit in &self.units {
            if (unit.position - center).len() - unit.radius() > effect.radius {
                continue;
            }
            match effect.filter {
                TargetFilter::Allies => {
                    if unit.faction != caster_faction {
                        continue;
                    }
                }
                TargetFilter::Enemies => {
                    if unit.faction == caster_faction {
                        continue;
                    }
                }
                TargetFilter::All => {}
            }
            for new_effect in &effect.effects {
                self.effects.push(QueuedEffect {
                    effect: new_effect.clone(),
                    caster: Some(caster.id),
                    target: Some(unit.id),
                });
            }
        }
    }
}
