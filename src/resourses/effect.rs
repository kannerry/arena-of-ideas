use std::ops::Deref;

use super::*;

#[derive(Serialize, Deserialize, Clone, Debug, Default, Display)]
pub enum Effect {
    Damage(Option<Expression>),
    Kill,
    UseAbility(String),
    AddStatus(String),
    Debug(Expression),
    Text(Expression),
    Vfx(String),
    List(Vec<Box<EffectWrapped>>),
    AoeFaction(Expression, Box<EffectWrapped>),
    #[default]
    Noop,
}

impl Effect {
    pub fn wrap(self) -> EffectWrapped {
        EffectWrapped {
            effect: self,
            ..default()
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct EffectWrapped {
    pub effect: Effect,
    pub owner: Option<Expression>,
    pub target: Option<Expression>,
    pub vars: Option<Vec<(VarName, Expression)>>,
    pub then: Option<Box<EffectWrapped>>,
}

impl EffectWrapped {
    pub fn invoke(&self, context: &mut Context, world: &mut World) -> Result<()> {
        debug!("Processing {:?}\n{}", &self.effect, context);
        if let Some(vars) = &self.vars {
            for (var, value) in vars {
                context.set_var(*var, value.get_value(context, world)?);
            }
        }
        if let Some(entity) = &self.target {
            context.set_target(entity.get_entity(&context, world)?, world);
        }
        if let Some(entity) = &self.owner {
            context.set_owner(entity.get_entity(&context, world)?, world);
        }
        match &self.effect {
            Effect::Damage(value) => {
                let target = context.get_target().context("Target not found")?;
                let owner = context.get_owner().context("Owner not found")?;
                let value = match value {
                    Some(value) => value.get_int(&context, world)?,
                    None => context
                        .get_var(VarName::Atk, world)
                        .context("Can't find ATK")?
                        .get_int()?,
                };
                debug!("Damage {value} {target:?}");
                VarState::change_int(target, VarName::Hp, -value, world)?;
                VarState::push_back(
                    target,
                    VarName::LastAttacker,
                    Change::new(VarValue::Entity(context.owner())),
                    world,
                );
                Event::DamageTaken {
                    owner: target,
                    value,
                }
                .send(world);
                Event::DamageDealt {
                    owner,
                    target,
                    value,
                }
                .send(world);
                Pools::get_vfx("text", world)
                    .clone()
                    .set_var(
                        VarName::Position,
                        VarState::get(context.target(), world).get_value_last(VarName::Position)?,
                    )
                    .set_var(VarName::Text, VarValue::String(format!("-{value}")))
                    .set_var(VarName::Color, VarValue::Color(Color::ORANGE_RED))
                    .unpack(world)?;
            }
            Effect::Kill => {
                let target = context.get_target().context("Target not found")?;
                VarState::change_int(target, VarName::Hp, -9999999, world)?;
                Pools::get_vfx("text", world)
                    .clone()
                    .set_var(
                        VarName::Position,
                        VarState::get(context.target(), world).get_value_last(VarName::Position)?,
                    )
                    .set_var(VarName::Text, VarValue::String(format!("Kill")))
                    .set_var(VarName::Color, VarValue::Color(Color::RED))
                    .unpack(world)?;
            }
            Effect::Debug(msg) => {
                let msg = msg.get_string(&context, world)?;
                debug!("Debug effect: {msg}");
            }
            Effect::Noop => {}
            Effect::UseAbility(ability) => {
                let effect = Pools::get_ability(&ability, world).effect.clone();
                ActionPlugin::push_front(effect, context.clone(), world);
                Pools::get_vfx("text", world)
                    .clone()
                    .set_var(
                        VarName::Position,
                        VarState::get(context.target(), world).get_value_last(VarName::Position)?,
                    )
                    .set_var(VarName::Text, VarValue::String(format!("Use {ability}")))
                    .set_var(VarName::Color, VarValue::Color(Color::PURPLE))
                    .unpack(world)?;
            }
            Effect::AddStatus(status) => {
                let charges = context
                    .get_var(VarName::Charges, world)
                    .unwrap_or(VarValue::Int(1))
                    .get_int()?;
                Status::change_charges(&status, context.target(), charges, world)?;
                Pools::get_vfx("text", world)
                    .clone()
                    .set_var(
                        VarName::Position,
                        VarState::get(context.target(), world).get_value_last(VarName::Position)?,
                    )
                    .set_var(
                        VarName::Text,
                        VarValue::String(format!("gain {status} x{charges}")),
                    )
                    .set_var(VarName::Color, VarValue::Color(Color::TEAL))
                    .unpack(world)?;
            }
            Effect::List(effects) => {
                for effect in effects {
                    ActionPlugin::push_front(effect.deref().clone(), context.clone(), world)
                }
            }
            Effect::AoeFaction(faction, effect) => {
                for unit in UnitPlugin::collect_faction(faction.get_faction(context, world)?, world)
                {
                    ActionPlugin::push_front(
                        effect.deref().clone(),
                        context.clone().set_target(unit, world).take(),
                        world,
                    )
                }
            }
            Effect::Text(text) => {
                let text = text.get_string(context, world)?;
                Pools::get_vfx("text", world)
                    .clone()
                    .set_var(
                        VarName::Position,
                        VarValue::Vec2(UnitPlugin::get_unit_position(context.owner(), world)?),
                    )
                    .set_var(VarName::Text, VarValue::String(text))
                    .set_var(VarName::Color, VarValue::Color(Color::PINK))
                    .unpack(world)?;
            }
            Effect::Vfx(name) => {
                let owner_pos = UnitPlugin::get_unit_position(context.owner(), world)?;
                let delta = UnitPlugin::get_unit_position(context.target(), world)? - owner_pos;

                Pools::get_vfx(name, world)
                    .clone()
                    .attach_context(context)
                    .set_var(VarName::Delta, VarValue::Vec2(delta))
                    .set_var(VarName::Position, VarValue::Vec2(owner_pos))
                    .set_var(
                        VarName::Color,
                        context
                            .get_var(VarName::Color, world)
                            .context("Color not found in context")?,
                    )
                    .unpack(world)?;
            }
        }
        if let Some(then) = &self.then {
            ActionPlugin::push_front(then.deref().clone(), context.clone(), world)
        }
        Ok(())
    }
}
