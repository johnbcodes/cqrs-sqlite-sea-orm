#![forbid(unsafe_code)]
#![deny(clippy::all)]
// #![warn(clippy::pedantic)]

use std::sync::Arc;

use axum::extract::{Extension, Path};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::routing::get;
use axum::{Json, Router};
use sea_orm::{
    ColumnTrait, ConnectOptions, Database, DatabaseConnection, DbErr, EntityTrait, QueryFilter,
    QueryOrder,
};
use sqlite_es::{default_sqlite_pool, SqliteCqrs};
use std::time::Duration;
use tracing::log;

use crate::bank_account::aggregate::BankAccount;
use crate::bank_account::commands::BankAccountCommand;
use crate::bank_account::models::{checks, ledger_entries, prelude::*};
use crate::config::cqrs_framework;
use crate::metadata_extension::MetadataExtension;

mod bank_account;
mod config;
mod metadata_extension;
mod services;

#[tokio::main]
async fn main() {
    let uri = "sqlite://demo.db";
    let pool = default_sqlite_pool(&uri).await;
    sqlx::migrate!().run(&pool).await.unwrap();

    let mut opt = ConnectOptions::new(uri.to_string());
    opt.max_connections(10)
        .min_connections(5)
        .connect_timeout(Duration::from_secs(8))
        .acquire_timeout(Duration::from_secs(8))
        .idle_timeout(Duration::from_secs(8))
        .max_lifetime(Duration::from_secs(8))
        .sqlx_logging(true)
        .sqlx_logging_level(log::LevelFilter::Info);
    let db = Database::connect(opt).await.unwrap();

    let cqrs = cqrs_framework(pool, db.clone());

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
        .layer(Extension(cqrs))
        .layer(Extension(db));

    // Start the Axum server.
    axum::Server::bind(&"[::]:3030".parse().unwrap())
        .serve(router.into_make_service())
        .await
        .unwrap();
}

async fn account_handler(
    Path(account_id): Path<String>,
    Extension(db): Extension<DatabaseConnection>,
) -> Response {
    let account = match Accounts::find_by_id(account_id).one(&db).await {
        Ok(account) => account,
        Err(err) => {
            return (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()).into_response();
        }
    };
    match account {
        None => StatusCode::NOT_FOUND.into_response(),
        Some(account) => (StatusCode::OK, Json(account)).into_response(),
    }
}

async fn checks_handler(
    Path(account_id): Path<String>,
    Extension(db): Extension<DatabaseConnection>,
) -> Response {
    let result: Result<Vec<checks::Model>, DbErr> = Checks::find()
        .filter(checks::Column::AccountId.eq(account_id))
        .order_by_asc(checks::Column::Id)
        .all(&db)
        .await;
    match result {
        Ok(checks) => (StatusCode::OK, Json(checks)).into_response(),
        Err(err) => (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()).into_response(),
    }
}

async fn ledger_entries_handler(
    Path(account_id): Path<String>,
    Extension(db): Extension<DatabaseConnection>,
) -> Response {
    let result: Result<Vec<ledger_entries::Model>, DbErr> = LedgerEntries::find()
        .filter(ledger_entries::Column::AccountId.eq(account_id))
        .order_by_asc(ledger_entries::Column::Id)
        .all(&db)
        .await;
    match result {
        Ok(entries) => (StatusCode::OK, Json(entries)).into_response(),
        Err(err) => (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()).into_response(),
    }
}

// Serves as our command endpoint to make changes in a `BankAccount` aggregate.
async fn command_handler(
    Path(account_id): Path<String>,
    Json(command): Json<BankAccountCommand>,
    Extension(cqrs): Extension<Arc<SqliteCqrs<BankAccount>>>,
    MetadataExtension(metadata): MetadataExtension,
) -> Response {
    match cqrs
        .execute_with_metadata(&account_id, command, metadata)
        .await
    {
        Ok(_) => StatusCode::NO_CONTENT.into_response(),
        Err(err) => (StatusCode::BAD_REQUEST, err.to_string()).into_response(),
    }
}
