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
pub struct AddLadderLevelsArgs {
    pub ladder_id: u64,
    pub levels: Vec<String>,
}

impl Reducer for AddLadderLevelsArgs {
    const REDUCER_NAME: &'static str = "add_ladder_levels";
}

#[allow(unused)]
pub fn add_ladder_levels(ladder_id: u64, levels: Vec<String>) {
    AddLadderLevelsArgs { ladder_id, levels }.invoke();
}

#[allow(unused)]
pub fn on_add_ladder_levels(
    mut __callback: impl FnMut(&Identity, Option<Address>, &Status, &u64, &Vec<String>) + Send + 'static,
) -> ReducerCallbackId<AddLadderLevelsArgs> {
    AddLadderLevelsArgs::on_reducer(move |__identity, __addr, __status, __args| {
        let AddLadderLevelsArgs { ladder_id, levels } = __args;
        __callback(__identity, __addr, __status, ladder_id, levels);
    })
}

#[allow(unused)]
pub fn once_on_add_ladder_levels(
    __callback: impl FnOnce(&Identity, Option<Address>, &Status, &u64, &Vec<String>) + Send + 'static,
) -> ReducerCallbackId<AddLadderLevelsArgs> {
    AddLadderLevelsArgs::once_on_reducer(move |__identity, __addr, __status, __args| {
        let AddLadderLevelsArgs { ladder_id, levels } = __args;
        __callback(__identity, __addr, __status, ladder_id, levels);
    })
}

#[allow(unused)]
pub fn remove_on_add_ladder_levels(id: ReducerCallbackId<AddLadderLevelsArgs>) {
    AddLadderLevelsArgs::remove_on_reducer(id);
}
