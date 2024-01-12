// THIS FILE IS AUTOMATICALLY GENERATED BY SPACETIMEDB. EDITS TO THIS FILE
// WILL NOT BE SAVED. MODIFY TABLES IN RUST INSTEAD.

#[allow(unused)]
use spacetimedb_sdk::{
    anyhow::{anyhow, Result},
    identity::Identity,
    reducer::{Reducer, ReducerCallbackId, Status},
    sats::{de::Deserialize, ser::Serialize},
    spacetimedb_lib,
    table::{TableIter, TableType, TableWithPrimaryKey},
    Address,
};

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct ArenaPool {
    pub id: u64,
    pub owner: u64,
    pub round: u8,
    pub team: String,
}

impl TableType for ArenaPool {
    const TABLE_NAME: &'static str = "ArenaPool";
    type ReducerEvent = super::ReducerEvent;
}

impl TableWithPrimaryKey for ArenaPool {
    type PrimaryKey = u64;
    fn primary_key(&self) -> &Self::PrimaryKey {
        &self.id
    }
}

impl ArenaPool {
    #[allow(unused)]
    pub fn filter_by_id(id: u64) -> Option<Self> {
        Self::find(|row| row.id == id)
    }
    #[allow(unused)]
    pub fn filter_by_owner(owner: u64) -> TableIter<Self> {
        Self::filter(|row| row.owner == owner)
    }
    #[allow(unused)]
    pub fn filter_by_round(round: u8) -> TableIter<Self> {
        Self::filter(|row| row.round == round)
    }
    #[allow(unused)]
    pub fn filter_by_team(team: String) -> TableIter<Self> {
        Self::filter(|row| row.team == team)
    }
}