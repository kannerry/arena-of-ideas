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
pub struct RunStackArgs {
    pub target: u64,
    pub source: u64,
}

impl Reducer for RunStackArgs {
    const REDUCER_NAME: &'static str = "run_stack";
}

#[allow(unused)]
pub fn run_stack(target: u64, source: u64) {
    RunStackArgs { target, source }.invoke();
}

#[allow(unused)]
pub fn on_run_stack(
    mut __callback: impl FnMut(&Identity, Option<Address>, &Status, &u64, &u64) + Send + 'static,
) -> ReducerCallbackId<RunStackArgs> {
    RunStackArgs::on_reducer(move |__identity, __addr, __status, __args| {
        let RunStackArgs { target, source } = __args;
        __callback(__identity, __addr, __status, target, source);
    })
}

#[allow(unused)]
pub fn once_on_run_stack(
    __callback: impl FnOnce(&Identity, Option<Address>, &Status, &u64, &u64) + Send + 'static,
) -> ReducerCallbackId<RunStackArgs> {
    RunStackArgs::once_on_reducer(move |__identity, __addr, __status, __args| {
        let RunStackArgs { target, source } = __args;
        __callback(__identity, __addr, __status, target, source);
    })
}

#[allow(unused)]
pub fn remove_on_run_stack(id: ReducerCallbackId<RunStackArgs>) {
    RunStackArgs::remove_on_reducer(id);
}
