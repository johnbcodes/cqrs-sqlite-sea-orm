#![forbid(unsafe_code)]
#![deny(clippy::all)]
// #![warn(clippy::pedantic)]
mod bank_account;
mod config;
mod metadata_extractor;
mod route_handler;
mod services;
mod state;

use crate::route_handler::{
    account_handler, checks_handler, command_handler, ledger_entries_handler,
};
use crate::state::new_application_state;
use axum::routing::get;
use axum::Router;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    let state = new_application_state().await;

    // Configure the Axum routes and services.
    // For this example a single logical endpoint is used and the HTTP method
    // distinguishes whether the call is a command or a query.
    let router = Router::new()
        .route(
            "/account/:account_id",
            get(account_handler).post(command_handler),
        )
        .route("/account/:account_id/checks", get(checks_handler))
        .route(
            "/account/:account_id/ledger_entries",
            get(ledger_entries_handler),
        )
        .with_state(state);

    // Start the Axum server.
    let listener = TcpListener::bind("0.0.0.0:3030").await.unwrap();
    axum::serve(listener, router).await.unwrap();
}
