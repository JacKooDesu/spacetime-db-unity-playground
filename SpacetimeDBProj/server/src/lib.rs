mod types;
mod models;

use spacetimedb::{Identity, ReducerContext, Table};

#[spacetimedb::reducer(init)]
pub fn init(_ctx: &ReducerContext) {
}

#[spacetimedb::reducer(client_connected)]
pub fn identity_connected(_ctx: &ReducerContext) {
}

#[spacetimedb::reducer(client_disconnected)]
pub fn identity_disconnected(_ctx: &ReducerContext) {
}
