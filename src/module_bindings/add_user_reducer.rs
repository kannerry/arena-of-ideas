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
pub struct AddUserArgs {}

impl Reducer for AddUserArgs {
    const REDUCER_NAME: &'static str = "add_user";
}

#[allow(unused)]
pub fn add_user() {
    AddUserArgs {}.invoke();
}

#[allow(unused)]
pub fn on_add_user(
    mut __callback: impl FnMut(&Identity, Option<Address>, &Status) + Send + 'static,
) -> ReducerCallbackId<AddUserArgs> {
    AddUserArgs::on_reducer(move |__identity, __addr, __status, __args| {
        let AddUserArgs {} = __args;
        __callback(__identity, __addr, __status);
    })
}

#[allow(unused)]
pub fn once_on_add_user(
    __callback: impl FnOnce(&Identity, Option<Address>, &Status) + Send + 'static,
) -> ReducerCallbackId<AddUserArgs> {
    AddUserArgs::once_on_reducer(move |__identity, __addr, __status, __args| {
        let AddUserArgs {} = __args;
        __callback(__identity, __addr, __status);
    })
}

#[allow(unused)]
pub fn remove_on_add_user(id: ReducerCallbackId<AddUserArgs>) {
    AddUserArgs::remove_on_reducer(id);
}