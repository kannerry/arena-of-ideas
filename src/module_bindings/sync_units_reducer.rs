// THIS FILE IS AUTOMATICALLY GENERATED BY SPACETIMEDB. EDITS TO THIS FILE
// WILL NOT BE SAVED. MODIFY TABLES IN RUST INSTEAD.

use super::unit::Unit;
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
pub struct SyncUnitsArgs {
    pub units: Vec<Unit>,
}

impl Reducer for SyncUnitsArgs {
    const REDUCER_NAME: &'static str = "sync_units";
}

#[allow(unused)]
pub fn sync_units(units: Vec<Unit>) {
    SyncUnitsArgs { units }.invoke();
}

#[allow(unused)]
pub fn on_sync_units(
    mut __callback: impl FnMut(&Identity, Option<Address>, &Status, &Vec<Unit>) + Send + 'static,
) -> ReducerCallbackId<SyncUnitsArgs> {
    SyncUnitsArgs::on_reducer(move |__identity, __addr, __status, __args| {
        let SyncUnitsArgs { units } = __args;
        __callback(__identity, __addr, __status, units);
    })
}

#[allow(unused)]
pub fn once_on_sync_units(
    __callback: impl FnOnce(&Identity, Option<Address>, &Status, &Vec<Unit>) + Send + 'static,
) -> ReducerCallbackId<SyncUnitsArgs> {
    SyncUnitsArgs::once_on_reducer(move |__identity, __addr, __status, __args| {
        let SyncUnitsArgs { units } = __args;
        __callback(__identity, __addr, __status, units);
    })
}

#[allow(unused)]
pub fn remove_on_sync_units(id: ReducerCallbackId<SyncUnitsArgs>) {
    SyncUnitsArgs::remove_on_reducer(id);
}