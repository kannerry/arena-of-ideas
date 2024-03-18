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
pub mod game_state;
pub mod give_right_reducer;
pub mod global_data;
pub mod house;
pub mod login_by_identity_reducer;
pub mod login_reducer;
pub mod logout_reducer;
pub mod register_empty_reducer;
pub mod register_reducer;
pub mod run_buy_reducer;
pub mod run_change_g_reducer;
pub mod run_fuse_reducer;
pub mod run_reroll_reducer;
pub mod run_sell_reducer;
pub mod run_stack_reducer;
pub mod run_start_reducer;
pub mod run_submit_result_reducer;
pub mod run_team_reorder_reducer;
pub mod set_name_reducer;
pub mod set_password_reducer;
pub mod shop_offer;
pub mod status_charges;
pub mod statuses;
pub mod summon;
pub mod sync_data_reducer;
pub mod table_unit;
pub mod team_unit;
pub mod user;
pub mod user_access;
pub mod user_right;
pub mod vfx;

pub use ability::*;
pub use arena_pool::*;
pub use arena_run::*;
pub use game_state::*;
pub use give_right_reducer::*;
pub use global_data::*;
pub use house::*;
pub use login_by_identity_reducer::*;
pub use login_reducer::*;
pub use logout_reducer::*;
pub use register_empty_reducer::*;
pub use register_reducer::*;
pub use run_buy_reducer::*;
pub use run_change_g_reducer::*;
pub use run_fuse_reducer::*;
pub use run_reroll_reducer::*;
pub use run_sell_reducer::*;
pub use run_stack_reducer::*;
pub use run_start_reducer::*;
pub use run_submit_result_reducer::*;
pub use run_team_reorder_reducer::*;
pub use set_name_reducer::*;
pub use set_password_reducer::*;
pub use shop_offer::*;
pub use status_charges::*;
pub use statuses::*;
pub use summon::*;
pub use sync_data_reducer::*;
pub use table_unit::*;
pub use team_unit::*;
pub use user::*;
pub use user_access::*;
pub use user_right::*;
pub use vfx::*;

#[allow(unused)]
#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub enum ReducerEvent {
    GiveRight(give_right_reducer::GiveRightArgs),
    Login(login_reducer::LoginArgs),
    LoginByIdentity(login_by_identity_reducer::LoginByIdentityArgs),
    Logout(logout_reducer::LogoutArgs),
    Register(register_reducer::RegisterArgs),
    RegisterEmpty(register_empty_reducer::RegisterEmptyArgs),
    RunBuy(run_buy_reducer::RunBuyArgs),
    RunChangeG(run_change_g_reducer::RunChangeGArgs),
    RunFuse(run_fuse_reducer::RunFuseArgs),
    RunReroll(run_reroll_reducer::RunRerollArgs),
    RunSell(run_sell_reducer::RunSellArgs),
    RunStack(run_stack_reducer::RunStackArgs),
    RunStart(run_start_reducer::RunStartArgs),
    RunSubmitResult(run_submit_result_reducer::RunSubmitResultArgs),
    RunTeamReorder(run_team_reorder_reducer::RunTeamReorderArgs),
    SetName(set_name_reducer::SetNameArgs),
    SetPassword(set_password_reducer::SetPasswordArgs),
    SyncData(sync_data_reducer::SyncDataArgs),
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
            "House" => client_cache
                .handle_table_update_with_primary_key::<house::House>(callbacks, table_update),
            "Statuses" => client_cache.handle_table_update_with_primary_key::<statuses::Statuses>(
                callbacks,
                table_update,
            ),
            "Summon" => client_cache
                .handle_table_update_with_primary_key::<summon::Summon>(callbacks, table_update),
            "TableUnit" => client_cache
                .handle_table_update_with_primary_key::<table_unit::TableUnit>(
                    callbacks,
                    table_update,
                ),
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
        reminders.invoke_callbacks::<house::House>(worker, &reducer_event, state);
        reminders.invoke_callbacks::<statuses::Statuses>(worker, &reducer_event, state);
        reminders.invoke_callbacks::<summon::Summon>(worker, &reducer_event, state);
        reminders.invoke_callbacks::<table_unit::TableUnit>(worker, &reducer_event, state);
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
						"give_right" => _reducer_callbacks.handle_event_of_type::<give_right_reducer::GiveRightArgs, ReducerEvent>(event, _state, ReducerEvent::GiveRight),
			"login" => _reducer_callbacks.handle_event_of_type::<login_reducer::LoginArgs, ReducerEvent>(event, _state, ReducerEvent::Login),
			"login_by_identity" => _reducer_callbacks.handle_event_of_type::<login_by_identity_reducer::LoginByIdentityArgs, ReducerEvent>(event, _state, ReducerEvent::LoginByIdentity),
			"logout" => _reducer_callbacks.handle_event_of_type::<logout_reducer::LogoutArgs, ReducerEvent>(event, _state, ReducerEvent::Logout),
			"register" => _reducer_callbacks.handle_event_of_type::<register_reducer::RegisterArgs, ReducerEvent>(event, _state, ReducerEvent::Register),
			"register_empty" => _reducer_callbacks.handle_event_of_type::<register_empty_reducer::RegisterEmptyArgs, ReducerEvent>(event, _state, ReducerEvent::RegisterEmpty),
			"run_buy" => _reducer_callbacks.handle_event_of_type::<run_buy_reducer::RunBuyArgs, ReducerEvent>(event, _state, ReducerEvent::RunBuy),
			"run_change_g" => _reducer_callbacks.handle_event_of_type::<run_change_g_reducer::RunChangeGArgs, ReducerEvent>(event, _state, ReducerEvent::RunChangeG),
			"run_fuse" => _reducer_callbacks.handle_event_of_type::<run_fuse_reducer::RunFuseArgs, ReducerEvent>(event, _state, ReducerEvent::RunFuse),
			"run_reroll" => _reducer_callbacks.handle_event_of_type::<run_reroll_reducer::RunRerollArgs, ReducerEvent>(event, _state, ReducerEvent::RunReroll),
			"run_sell" => _reducer_callbacks.handle_event_of_type::<run_sell_reducer::RunSellArgs, ReducerEvent>(event, _state, ReducerEvent::RunSell),
			"run_stack" => _reducer_callbacks.handle_event_of_type::<run_stack_reducer::RunStackArgs, ReducerEvent>(event, _state, ReducerEvent::RunStack),
			"run_start" => _reducer_callbacks.handle_event_of_type::<run_start_reducer::RunStartArgs, ReducerEvent>(event, _state, ReducerEvent::RunStart),
			"run_submit_result" => _reducer_callbacks.handle_event_of_type::<run_submit_result_reducer::RunSubmitResultArgs, ReducerEvent>(event, _state, ReducerEvent::RunSubmitResult),
			"run_team_reorder" => _reducer_callbacks.handle_event_of_type::<run_team_reorder_reducer::RunTeamReorderArgs, ReducerEvent>(event, _state, ReducerEvent::RunTeamReorder),
			"set_name" => _reducer_callbacks.handle_event_of_type::<set_name_reducer::SetNameArgs, ReducerEvent>(event, _state, ReducerEvent::SetName),
			"set_password" => _reducer_callbacks.handle_event_of_type::<set_password_reducer::SetPasswordArgs, ReducerEvent>(event, _state, ReducerEvent::SetPassword),
			"sync_data" => _reducer_callbacks.handle_event_of_type::<sync_data_reducer::SyncDataArgs, ReducerEvent>(event, _state, ReducerEvent::SyncData),
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
            "House" => {
                client_cache.handle_resubscribe_for_type::<house::House>(callbacks, new_subs)
            }
            "Statuses" => {
                client_cache.handle_resubscribe_for_type::<statuses::Statuses>(callbacks, new_subs)
            }
            "Summon" => {
                client_cache.handle_resubscribe_for_type::<summon::Summon>(callbacks, new_subs)
            }
            "TableUnit" => client_cache
                .handle_resubscribe_for_type::<table_unit::TableUnit>(callbacks, new_subs),
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
