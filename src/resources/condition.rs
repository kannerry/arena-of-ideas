use geng::prelude::rand::random;

use super::*;

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum Condition {
    Always,
    EqualsInt {
        a: ExpressionInt,
        b: ExpressionInt,
    },
    LessInt {
        a: ExpressionInt,
        b: ExpressionInt,
    },
    MoreInt {
        a: ExpressionInt,
        b: ExpressionInt,
    },
    SlotOccupied {
        slot: ExpressionInt,
        faction: Faction,
    },
    IsCorpse {
        entity: ExpressionEntity,
    },
    IsAlive {
        entity: ExpressionEntity,
    },
    Not {
        condition: Box<Condition>,
    },
    And {
        a: Box<Condition>,
        b: Box<Condition>,
    },
    Chance {
        part: f32,
    },
}

impl Condition {
    pub fn calculate(
        &self,
        context: &Context,
        world: &legion::World,
        resources: &Resources,
    ) -> Result<bool, Error> {
        resources.logger.log(
            &format!("Calculating condition {:?} {:?}", self, context),
            &LogContext::Condition,
        );
        match self {
            Condition::EqualsInt { a, b } => Ok(a.calculate(context, world, resources)?
                == b.calculate(context, world, resources)?),
            Condition::LessInt { a, b } => {
                Ok(a.calculate(context, world, resources)?
                    < b.calculate(context, world, resources)?)
            }
            Condition::MoreInt { a, b } => {
                Ok(a.calculate(context, world, resources)?
                    > b.calculate(context, world, resources)?)
            }
            Condition::SlotOccupied { slot, faction } => Ok(SlotSystem::find_unit_by_slot(
                slot.calculate(context, world, resources)? as usize,
                faction,
                world,
            )
            .is_some()),
            Condition::IsCorpse { entity } => Ok(resources
                .unit_corpses
                .contains_key(&entity.calculate(context, world, resources)?)),
            Condition::Not { condition } => Ok(!condition.calculate(context, world, resources)?),
            Condition::And { a, b } => Ok(a.calculate(context, world, resources)?
                && b.calculate(context, world, resources)?),
            Condition::Always => Ok(true),
            Condition::Chance { part } => Ok(random::<f32>() < *part),
            Condition::IsAlive { entity } => {
                if let Ok(context) = ContextSystem::try_get_context(
                    entity.calculate(context, world, resources)?,
                    world,
                ) {
                    let vars = context.vars;
                    Ok(vars.get_int(&VarName::HpValue) > vars.get_int(&VarName::HpDamage))
                } else {
                    Ok(false)
                }
            }
        }
    }
}
