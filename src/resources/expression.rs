use bevy_egui::egui::DragValue;
use convert_case::{Case, Casing};
use hex::encode;
use rand::{
    seq::{IteratorRandom, SliceRandom},
    Rng, SeedableRng,
};
use rand_chacha::ChaCha8Rng;
use std::{collections::hash_map::DefaultHasher, f32::consts::PI};
use std::{
    collections::VecDeque,
    hash::{Hash, Hasher},
};

use super::*;

#[derive(Serialize, Deserialize, Debug, Clone, Default, PartialEq, EnumIter, AsRefStr)]
pub enum Expression {
    #[default]
    Zero,
    GameTime,
    PI,
    PI2,
    Age,
    SlotPosition,
    OwnerFaction,
    OppositeFaction,
    Beat,
    Index,

    Owner,
    Caster,
    Target,
    Status,
    RandomUnit,
    RandomAdjacentUnit,
    RandomAlly,
    RandomEnemy,
    AllAllyUnits,
    AllEnemyUnits,
    AllUnits,
    AllOtherUnits,
    AdjacentUnits,

    Float(f32),
    Int(i32),
    Bool(bool),
    String(String),
    Hex(String),
    Faction(Faction),
    OwnerState(VarName),
    OwnerStateLast(VarName),
    TargetState(VarName),
    TargetStateLast(VarName),
    Context(VarName),
    AbilityContext(String, VarName),
    AbilityState(String, VarName),
    Value(VarValue),
    RandomStatusAlly(String),
    RandomStatusEnemy(String),
    AllStatusAllies(String),
    AllStatusEnemies(String),
    HasStatus(String),

    Vec2(f32, f32),

    Vec2E(Box<Expression>),
    StringInt(Box<Expression>),
    StringFloat(Box<Expression>),
    StringVec(Box<Expression>),
    IntFloat(Box<Expression>),
    Sin(Box<Expression>),
    Cos(Box<Expression>),
    Sign(Box<Expression>),
    Fract(Box<Expression>),
    Floor(Box<Expression>),
    UnitVec(Box<Expression>),
    Even(Box<Expression>),
    Abs(Box<Expression>),
    SlotUnit(Box<Expression>),
    FactionCount(Box<Expression>),
    StatusCharges(Box<Expression>),
    FilterUnits(Box<Expression>),
    FilterMaxEnemy(Box<Expression>),
    FilterMaxAlly(Box<Expression>),
    FindUnit(Box<Expression>),
    UnitCount(Box<Expression>),
    ToInt(Box<Expression>),
    RandomFloat(Box<Expression>),
    RandomFloatUnit(Box<Expression>),
    RandomEnemySubset(Box<Expression>),
    Parent(Box<Expression>),
    StatusEntity(String, Box<Expression>),
    StateLast(VarName, Box<Expression>),

    Vec2EE(Box<Expression>, Box<Expression>),
    Sum(Box<Expression>, Box<Expression>),
    Sub(Box<Expression>, Box<Expression>),
    Mul(Box<Expression>, Box<Expression>),
    Div(Box<Expression>, Box<Expression>),
    GreaterThen(Box<Expression>, Box<Expression>),
    LessThen(Box<Expression>, Box<Expression>),
    Min(Box<Expression>, Box<Expression>),
    Max(Box<Expression>, Box<Expression>),
    Equals(Box<Expression>, Box<Expression>),
    And(Box<Expression>, Box<Expression>),
    Or(Box<Expression>, Box<Expression>),

    If(Box<Expression>, Box<Expression>, Box<Expression>),
    Spread(Box<Expression>, Box<Expression>, Box<Expression>),

    WithVar(VarName, Box<Expression>, Box<Expression>),
}
impl Expression {
    pub fn get_value(&self, context: &Context, world: &mut World) -> Result<VarValue> {
        match self {
            Expression::Zero => Ok(VarValue::Int(0)),
            Expression::Float(x) => Ok(VarValue::Float(*x)),
            Expression::Int(x) => Ok(VarValue::Int(*x)),
            Expression::Bool(x) => Ok(VarValue::Bool(*x)),
            Expression::String(x) => Ok(VarValue::String(x.into())),
            Expression::Vec2(x, y) => Ok(VarValue::Vec2(vec2(*x, *y))),
            Expression::Vec2EE(x, y) => Ok(VarValue::Vec2(vec2(
                x.get_float(context, world)?,
                y.get_float(context, world)?,
            ))),
            Expression::Vec2E(x) => {
                let x = x.get_float(context, world)?;
                Ok(VarValue::Vec2(vec2(x, x)))
            }
            Expression::StringInt(x) => {
                Ok(VarValue::String(x.get_int(context, world)?.to_string()))
            }
            Expression::StringFloat(x) => {
                Ok(VarValue::String(x.get_float(context, world)?.to_string()))
            }
            Expression::StringVec(x) => {
                let Vec2 { x, y } = x.get_vec2(context, world)?;
                Ok(VarValue::String(format!("({x:.1}:{y:.1})")))
            }
            Expression::IntFloat(x) => Ok(VarValue::Float(x.get_int(context, world)? as f32)),
            Expression::ToInt(x) => {
                Ok(VarValue::Int(x.get_int(context, world).unwrap_or_default()))
            }
            Expression::Sin(x) => Ok(VarValue::Float(x.get_float(context, world)?.sin())),
            Expression::Cos(x) => Ok(VarValue::Float(x.get_float(context, world)?.cos())),
            Expression::Sign(x) => Ok(VarValue::Float(x.get_float(context, world)?.signum())),
            Expression::Fract(x) => Ok(VarValue::Float(x.get_float(context, world)?.fract())),
            Expression::Floor(x) => Ok(VarValue::Float(x.get_float(context, world)?.floor())),
            Expression::UnitVec(x) => {
                let x = x.get_float(context, world)?;
                let x = vec2(x.cos(), x.sin());
                Ok(VarValue::Vec2(x))
            }
            Expression::Even(x) => {
                let x = x.get_int(context, world)?;
                Ok(VarValue::Bool(x % 2 == 0))
            }
            Expression::RandomFloat(x) => {
                let x = x.get_value(context, world)?;
                let mut hasher = DefaultHasher::new();
                x.hash(&mut hasher);
                let mut rng = ChaCha8Rng::seed_from_u64(hasher.finish());
                Ok(VarValue::Float(rng.gen_range(0.0..1.0)))
            }
            Expression::RandomFloatUnit(x) => Ok(VarValue::Float(
                Expression::RandomFloat(x.clone()).get_float(context, world)? * 2.0 - 1.0,
            )),
            Expression::GameTime => Ok(VarValue::Float(GameTimer::get().play_head())),
            Expression::PI => Ok(VarValue::Float(PI)),
            Expression::PI2 => Ok(VarValue::Float(PI * 2.0)),
            Expression::Sum(a, b) => {
                VarValue::sum(&a.get_value(context, world)?, &b.get_value(context, world)?)
            }
            Expression::Sub(a, b) => {
                VarValue::sub(&a.get_value(context, world)?, &b.get_value(context, world)?)
            }
            Expression::Mul(a, b) => {
                VarValue::mul(&a.get_value(context, world)?, &b.get_value(context, world)?)
            }
            Expression::Div(a, b) => {
                VarValue::div(&a.get_value(context, world)?, &b.get_value(context, world)?)
            }
            Expression::OwnerState(var) => {
                let t = GameTimer::get().play_head();
                VarState::find_value(context.owner(), *var, t, world)
            }
            Expression::TargetState(var) => {
                let t = GameTimer::get().play_head();
                VarState::find_value(context.get_target()?, *var, t, world)
            }
            Expression::StateLast(var, target) => {
                VarState::get(target.get_entity(context, world)?, world).get_value_last(*var)
            }
            Expression::TargetStateLast(var) => {
                VarState::get(context.get_target()?, world).get_value_last(*var)
            }
            Expression::OwnerStateLast(var) => Ok(VarState::get(context.owner(), world)
                .get_value_last(*var)
                .unwrap_or_default()),
            Expression::Age => Ok(VarValue::Float(
                GameTimer::get().play_head() - VarState::find(context.owner(), world).birth,
            )),
            Expression::Context(var) => context.get_var(*var, world),
            Expression::AbilityContext(ability, var) => context.get_ability_var(ability, *var),
            Expression::AbilityState(ability, var) => {
                let faction = context.get_faction(world)?;
                TeamPlugin::get_ability_state(faction, ability, world)
                    .with_context(|| format!("No ability state for {faction} {ability}"))?
                    .get_value_at(*var, GameTimer::get().insert_head())
            }
            Expression::Index => Expression::Context(VarName::Index).get_value(context, world),
            Expression::Owner => Ok(VarValue::Entity(context.get_owner()?)),
            Expression::Caster => Ok(VarValue::Entity(context.get_caster()?)),
            Expression::Target => Ok(VarValue::Entity(context.get_target()?)),
            Expression::Status => Ok(VarValue::Entity(context.get_status()?)),
            Expression::SlotPosition => Ok(VarValue::Vec2(UnitPlugin::get_entity_slot_position(
                context.owner(),
                world,
            )?)),
            Expression::AllUnits => Ok(VarValue::EntityList(
                UnitPlugin::collect_factions(
                    [
                        context.get_faction(world)?,
                        context.get_faction(world)?.opposite(),
                    ]
                    .into(),
                    world,
                )
                .into_iter()
                .map(|(u, _)| u)
                .collect_vec(),
            )),
            Expression::AllOtherUnits => {
                let mut entities = Expression::AllUnits
                    .get_value(context, world)?
                    .get_entity_list()?;
                let owner = context.owner();
                if let Some(p) = entities.iter().position(|e| owner.eq(e)) {
                    entities.remove(p);
                }
                Ok(VarValue::EntityList(entities))
            }
            Expression::AdjacentUnits => {
                let own_slot = context.get_var(VarName::Slot, world)?.get_int()?;
                let faction = context.get_var(VarName::Faction, world)?.get_faction()?;
                let mut min_distance = 999999;
                for unit in UnitPlugin::collect_faction(faction, world) {
                    let state = VarState::get(unit, world);
                    let slot = state.get_int(VarName::Slot)?;
                    let delta = (slot - own_slot).abs();
                    if delta == 0 {
                        continue;
                    }
                    min_distance = min_distance.min(delta);
                }
                Ok(VarValue::EntityList(
                    UnitPlugin::find_unit(faction, (own_slot - min_distance) as usize, world)
                        .into_iter()
                        .chain(UnitPlugin::find_unit(
                            faction,
                            (own_slot + min_distance) as usize,
                            world,
                        ))
                        .collect_vec(),
                ))
            }
            Expression::RandomAdjacentUnit => Ok(VarValue::Entity(
                *Self::AdjacentUnits
                    .get_value(context, world)?
                    .get_entity_list()?
                    .choose(&mut thread_rng())
                    .context("No adjacent units found")?,
            )),
            Expression::AllAllyUnits => Ok(VarValue::EntityList(UnitPlugin::collect_faction(
                context.get_faction(world)?,
                world,
            ))),
            Expression::AllEnemyUnits => Ok(VarValue::EntityList(UnitPlugin::collect_faction(
                context.get_faction(world)?.opposite(),
                world,
            ))),
            Expression::RandomEnemySubset(max) => Ok(VarValue::EntityList(
                Expression::AllEnemyUnits
                    .get_value(context, world)?
                    .get_entity_list()?
                    .choose_multiple(&mut thread_rng(), max.get_int(context, world)? as usize)
                    .copied()
                    .collect_vec(),
            )),
            Expression::SlotUnit(index) => Ok(VarValue::Entity(
                UnitPlugin::find_unit(
                    context.get_var(VarName::Faction, world)?.get_faction()?,
                    index.get_int(context, world)? as usize,
                    world,
                )
                .context("No unit in slot")?,
            )),
            Expression::StatusEntity(status, target) => Ok(VarValue::Entity(
                Status::find_unit_status(target.get_entity(context, world)?, status, world)
                    .with_context(|| format!("Status not found {status} {target}"))?
                    .0,
            )),
            Expression::RandomUnit => Ok(VarValue::Entity(
                UnitPlugin::collect_faction(
                    context.get_var(VarName::Faction, world)?.get_faction()?,
                    world,
                )
                .into_iter()
                .filter(|x| !x.eq(&context.owner()))
                .choose(&mut thread_rng())
                .context("No other units found")?,
            )),
            Expression::RandomAlly => Self::RandomUnit.get_value(
                context.clone().set_var(
                    VarName::Faction,
                    VarValue::Faction(context.get_faction(world)?),
                ),
                world,
            ),
            Expression::RandomEnemy => Self::RandomUnit.get_value(
                context.clone().set_var(
                    VarName::Faction,
                    VarValue::Faction(context.get_faction(world)?.opposite()),
                ),
                world,
            ),
            Expression::OwnerFaction => Ok(VarValue::Faction(context.get_faction(world)?)),
            Expression::OppositeFaction => {
                Ok(VarValue::Faction(context.get_faction(world)?.opposite()))
            }
            Expression::Faction(faction) => Ok(VarValue::Faction(*faction)),
            Expression::FactionCount(faction) => Ok(VarValue::Int(
                UnitPlugin::collect_faction(faction.get_faction(context, world)?, world).len()
                    as i32,
            )),
            Expression::UnitCount(condition) => {
                let faction = context.get_var(VarName::Faction, world)?.get_faction()?;
                let mut cnt = 0;
                for unit in UnitPlugin::collect_faction(faction, world) {
                    let context = Context::from_owner(unit, world);
                    if condition.get_bool(&context, world)? {
                        cnt += 1;
                    }
                }
                Ok(VarValue::Int(cnt))
            }
            Expression::Equals(a, b) => Ok(VarValue::Bool(
                a.get_value(context, world)?
                    .eq(&b.get_value(context, world)?),
            )),
            Expression::And(a, b) => Ok(VarValue::Bool(
                a.get_bool(context, world)? && b.get_bool(context, world)?,
            )),
            Expression::Or(a, b) => Ok(VarValue::Bool(
                a.get_bool(context, world)? || b.get_bool(context, world)?,
            )),
            Expression::Hex(color) => Ok(VarValue::Color(Color::hex(color)?)),
            Expression::StatusCharges(name) => {
                let status_name = name.get_string(context, world)?;
                for status in Status::collect_unit_statuses(context.owner(), world) {
                    let state = VarState::get(status, world);
                    if let Ok(name) = state.get_string(VarName::Name) {
                        if name.eq(&status_name) {
                            return Ok(VarValue::Int(state.get_int(VarName::Charges)?));
                        }
                    }
                }
                Ok(VarValue::Int(0))
            }
            Expression::HasStatus(name) => Ok(VarValue::Bool(
                Expression::StatusCharges(Box::new(Expression::String(name.to_owned())))
                    .get_int(context, world)?
                    > 0,
            )),
            Expression::FilterUnits(condition) => {
                let faction = context.get_faction(world)?;
                Ok(VarValue::EntityList(
                    UnitPlugin::collect_faction(faction, world)
                        .into_iter()
                        .filter(|u| {
                            condition
                                .get_bool(&Context::from_owner(*u, world), world)
                                .unwrap_or_default()
                        })
                        .collect_vec(),
                ))
            }
            Expression::FilterMaxEnemy(value) | Expression::FilterMaxAlly(value) => {
                let faction = if matches!(self, Expression::FilterMaxAlly(..)) {
                    context.get_faction(world)?
                } else {
                    context.get_faction(world)?.opposite()
                };
                let (unit, _) = UnitPlugin::collect_faction(faction, world)
                    .into_iter()
                    .filter_map(
                        |u| match value.get_value(&Context::from_owner(u, world), world) {
                            Ok(v) => Some((u, v)),
                            Err(_) => None,
                        },
                    )
                    .reduce(
                        |(ae, av), (be, bv)| match VarValue::compare(&av, &bv).unwrap() {
                            std::cmp::Ordering::Less => (be, bv),
                            std::cmp::Ordering::Equal | std::cmp::Ordering::Greater => (ae, av),
                        },
                    )
                    .context("Failed to filer max enemy")?;
                Ok(VarValue::Entity(unit))
            }
            Expression::FindUnit(condition) => {
                let faction = context.get_var(VarName::Faction, world)?.get_faction()?;
                let mut units = UnitPlugin::collect_faction(faction, world)
                    .into_iter()
                    .collect_vec();
                units.shuffle(&mut thread_rng());

                let unit = units
                    .into_iter()
                    .find(|u| {
                        condition
                            .get_bool(&Context::from_owner(*u, world), world)
                            .unwrap_or_default()
                    })
                    .context("Failed to find unit")?;
                Ok(VarValue::Entity(unit))
            }
            Expression::RandomStatusAlly(name) => {
                Expression::FindUnit(Box::new(Expression::HasStatus(name.to_owned())))
                    .get_value(context, world)
            }
            Expression::RandomStatusEnemy(name) => {
                Expression::FindUnit(Box::new(Expression::HasStatus(name.to_owned()))).get_value(
                    context.clone().set_var(
                        VarName::Faction,
                        VarValue::Faction(context.get_faction(world)?.opposite()),
                    ),
                    world,
                )
            }
            Expression::AllStatusAllies(name) => {
                Expression::FilterUnits(Box::new(Expression::HasStatus(name.to_owned())))
                    .get_value(context, world)
            }
            Expression::AllStatusEnemies(name) => {
                Expression::FilterUnits(Box::new(Expression::HasStatus(name.to_owned()))).get_value(
                    context.clone().set_var(
                        VarName::Faction,
                        VarValue::Faction(context.get_faction(world)?.opposite()),
                    ),
                    world,
                )
            }
            Expression::Parent(entity) => Ok(VarValue::Entity(
                entity
                    .get_entity(context, world)?
                    .get_parent(world)
                    .with_context(|| format!("Parent not found for {entity}"))?,
            )),
            Expression::Beat => {
                let beat = AudioPlugin::beat_index(world);
                let to_next = AudioPlugin::to_next_beat(world);
                let timeframe = AudioPlugin::beat_timeframe();
                let start = match beat % 2 == 0 {
                    true => -1.0,
                    false => 1.0,
                };
                let start = VarValue::Float(start);
                let t = timeframe - to_next;

                Tween::QuartOut.f(&start, &VarValue::Float(0.0), t, timeframe * 0.5)
            }
            Expression::If(cond, th, el) => {
                if cond.get_bool(context, world)? {
                    th.get_value(context, world)
                } else {
                    el.get_value(context, world)
                }
            }
            Expression::Spread(x, min, max) => {
                let x = x.get_float(context, world)?;
                let min = min.get_float(context, world)?;
                let max = max.get_float(context, world)?;
                Ok(VarValue::Float(min * (1.0 - x) + max * x))
            }
            Expression::GreaterThen(a, b) => Ok(VarValue::Bool(matches!(
                VarValue::compare(&a.get_value(context, world)?, &b.get_value(context, world)?,)?,
                std::cmp::Ordering::Greater
            ))),
            Expression::LessThen(a, b) => Ok(VarValue::Bool(matches!(
                VarValue::compare(&a.get_value(context, world)?, &b.get_value(context, world)?,)?,
                std::cmp::Ordering::Less
            ))),
            Expression::Min(a, b) => Ok(VarValue::min(
                &a.get_value(context, world)?,
                &b.get_value(context, world)?,
            )?),
            Expression::Max(a, b) => Ok(VarValue::max(
                &a.get_value(context, world)?,
                &b.get_value(context, world)?,
            )?),
            Expression::Abs(x) => x.get_value(context, world)?.abs(),
            Expression::Value(v) => Ok(v.clone()),
            Expression::WithVar(var, value, e) => e.get_value(
                context
                    .clone()
                    .set_var(*var, value.get_value(context, world)?),
                world,
            ),
        }
    }

    pub fn get_inner(&mut self) -> Vec<&mut Box<Self>> {
        match self {
            Self::Zero
            | Self::GameTime
            | Self::PI
            | Self::PI2
            | Self::Owner
            | Self::Caster
            | Self::Target
            | Self::Status
            | Self::RandomUnit
            | Self::RandomAdjacentUnit
            | Self::RandomAlly
            | Self::RandomEnemy
            | Self::Age
            | Self::SlotPosition
            | Self::OwnerFaction
            | Self::OppositeFaction
            | Self::Beat
            | Self::AllUnits
            | Self::AllOtherUnits
            | Self::AdjacentUnits
            | Self::AllAllyUnits
            | Self::AllEnemyUnits
            | Self::Index
            | Self::Float(..)
            | Self::Int(..)
            | Self::Bool(..)
            | Self::String(..)
            | Self::Hex(..)
            | Self::Faction(..)
            | Self::OwnerState(..)
            | Self::TargetState(..)
            | Self::OwnerStateLast(..)
            | Self::TargetStateLast(..)
            | Self::Context(..)
            | Self::AbilityContext(..)
            | Self::AbilityState(..)
            | Self::Value(..)
            | Self::Vec2(..)
            | Self::RandomStatusAlly(..)
            | Self::RandomStatusEnemy(..)
            | Self::AllStatusAllies(..)
            | Self::AllStatusEnemies(..)
            | Self::HasStatus(..) => default(),
            Self::StringInt(x)
            | Self::StringFloat(x)
            | Self::StringVec(x)
            | Self::IntFloat(x)
            | Self::ToInt(x)
            | Self::Sin(x)
            | Self::Cos(x)
            | Self::Sign(x)
            | Self::Fract(x)
            | Self::Floor(x)
            | Self::UnitVec(x)
            | Self::Even(x)
            | Self::Abs(x)
            | Self::SlotUnit(x)
            | Self::StatusEntity(_, x)
            | Self::StateLast(_, x)
            | Self::FactionCount(x)
            | Self::StatusCharges(x)
            | Self::FilterMaxEnemy(x)
            | Self::FilterMaxAlly(x)
            | Self::FindUnit(x)
            | Self::Parent(x)
            | Self::UnitCount(x)
            | Self::RandomFloat(x)
            | Self::RandomFloatUnit(x)
            | Self::RandomEnemySubset(x)
            | Self::FilterUnits(x)
            | Self::Vec2E(x) => vec![x],

            Self::Vec2EE(a, b)
            | Self::Sum(a, b)
            | Self::Sub(a, b)
            | Self::Mul(a, b)
            | Self::Div(a, b)
            | Self::GreaterThen(a, b)
            | Self::LessThen(a, b)
            | Self::Min(a, b)
            | Self::Equals(a, b)
            | Self::And(a, b)
            | Self::Or(a, b)
            | Self::Max(a, b)
            | Self::WithVar(_, a, b) => vec![a, b],
            Self::Spread(a, b, c) | Self::If(a, b, c) => vec![a, b, c],
        }
    }

    pub fn set_inner(mut self, mut other: Self) -> Self {
        let inner_self = self.get_inner();
        let inner_other = other.get_inner();
        for (i, e) in inner_self.into_iter().enumerate() {
            if inner_other.len() <= i {
                break;
            }
            let oe = inner_other.get(i).unwrap().as_ref().clone();
            *e = Box::new(oe);
        }
        self
    }

    pub fn get_float(&self, context: &Context, world: &mut World) -> Result<f32> {
        self.get_value(context, world)?.get_float()
    }
    pub fn get_int(&self, context: &Context, world: &mut World) -> Result<i32> {
        self.get_value(context, world)?.get_int()
    }
    pub fn get_vec2(&self, context: &Context, world: &mut World) -> Result<Vec2> {
        self.get_value(context, world)?.get_vec2()
    }
    pub fn get_bool(&self, context: &Context, world: &mut World) -> Result<bool> {
        self.get_value(context, world)?.get_bool()
    }
    pub fn get_string(&self, context: &Context, world: &mut World) -> Result<String> {
        self.get_value(context, world)?.get_string()
    }
    pub fn get_entity(&self, context: &Context, world: &mut World) -> Result<Entity> {
        self.get_value(context, world)?.get_entity()
    }
    pub fn get_faction(&self, context: &Context, world: &mut World) -> Result<Faction> {
        self.get_value(context, world)?.get_faction()
    }
    pub fn get_color(&self, context: &Context, world: &mut World) -> Result<Color> {
        self.get_value(context, world)?.get_color()
    }

    pub fn try_randomize(&mut self, range: f32) -> bool {
        let delta = || (&mut thread_rng()).gen_range(-range..range);
        match self {
            Expression::Float(x) => *x += delta(),
            Expression::Int(x) => *x += delta() as i32,
            Expression::Value(x) => match x {
                VarValue::Float(x) => *x += delta(),
                VarValue::Int(x) => *x += delta() as i32,
                VarValue::Vec2(x) => {
                    x.x += delta();
                    x.y += delta();
                }
                _ => {
                    return false;
                }
            },
            Expression::Vec2(x, y) => {
                *x += delta();
                *y += delta();
            }
            _ => {
                return false;
            }
        }
        true
    }

    pub fn mut_all(&mut self, f: impl Fn(&mut Self)) {
        f(self);
        let mut q: VecDeque<&mut Box<Self>> = VecDeque::from_iter(self.get_inner().into_iter());
        while let Some(t) = q.pop_front() {
            f(t);
            for e in t.get_inner() {
                q.push_back(e);
            }
        }
    }
}

impl EditorNodeGenerator for Expression {
    fn node_color(&self) -> Color32 {
        match self {
            Expression::Zero
            | Expression::GameTime
            | Expression::PI
            | Expression::PI2
            | Expression::Owner
            | Expression::Caster
            | Expression::Target
            | Expression::Status
            | Expression::RandomUnit
            | Expression::RandomAdjacentUnit
            | Expression::RandomAlly
            | Expression::RandomEnemy
            | Expression::Age
            | Expression::SlotPosition
            | Expression::OwnerFaction
            | Expression::AllAllyUnits
            | Expression::AllEnemyUnits
            | Expression::AllUnits
            | Expression::AllOtherUnits
            | Expression::AdjacentUnits
            | Expression::OppositeFaction
            | Expression::Index
            | Expression::Beat => hex_color!("#80D8FF"),

            Expression::Float(_)
            | Expression::Int(_)
            | Expression::Bool(_)
            | Expression::String(_)
            | Expression::Hex(_)
            | Expression::Faction(_)
            | Expression::OwnerState(_)
            | Expression::OwnerStateLast(_)
            | Expression::TargetState(_)
            | Expression::TargetStateLast(_)
            | Expression::Context(_)
            | Expression::AbilityContext(..)
            | Expression::AbilityState(..)
            | Expression::Value(_)
            | Expression::RandomStatusAlly(_)
            | Expression::RandomStatusEnemy(_)
            | Expression::HasStatus(_)
            | Expression::AllStatusAllies(_)
            | Expression::AllStatusEnemies(_)
            | Expression::Vec2(_, _) => hex_color!("#18FFFF"),
            Expression::Vec2E(_)
            | Expression::StringInt(_)
            | Expression::StringFloat(_)
            | Expression::StringVec(_)
            | Expression::IntFloat(_)
            | Expression::ToInt(_)
            | Expression::Sin(_)
            | Expression::Cos(_)
            | Expression::Sign(_)
            | Expression::Fract(_)
            | Expression::Floor(_)
            | Expression::UnitVec(_)
            | Expression::Even(_)
            | Expression::Abs(_)
            | Expression::SlotUnit(_)
            | Expression::StatusEntity(..)
            | Expression::StateLast(..)
            | Expression::FactionCount(_)
            | Expression::FilterMaxEnemy(_)
            | Expression::FilterMaxAlly(_)
            | Expression::FindUnit(_)
            | Expression::Parent(_)
            | Expression::UnitCount(_)
            | Expression::RandomFloat(_)
            | Expression::RandomFloatUnit(_)
            | Expression::RandomEnemySubset(_)
            | Expression::StatusCharges(_)
            | Expression::FilterUnits(_) => hex_color!("#448AFF"),
            Expression::Vec2EE(_, _)
            | Expression::Sum(_, _)
            | Expression::Sub(_, _)
            | Expression::Mul(_, _)
            | Expression::Div(_, _)
            | Expression::GreaterThen(_, _)
            | Expression::LessThen(_, _)
            | Expression::Min(_, _)
            | Expression::Max(_, _)
            | Expression::Equals(_, _)
            | Expression::And(_, _)
            | Expression::Or(_, _)
            | Expression::WithVar(_, _, _) => hex_color!("#FFEB3B"),
            Expression::If(_, _, _) | Expression::Spread(_, _, _) => hex_color!("#BA68C8"),
        }
    }

    fn show_extra(&mut self, path: &str, context: &Context, world: &mut World, ui: &mut Ui) {
        let value = self.get_value(context, world);
        match self {
            Expression::Value(v) => {
                show_value(&Ok(v.clone()), ui);
            }
            Expression::Float(x) => {
                ui.add(DragValue::new(x).speed(0.1));
            }
            Expression::Int(x) => {
                ui.add(DragValue::new(x));
            }
            Expression::Bool(x) => {
                ui.checkbox(x, "");
            }
            Expression::String(x) => {
                ui.text_edit_singleline(x);
            }
            Expression::Hex(x) => {
                let c = Color::hex(&x).unwrap_or_default().as_rgba_u8();
                let mut c = Color32::from_rgba_premultiplied(c[0], c[1], c[2], c[3]);
                if ui.color_edit_button_srgba(&mut c).changed() {
                    *x = encode(c.to_array());
                }
            }
            Expression::Faction(x) => {
                ComboBox::from_id_source(&path)
                    .selected_text(x.to_string())
                    .show_ui(ui, |ui| {
                        for option in Faction::iter() {
                            let text = option.to_string();
                            ui.selectable_value(x, option, text);
                        }
                    });
            }
            Expression::OwnerState(x)
            | Expression::TargetState(x)
            | Expression::TargetStateLast(x)
            | Expression::Context(x)
            | Expression::OwnerStateLast(x) => {
                x.show_editor_with_context(context, path, world, ui);
            }
            Expression::WithVar(x, ..) => {
                ui.vertical(|ui| {
                    x.show_editor_with_context(context, path, world, ui);
                    show_value(&value, ui);
                });
            }
            Expression::Vec2(x, y) => {
                ui.add(DragValue::new(x).speed(0.1));
                ui.add(DragValue::new(y).speed(0.1));
            }
            Expression::HasStatus(name)
            | Expression::RandomStatusAlly(name)
            | Expression::RandomStatusEnemy(name)
            | Expression::AllStatusAllies(name)
            | Expression::AllStatusEnemies(name) => {
                Status::show_selector(name, path, ui, world);
            }
            _ => show_value(&value, ui),
        };
    }

    fn show_children(
        &mut self,
        path: &str,
        connect_pos: Option<Pos2>,
        context: &Context,
        ui: &mut Ui,
        world: &mut World,
    ) {
        match self {
            Expression::Zero
            | Expression::GameTime
            | Expression::PI
            | Expression::PI2
            | Expression::Age
            | Expression::SlotPosition
            | Expression::OwnerFaction
            | Expression::OppositeFaction
            | Expression::Beat
            | Expression::Owner
            | Expression::Caster
            | Expression::Target
            | Expression::Status
            | Expression::RandomUnit
            | Expression::RandomAdjacentUnit
            | Expression::RandomAlly
            | Expression::RandomEnemy
            | Expression::AllAllyUnits
            | Expression::AllEnemyUnits
            | Expression::AllUnits
            | Expression::AllOtherUnits
            | Expression::AdjacentUnits
            | Expression::Index
            | Expression::Float(_)
            | Expression::Int(_)
            | Expression::Bool(_)
            | Expression::String(_)
            | Expression::Hex(_)
            | Expression::Faction(_)
            | Expression::OwnerState(_)
            | Expression::OwnerStateLast(_)
            | Expression::TargetState(_)
            | Expression::TargetStateLast(_)
            | Expression::Context(_)
            | Expression::AbilityContext(_, _)
            | Expression::AbilityState(_, _)
            | Expression::Value(_)
            | Expression::RandomStatusAlly(_)
            | Expression::RandomStatusEnemy(_)
            | Expression::HasStatus(_)
            | Expression::AllStatusAllies(_)
            | Expression::AllStatusEnemies(_)
            | Expression::Vec2(_, _) => default(),
            Expression::Sin(x)
            | Expression::Cos(x)
            | Expression::Sign(x)
            | Expression::Fract(x)
            | Expression::Floor(x)
            | Expression::UnitVec(x)
            | Expression::Even(x)
            | Expression::Abs(x)
            | Expression::Vec2E(x)
            | Expression::StringInt(x)
            | Expression::StringFloat(x)
            | Expression::StringVec(x)
            | Expression::IntFloat(x)
            | Expression::ToInt(x)
            | Expression::SlotUnit(x)
            | Expression::StatusEntity(_, x)
            | Expression::StateLast(_, x)
            | Expression::FactionCount(x)
            | Expression::FilterMaxEnemy(x)
            | Expression::FilterMaxAlly(x)
            | Expression::FindUnit(x)
            | Expression::Parent(x)
            | Expression::UnitCount(x)
            | Expression::RandomFloat(x)
            | Expression::RandomFloatUnit(x)
            | Expression::RandomEnemySubset(x)
            | Expression::StatusCharges(x)
            | Expression::FilterUnits(x) => show_node(
                x.as_mut(),
                format!("{path}:x"),
                connect_pos,
                context,
                ui,
                world,
            ),

            Expression::Sum(a, b)
            | Expression::Sub(a, b)
            | Expression::Mul(a, b)
            | Expression::Div(a, b)
            | Expression::GreaterThen(a, b)
            | Expression::LessThen(a, b)
            | Expression::Min(a, b)
            | Expression::Max(a, b)
            | Expression::Equals(a, b)
            | Expression::And(a, b)
            | Expression::Vec2EE(a, b)
            | Expression::Or(a, b) => {
                ui.vertical(|ui| {
                    ui.horizontal(|ui| {
                        show_node(
                            a.as_mut(),
                            format!("{path}:a"),
                            connect_pos,
                            context,
                            ui,
                            world,
                        );
                    });
                    ui.horizontal(|ui| {
                        show_node(
                            b.as_mut(),
                            format!("{path}:b"),
                            connect_pos,
                            context,
                            ui,
                            world,
                        );
                    });
                });
            }
            Expression::If(a, b, c) | Expression::Spread(a, b, c) => {
                ui.vertical(|ui| {
                    ui.horizontal(|ui| {
                        show_node(
                            a.as_mut(),
                            format!("{path}:a"),
                            connect_pos,
                            context,
                            ui,
                            world,
                        );
                    });
                    ui.horizontal(|ui| {
                        show_node(
                            b.as_mut(),
                            format!("{path}:b"),
                            connect_pos,
                            context,
                            ui,
                            world,
                        );
                    });
                    ui.horizontal(|ui| {
                        show_node(
                            c.as_mut(),
                            format!("{path}:c"),
                            connect_pos,
                            context,
                            ui,
                            world,
                        );
                    });
                });
            }
            Expression::WithVar(_, val, e) => {
                ui.vertical(|ui| {
                    ui.horizontal(|ui| {
                        show_node(
                            val.as_mut(),
                            format!("{path}:val"),
                            connect_pos,
                            context,
                            ui,
                            world,
                        );
                    });
                    ui.horizontal(|ui| {
                        show_node(
                            e.as_mut(),
                            format!("{path}:e"),
                            connect_pos,
                            context,
                            ui,
                            world,
                        );
                    });
                });
            }
        };
    }

    fn show_replace_buttons(&mut self, lookup: &str, submit: bool, ui: &mut Ui) -> bool {
        for (e, _) in Expression::iter()
            .filter_map(|e| {
                let s = e.as_ref().to_lowercase();
                match s.contains(lookup) {
                    true => Some((e, s)),
                    false => None,
                }
            })
            .sorted_by_key(|(_, s)| !s.starts_with(lookup))
        {
            let btn = e.as_ref().add_color(e.node_color()).rich_text(ui);
            let btn = ui.button(btn);
            if btn.clicked() || submit {
                btn.request_focus();
            }
            if btn.gained_focus() {
                *self = e.set_inner(self.clone());
                return true;
            }
        }
        false
    }

    fn wrap(&mut self) {
        *self = Self::Mul(Box::new(self.clone()), Box::new(Self::Float(0.1)))
    }

    fn show_context_menu(&mut self, ui: &mut Ui) {
        if ui.button("Randomize 0.1").clicked() {
            self.mut_all(|e| {
                e.try_randomize(0.1);
            });
            ui.close_menu();
        }
        if ui.button("Randomize 1.0").clicked() {
            self.mut_all(|e| {
                e.try_randomize(1.0);
            });
            ui.close_menu();
        }
        if ui.button("Randomize 10.0").clicked() {
            self.mut_all(|e| {
                e.try_randomize(10.0);
            });
            ui.close_menu();
        }
    }
}

impl std::fmt::Display for Expression {
    fn fmt(&self, f: &mut __private::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expression::Zero
            | Expression::GameTime
            | Expression::PI
            | Expression::PI2
            | Expression::Age
            | Expression::SlotPosition
            | Expression::OwnerFaction
            | Expression::OppositeFaction
            | Expression::Beat
            | Expression::Owner
            | Expression::Caster
            | Expression::Target
            | Expression::Status
            | Expression::RandomUnit
            | Expression::RandomAdjacentUnit
            | Expression::RandomAlly
            | Expression::RandomEnemy
            | Expression::AllAllyUnits
            | Expression::AllEnemyUnits
            | Expression::AllUnits
            | Expression::AllOtherUnits
            | Expression::Index
            | Expression::AdjacentUnits => write!(f, "{}", self.as_ref().to_case(Case::Lower)),
            Expression::Float(v) => write!(f, "{v}"),
            Expression::Int(v) => write!(f, "{v}"),
            Expression::Bool(v) => write!(f, "{v}"),
            Expression::String(v) => write!(f, "{v}"),
            Expression::Hex(v) => write!(f, "{v}"),
            Expression::Faction(v) => write!(f, "{v}"),
            Expression::OwnerState(v) => write!(f, "{}({v})", self.as_ref()),
            Expression::OwnerStateLast(v) => write!(f, "{}({v})", self.as_ref()),
            Expression::TargetState(v) => write!(f, "{}({v})", self.as_ref()),
            Expression::TargetStateLast(v) => write!(f, "{}({v})", self.as_ref()),
            Expression::StateLast(v, t) => write!(f, "{} ({v}, {t})", self.as_ref()),
            Expression::Context(v) => write!(f, "{{{v}}}"),
            Expression::StatusEntity(s, v) => write!(f, "{} ({s}, {v})", self.as_ref()),
            Expression::AbilityContext(a, v) | Expression::AbilityState(a, v) => {
                write!(f, "{}({a}:{v})", self.as_ref())
            }
            Expression::Value(v) => write!(f, "{}({v})", self.as_ref()),
            Expression::Vec2(x, y) => write!(f, "({x}, {y})"),
            Expression::Vec2E(x) => write!(f, "({x}, {x})"),
            Expression::HasStatus(v) => write!(f, "{}({v})", self.as_ref()),
            Expression::StringInt(v)
            | Expression::StringFloat(v)
            | Expression::StringVec(v)
            | Expression::IntFloat(v)
            | Expression::ToInt(v)
            | Expression::Sin(v)
            | Expression::Cos(v)
            | Expression::Sign(v)
            | Expression::Fract(v)
            | Expression::Floor(v)
            | Expression::UnitVec(v)
            | Expression::Even(v)
            | Expression::Abs(v)
            | Expression::SlotUnit(v)
            | Expression::FactionCount(v)
            | Expression::FilterMaxEnemy(v)
            | Expression::FilterMaxAlly(v)
            | Expression::FindUnit(v)
            | Expression::Parent(v)
            | Expression::UnitCount(v)
            | Expression::RandomFloat(v)
            | Expression::RandomFloatUnit(v)
            | Expression::StatusCharges(v)
            | Expression::FilterUnits(v) => {
                write!(f, "{} ({v})", self.as_ref().to_case(Case::Title),)
            }
            Expression::Vec2EE(x, y)
            | Expression::Sub(x, y)
            | Expression::Mul(x, y)
            | Expression::Div(x, y)
            | Expression::GreaterThen(x, y)
            | Expression::LessThen(x, y)
            | Expression::Min(x, y)
            | Expression::Max(x, y)
            | Expression::Equals(x, y)
            | Expression::And(x, y)
            | Expression::Or(x, y) => {
                write!(f, "{}({x}, {y})", self.as_ref().to_case(Case::Title),)
            }
            Expression::Sum(x, y) => write!(f, "({x}+{y})"),
            Expression::If(x, y, z) | Expression::Spread(x, y, z) => {
                write!(f, "{}({x}, {y}, {z})", self.as_ref())
            }
            Expression::WithVar(var, val, e) => write!(
                f,
                "{}({var}, {val}, {e})",
                self.as_ref().to_case(Case::Title)
            ),
            Expression::RandomEnemySubset(amount) => {
                write!(f, "{{{amount}}} random enemies")
            }
            Expression::RandomStatusAlly(v) | Expression::RandomStatusEnemy(v) => write!(
                f,
                "random {} with [{v}]",
                if matches!(self, Self::RandomStatusAlly(..)) {
                    "ally"
                } else {
                    "enemy"
                }
            ),
            Expression::AllStatusAllies(v) | Expression::AllStatusEnemies(v) => write!(
                f,
                "all {} with [{v}]",
                if matches!(self, Self::AllStatusAllies(..)) {
                    "allies"
                } else {
                    "enemies"
                }
            ),
        }
    }
}
