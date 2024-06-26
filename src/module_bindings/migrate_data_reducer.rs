// THIS FILE IS AUTOMATICALLY GENERATED BY SPACETIMEDB. EDITS TO THIS FILE
// WILL NOT BE SAVED. MODIFY TABLES IN RUST INSTEAD.

use super::arena_archive::ArenaArchive;
use super::arena_pool::ArenaPool;
use super::user::User;
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
pub struct MigrateDataArgs {
    pub arena_archive: Vec<ArenaArchive>,
    pub arena_pool: Vec<ArenaPool>,
    pub users: Vec<User>,
}

impl Reducer for MigrateDataArgs {
    const REDUCER_NAME: &'static str = "migrate_data";
}

#[allow(unused)]
pub fn migrate_data(
    arena_archive: Vec<ArenaArchive>,
    arena_pool: Vec<ArenaPool>,
    users: Vec<User>,
) {
    MigrateDataArgs {
        arena_archive,
        arena_pool,
        users,
    }
    .invoke();
}

#[allow(unused)]
pub fn on_migrate_data(
    mut __callback: impl FnMut(&Identity, Option<Address>, &Status, &Vec<ArenaArchive>, &Vec<ArenaPool>, &Vec<User>)
        + Send
        + 'static,
) -> ReducerCallbackId<MigrateDataArgs> {
    MigrateDataArgs::on_reducer(move |__identity, __addr, __status, __args| {
        let MigrateDataArgs {
            arena_archive,
            arena_pool,
            users,
        } = __args;
        __callback(
            __identity,
            __addr,
            __status,
            arena_archive,
            arena_pool,
            users,
        );
    })
}

#[allow(unused)]
pub fn once_on_migrate_data(
    __callback: impl FnOnce(&Identity, Option<Address>, &Status, &Vec<ArenaArchive>, &Vec<ArenaPool>, &Vec<User>)
        + Send
        + 'static,
) -> ReducerCallbackId<MigrateDataArgs> {
    MigrateDataArgs::once_on_reducer(move |__identity, __addr, __status, __args| {
        let MigrateDataArgs {
            arena_archive,
            arena_pool,
            users,
        } = __args;
        __callback(
            __identity,
            __addr,
            __status,
            arena_archive,
            arena_pool,
            users,
        );
    })
}

#[allow(unused)]
pub fn remove_on_migrate_data(id: ReducerCallbackId<MigrateDataArgs>) {
    MigrateDataArgs::remove_on_reducer(id);
}
