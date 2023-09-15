use super::*;

#[derive(Debug, Display)]
pub enum Event {
    DamageTaken { unit: Entity, value: i32 },
    BattleStart,
    TurnStart,
    BeforeStrike(Entity),
    Death(Entity),
    Kill { killer: Entity, target: Entity },
}

impl Event {
    pub fn send(self, world: &mut World) {
        debug!("Send event {self}");
        let mut context = Context::new_named(self.to_string());
        let statuses = match &self {
            Event::DamageTaken { unit, value } => {
                context.set_var(VarName::Value, VarValue::Int(*value));
                Status::collect_entity_statuses(*unit, world)
            }
            Event::BattleStart | Event::TurnStart | Event::Death(..) => {
                Status::collect_all_statuses(world)
            }
            Event::BeforeStrike(unit) => Status::collect_entity_statuses(*unit, world),
            Event::Kill { killer, .. } => Status::collect_entity_statuses(*killer, world),
        };
        Status::notify(statuses, &self, &context, world);
    }
}
