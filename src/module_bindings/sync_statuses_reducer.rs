// THIS FILE IS AUTOMATICALLY GENERATED BY SPACETIMEDB. EDITS TO THIS FILE
// WILL NOT BE SAVED. MODIFY TABLES IN RUST INSTEAD.

use super::statuses::Statuses;
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
pub struct SyncStatusesArgs {
    pub statuses: Vec<Statuses>,
}

impl Reducer for SyncStatusesArgs {
    const REDUCER_NAME: &'static str = "sync_statuses";
}

#[allow(unused)]
pub fn sync_statuses(statuses: Vec<Statuses>) {
    SyncStatusesArgs { statuses }.invoke();
}

#[allow(unused)]
pub fn on_sync_statuses(
    mut __callback: impl FnMut(&Identity, Option<Address>, &Status, &Vec<Statuses>) + Send + 'static,
) -> ReducerCallbackId<SyncStatusesArgs> {
    SyncStatusesArgs::on_reducer(move |__identity, __addr, __status, __args| {
        let SyncStatusesArgs { statuses } = __args;
        __callback(__identity, __addr, __status, statuses);
    })
}

#[allow(unused)]
pub fn once_on_sync_statuses(
    __callback: impl FnOnce(&Identity, Option<Address>, &Status, &Vec<Statuses>) + Send + 'static,
) -> ReducerCallbackId<SyncStatusesArgs> {
    SyncStatusesArgs::once_on_reducer(move |__identity, __addr, __status, __args| {
        let SyncStatusesArgs { statuses } = __args;
        __callback(__identity, __addr, __status, statuses);
    })
}

#[allow(unused)]
pub fn remove_on_sync_statuses(id: ReducerCallbackId<SyncStatusesArgs>) {
    SyncStatusesArgs::remove_on_reducer(id);
}