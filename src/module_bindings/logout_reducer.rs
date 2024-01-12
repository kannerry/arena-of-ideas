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
pub struct LogoutArgs {}

impl Reducer for LogoutArgs {
    const REDUCER_NAME: &'static str = "logout";
}

#[allow(unused)]
pub fn logout() {
    LogoutArgs {}.invoke();
}

#[allow(unused)]
pub fn on_logout(
    mut __callback: impl FnMut(&Identity, Option<Address>, &Status) + Send + 'static,
) -> ReducerCallbackId<LogoutArgs> {
    LogoutArgs::on_reducer(move |__identity, __addr, __status, __args| {
        let LogoutArgs {} = __args;
        __callback(__identity, __addr, __status);
    })
}

#[allow(unused)]
pub fn once_on_logout(
    __callback: impl FnOnce(&Identity, Option<Address>, &Status) + Send + 'static,
) -> ReducerCallbackId<LogoutArgs> {
    LogoutArgs::once_on_reducer(move |__identity, __addr, __status, __args| {
        let LogoutArgs {} = __args;
        __callback(__identity, __addr, __status);
    })
}

#[allow(unused)]
pub fn remove_on_logout(id: ReducerCallbackId<LogoutArgs>) {
    LogoutArgs::remove_on_reducer(id);
}
