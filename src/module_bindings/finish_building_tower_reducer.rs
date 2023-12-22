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
pub struct FinishBuildingTowerArgs {
    pub levels: Vec<String>,
    pub owner_team: String,
}

impl Reducer for FinishBuildingTowerArgs {
    const REDUCER_NAME: &'static str = "finish_building_tower";
}

#[allow(unused)]
pub fn finish_building_tower(levels: Vec<String>, owner_team: String) {
    FinishBuildingTowerArgs { levels, owner_team }.invoke();
}

#[allow(unused)]
pub fn on_finish_building_tower(
    mut __callback: impl FnMut(&Identity, Option<Address>, &Status, &Vec<String>, &String)
        + Send
        + 'static,
) -> ReducerCallbackId<FinishBuildingTowerArgs> {
    FinishBuildingTowerArgs::on_reducer(move |__identity, __addr, __status, __args| {
        let FinishBuildingTowerArgs { levels, owner_team } = __args;
        __callback(__identity, __addr, __status, levels, owner_team);
    })
}

#[allow(unused)]
pub fn once_on_finish_building_tower(
    __callback: impl FnOnce(&Identity, Option<Address>, &Status, &Vec<String>, &String) + Send + 'static,
) -> ReducerCallbackId<FinishBuildingTowerArgs> {
    FinishBuildingTowerArgs::once_on_reducer(move |__identity, __addr, __status, __args| {
        let FinishBuildingTowerArgs { levels, owner_team } = __args;
        __callback(__identity, __addr, __status, levels, owner_team);
    })
}

#[allow(unused)]
pub fn remove_on_finish_building_tower(id: ReducerCallbackId<FinishBuildingTowerArgs>) {
    FinishBuildingTowerArgs::remove_on_reducer(id);
}