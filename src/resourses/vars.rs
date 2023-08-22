use anyhow::anyhow;

use super::*;

#[derive(Hash, Eq, PartialEq, Serialize, Deserialize, Clone, Copy, Debug, Reflect)]
pub enum VarName {
    Size,
    Radius,
    Position,
    Hp,
    Atk,
    Dmg,
    Name,
    Text,
    Spawn,
    Slot,
}

#[derive(Serialize, Deserialize, Clone, Debug, Reflect)]
pub enum VarValue {
    Float(f32),
    Int(i32),
    Vec2(Vec2),
    Bool(bool),
    String(String),
}

impl VarValue {
    pub fn get_float(&self) -> Result<f32> {
        match self {
            VarValue::Float(value) => Ok(*value),
            VarValue::Int(value) => Ok(*value as f32),
            _ => Err(anyhow!("Float not supported by {self:?}")),
        }
    }
    pub fn get_int(&self) -> Result<i32> {
        match self {
            VarValue::Int(value) => Ok(*value),
            _ => Err(anyhow!("Int not supported by {self:?}")),
        }
    }
    pub fn get_vec2(&self) -> Result<Vec2> {
        match self {
            VarValue::Vec2(value) => Ok(*value),
            _ => Err(anyhow!("Vec2 not supported by {self:?}")),
        }
    }
    pub fn get_bool(&self) -> Result<bool> {
        match self {
            VarValue::Bool(value) => Ok(*value),
            _ => Err(anyhow!("Bool not supported by {self:?}")),
        }
    }
    pub fn get_string(&self) -> Result<String> {
        match self {
            VarValue::String(value) => Ok(value.into()),
            _ => Err(anyhow!("String not supported by {self:?}")),
        }
    }
}
