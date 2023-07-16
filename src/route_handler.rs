use crate::bank_account::commands::BankAccountCommand;
use crate::bank_account::models::{checks, ledger_entries, prelude::*};
use crate::metadata_extractor::MetadataExtractor;
use crate::state::ApplicationState;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use sea_orm::{ColumnTrait, DbErr, EntityTrait, QueryFilter, QueryOrder};

pub(crate) async fn account_handler(
    State(state): State<ApplicationState>,
    Path(account_id): Path<String>,
) -> Response {
    let account = match Accounts::find_by_id(account_id).one(&*state.db).await {
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

pub(crate) async fn checks_handler(
    State(state): State<ApplicationState>,
    Path(account_id): Path<String>,
) -> Response {
    let result: Result<Vec<checks::Model>, DbErr> = Checks::find()
        .filter(checks::Column::AccountId.eq(account_id))
        .order_by_asc(checks::Column::Id)
        .all(&*state.db)
        .await;
    match result {
        Ok(checks) => (StatusCode::OK, Json(checks)).into_response(),
        Err(err) => (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()).into_response(),
    }
}

pub(crate) async fn ledger_entries_handler(
    State(state): State<ApplicationState>,
    Path(account_id): Path<String>,
) -> Response {
    let result: Result<Vec<ledger_entries::Model>, DbErr> = LedgerEntries::find()
        .filter(ledger_entries::Column::AccountId.eq(account_id))
        .order_by_asc(ledger_entries::Column::Id)
        .all(&*state.db)
        .await;
    match result {
        Ok(entries) => (StatusCode::OK, Json(entries)).into_response(),
        Err(err) => (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()).into_response(),
    }
}

// Serves as our command endpoint to make changes in a `BankAccount` aggregate.
pub(crate) async fn command_handler(
    State(state): State<ApplicationState>,
    Path(account_id): Path<String>,
    MetadataExtractor(metadata): MetadataExtractor,
    Json(command): Json<BankAccountCommand>,
) -> Response {
    match state
        .cqrs
        .execute_with_metadata(&account_id, command, metadata)
        .await
    {
        Ok(_) => StatusCode::NO_CONTENT.into_response(),
        Err(err) => (StatusCode::BAD_REQUEST, err.to_string()).into_response(),
    }
}
