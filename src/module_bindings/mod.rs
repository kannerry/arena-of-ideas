// THIS FILE IS AUTOMATICALLY GENERATED BY SPACETIMEDB. EDITS TO THIS FILE
// WILL NOT BE SAVED. MODIFY TABLES IN RUST INSTEAD.

use spacetimedb_sdk::callbacks::{DbCallbacks, ReducerCallbacks};
use spacetimedb_sdk::client_api_messages::{Event, TableUpdate};
use spacetimedb_sdk::client_cache::{ClientCache, RowCallbackReminders};
use spacetimedb_sdk::global_connection::with_connection_mut;
use spacetimedb_sdk::identity::Credentials;
use spacetimedb_sdk::reducer::AnyReducerEvent;
use spacetimedb_sdk::spacetime_module::SpacetimeModule;
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
use std::sync::Arc;

pub mod ability;
pub mod arena_pool;
pub mod arena_run;
pub mod extend_global_tower_reducer;
pub mod give_right_reducer;
pub mod global_data;
pub mod global_tower;
pub mod house;
pub mod login_by_identity_reducer;
pub mod login_reducer;
pub mod logout_reducer;
pub mod register_empty_reducer;
pub mod register_reducer;
pub mod set_name_reducer;
pub mod set_password_reducer;
pub mod start_run_reducer;
pub mod statuses;
pub mod submit_run_result_reducer;
pub mod sync_abilities_reducer;
pub mod sync_houses_reducer;
pub mod sync_statuses_reducer;
pub mod sync_units_reducer;
pub mod sync_vfxs_reducer;
pub mod tower_floor;
pub mod unit;
pub mod unit_pool;
pub mod user;
pub mod user_access;
pub mod user_right;
pub mod vfx;

pub use ability::*;
pub use arena_pool::*;
pub use arena_run::*;
pub use extend_global_tower_reducer::*;
pub use give_right_reducer::*;
pub use global_data::*;
pub use global_tower::*;
pub use house::*;
pub use login_by_identity_reducer::*;
pub use login_reducer::*;
pub use logout_reducer::*;
pub use register_empty_reducer::*;
pub use register_reducer::*;
pub use set_name_reducer::*;
pub use set_password_reducer::*;
pub use start_run_reducer::*;
pub use statuses::*;
pub use submit_run_result_reducer::*;
pub use sync_abilities_reducer::*;
pub use sync_houses_reducer::*;
pub use sync_statuses_reducer::*;
pub use sync_units_reducer::*;
pub use sync_vfxs_reducer::*;
pub use tower_floor::*;
pub use unit::*;
pub use unit_pool::*;
pub use user::*;
pub use user_access::*;
pub use user_right::*;
pub use vfx::*;

#[allow(unused)]
#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub enum ReducerEvent {
    ExtendGlobalTower(extend_global_tower_reducer::ExtendGlobalTowerArgs),
    GiveRight(give_right_reducer::GiveRightArgs),
    Login(login_reducer::LoginArgs),
    LoginByIdentity(login_by_identity_reducer::LoginByIdentityArgs),
    Logout(logout_reducer::LogoutArgs),
    Register(register_reducer::RegisterArgs),
    RegisterEmpty(register_empty_reducer::RegisterEmptyArgs),
    SetName(set_name_reducer::SetNameArgs),
    SetPassword(set_password_reducer::SetPasswordArgs),
    StartRun(start_run_reducer::StartRunArgs),
    SubmitRunResult(submit_run_result_reducer::SubmitRunResultArgs),
    SyncAbilities(sync_abilities_reducer::SyncAbilitiesArgs),
    SyncHouses(sync_houses_reducer::SyncHousesArgs),
    SyncStatuses(sync_statuses_reducer::SyncStatusesArgs),
    SyncUnits(sync_units_reducer::SyncUnitsArgs),
    SyncVfxs(sync_vfxs_reducer::SyncVfxsArgs),
}

#[allow(unused)]
pub struct Module;
impl SpacetimeModule for Module {
    fn handle_table_update(
        &self,
        table_update: TableUpdate,
        client_cache: &mut ClientCache,
        callbacks: &mut RowCallbackReminders,
    ) {
        let table_name = &table_update.table_name[..];
        match table_name {
            "Ability" => client_cache
                .handle_table_update_with_primary_key::<ability::Ability>(callbacks, table_update),
            "ArenaPool" => client_cache
                .handle_table_update_with_primary_key::<arena_pool::ArenaPool>(
                    callbacks,
                    table_update,
                ),
            "ArenaRun" => client_cache.handle_table_update_with_primary_key::<arena_run::ArenaRun>(
                callbacks,
                table_update,
            ),
            "GlobalData" => client_cache
                .handle_table_update_no_primary_key::<global_data::GlobalData>(
                    callbacks,
                    table_update,
                ),
            "GlobalTower" => client_cache
                .handle_table_update_with_primary_key::<global_tower::GlobalTower>(
                    callbacks,
                    table_update,
                ),
            "House" => client_cache
                .handle_table_update_with_primary_key::<house::House>(callbacks, table_update),
            "Statuses" => client_cache.handle_table_update_with_primary_key::<statuses::Statuses>(
                callbacks,
                table_update,
            ),
            "Unit" => client_cache
                .handle_table_update_with_primary_key::<unit::Unit>(callbacks, table_update),
            "User" => client_cache
                .handle_table_update_with_primary_key::<user::User>(callbacks, table_update),
            "UserAccess" => client_cache
                .handle_table_update_with_primary_key::<user_access::UserAccess>(
                    callbacks,
                    table_update,
                ),
            "Vfx" => client_cache
                .handle_table_update_with_primary_key::<vfx::Vfx>(callbacks, table_update),
            _ => {
                spacetimedb_sdk::log::error!("TableRowOperation on unknown table {:?}", table_name)
            }
        }
    }
    fn invoke_row_callbacks(
        &self,
        reminders: &mut RowCallbackReminders,
        worker: &mut DbCallbacks,
        reducer_event: Option<Arc<AnyReducerEvent>>,
        state: &Arc<ClientCache>,
    ) {
        reminders.invoke_callbacks::<ability::Ability>(worker, &reducer_event, state);
        reminders.invoke_callbacks::<arena_pool::ArenaPool>(worker, &reducer_event, state);
        reminders.invoke_callbacks::<arena_run::ArenaRun>(worker, &reducer_event, state);
        reminders.invoke_callbacks::<global_data::GlobalData>(worker, &reducer_event, state);
        reminders.invoke_callbacks::<global_tower::GlobalTower>(worker, &reducer_event, state);
        reminders.invoke_callbacks::<house::House>(worker, &reducer_event, state);
        reminders.invoke_callbacks::<statuses::Statuses>(worker, &reducer_event, state);
        reminders.invoke_callbacks::<unit::Unit>(worker, &reducer_event, state);
        reminders.invoke_callbacks::<user::User>(worker, &reducer_event, state);
        reminders.invoke_callbacks::<user_access::UserAccess>(worker, &reducer_event, state);
        reminders.invoke_callbacks::<vfx::Vfx>(worker, &reducer_event, state);
    }
    fn handle_event(
        &self,
        event: Event,
        _reducer_callbacks: &mut ReducerCallbacks,
        _state: Arc<ClientCache>,
    ) -> Option<Arc<AnyReducerEvent>> {
        let Some(function_call) = &event.function_call else {
            spacetimedb_sdk::log::warn!("Received Event with None function_call");
            return None;
        };
        #[allow(clippy::match_single_binding)]
match &function_call.reducer[..] {
						"extend_global_tower" => _reducer_callbacks.handle_event_of_type::<extend_global_tower_reducer::ExtendGlobalTowerArgs, ReducerEvent>(event, _state, ReducerEvent::ExtendGlobalTower),
			"give_right" => _reducer_callbacks.handle_event_of_type::<give_right_reducer::GiveRightArgs, ReducerEvent>(event, _state, ReducerEvent::GiveRight),
			"login" => _reducer_callbacks.handle_event_of_type::<login_reducer::LoginArgs, ReducerEvent>(event, _state, ReducerEvent::Login),
			"login_by_identity" => _reducer_callbacks.handle_event_of_type::<login_by_identity_reducer::LoginByIdentityArgs, ReducerEvent>(event, _state, ReducerEvent::LoginByIdentity),
			"logout" => _reducer_callbacks.handle_event_of_type::<logout_reducer::LogoutArgs, ReducerEvent>(event, _state, ReducerEvent::Logout),
			"register" => _reducer_callbacks.handle_event_of_type::<register_reducer::RegisterArgs, ReducerEvent>(event, _state, ReducerEvent::Register),
			"register_empty" => _reducer_callbacks.handle_event_of_type::<register_empty_reducer::RegisterEmptyArgs, ReducerEvent>(event, _state, ReducerEvent::RegisterEmpty),
			"set_name" => _reducer_callbacks.handle_event_of_type::<set_name_reducer::SetNameArgs, ReducerEvent>(event, _state, ReducerEvent::SetName),
			"set_password" => _reducer_callbacks.handle_event_of_type::<set_password_reducer::SetPasswordArgs, ReducerEvent>(event, _state, ReducerEvent::SetPassword),
			"start_run" => _reducer_callbacks.handle_event_of_type::<start_run_reducer::StartRunArgs, ReducerEvent>(event, _state, ReducerEvent::StartRun),
			"submit_run_result" => _reducer_callbacks.handle_event_of_type::<submit_run_result_reducer::SubmitRunResultArgs, ReducerEvent>(event, _state, ReducerEvent::SubmitRunResult),
			"sync_abilities" => _reducer_callbacks.handle_event_of_type::<sync_abilities_reducer::SyncAbilitiesArgs, ReducerEvent>(event, _state, ReducerEvent::SyncAbilities),
			"sync_houses" => _reducer_callbacks.handle_event_of_type::<sync_houses_reducer::SyncHousesArgs, ReducerEvent>(event, _state, ReducerEvent::SyncHouses),
			"sync_statuses" => _reducer_callbacks.handle_event_of_type::<sync_statuses_reducer::SyncStatusesArgs, ReducerEvent>(event, _state, ReducerEvent::SyncStatuses),
			"sync_units" => _reducer_callbacks.handle_event_of_type::<sync_units_reducer::SyncUnitsArgs, ReducerEvent>(event, _state, ReducerEvent::SyncUnits),
			"sync_vfxs" => _reducer_callbacks.handle_event_of_type::<sync_vfxs_reducer::SyncVfxsArgs, ReducerEvent>(event, _state, ReducerEvent::SyncVfxs),
			unknown => { spacetimedb_sdk::log::error!("Event on an unknown reducer: {:?}", unknown); None }
}
    }
    fn handle_resubscribe(
        &self,
        new_subs: TableUpdate,
        client_cache: &mut ClientCache,
        callbacks: &mut RowCallbackReminders,
    ) {
        let table_name = &new_subs.table_name[..];
        match table_name {
            "Ability" => {
                client_cache.handle_resubscribe_for_type::<ability::Ability>(callbacks, new_subs)
            }
            "ArenaPool" => client_cache
                .handle_resubscribe_for_type::<arena_pool::ArenaPool>(callbacks, new_subs),
            "ArenaRun" => {
                client_cache.handle_resubscribe_for_type::<arena_run::ArenaRun>(callbacks, new_subs)
            }
            "GlobalData" => client_cache
                .handle_resubscribe_for_type::<global_data::GlobalData>(callbacks, new_subs),
            "GlobalTower" => client_cache
                .handle_resubscribe_for_type::<global_tower::GlobalTower>(callbacks, new_subs),
            "House" => {
                client_cache.handle_resubscribe_for_type::<house::House>(callbacks, new_subs)
            }
            "Statuses" => {
                client_cache.handle_resubscribe_for_type::<statuses::Statuses>(callbacks, new_subs)
            }
            "Unit" => client_cache.handle_resubscribe_for_type::<unit::Unit>(callbacks, new_subs),
            "User" => client_cache.handle_resubscribe_for_type::<user::User>(callbacks, new_subs),
            "UserAccess" => client_cache
                .handle_resubscribe_for_type::<user_access::UserAccess>(callbacks, new_subs),
            "Vfx" => client_cache.handle_resubscribe_for_type::<vfx::Vfx>(callbacks, new_subs),
            _ => {
                spacetimedb_sdk::log::error!("TableRowOperation on unknown table {:?}", table_name)
            }
        }
    }
}

/// Connect to a database named `db_name` accessible over the internet at the URI `spacetimedb_uri`.
///
/// If `credentials` are supplied, they will be passed to the new connection to
/// identify and authenticate the user. Otherwise, a set of `Credentials` will be
/// generated by the server.
pub fn connect<IntoUri>(
    spacetimedb_uri: IntoUri,
    db_name: &str,
    credentials: Option<Credentials>,
) -> Result<()>
where
    IntoUri: TryInto<spacetimedb_sdk::http::Uri>,
    <IntoUri as TryInto<spacetimedb_sdk::http::Uri>>::Error:
        std::error::Error + Send + Sync + 'static,
{
    with_connection_mut(|connection| {
        connection.connect(spacetimedb_uri, db_name, credentials, Arc::new(Module))?;
        Ok(())
    })
}
