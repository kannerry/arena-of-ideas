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
pub struct BeatTowerArgs {
    pub tower_id: u64,
    pub levels: Vec<String>,
    pub owner_team: String,
}

impl Reducer for BeatTowerArgs {
    const REDUCER_NAME: &'static str = "beat_tower";
}

#[allow(unused)]
pub fn beat_tower(tower_id: u64, levels: Vec<String>, owner_team: String) {
    BeatTowerArgs {
        tower_id,
        levels,
        owner_team,
    }
    .invoke();
}

#[allow(unused)]
pub fn on_beat_tower(
    mut __callback: impl FnMut(&Identity, Option<Address>, &Status, &u64, &Vec<String>, &String)
        + Send
        + 'static,
) -> ReducerCallbackId<BeatTowerArgs> {
    BeatTowerArgs::on_reducer(move |__identity, __addr, __status, __args| {
        let BeatTowerArgs {
            tower_id,
            levels,
            owner_team,
        } = __args;
        __callback(__identity, __addr, __status, tower_id, levels, owner_team);
    })
}

#[allow(unused)]
pub fn once_on_beat_tower(
    __callback: impl FnOnce(&Identity, Option<Address>, &Status, &u64, &Vec<String>, &String)
        + Send
        + 'static,
) -> ReducerCallbackId<BeatTowerArgs> {
    BeatTowerArgs::once_on_reducer(move |__identity, __addr, __status, __args| {
        let BeatTowerArgs {
            tower_id,
            levels,
            owner_team,
        } = __args;
        __callback(__identity, __addr, __status, tower_id, levels, owner_team);
    })
}

#[allow(unused)]
pub fn remove_on_beat_tower(id: ReducerCallbackId<BeatTowerArgs>) {
    BeatTowerArgs::remove_on_reducer(id);
}