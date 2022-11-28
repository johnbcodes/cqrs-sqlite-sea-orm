use std::sync::Arc;

use cqrs_es::Query;
use sea_orm::DatabaseConnection;
use sqlite_es::SqliteCqrs;
use sqlx::{Pool, Sqlite};

use crate::bank_account::aggregate::BankAccount;
use crate::bank_account::queries::BankAccountReadModelQuery;
use crate::services::{BankAccountServices, HappyPathBankAccountServices};

pub fn cqrs_framework(
    sqlx_pool: Pool<Sqlite>,
    sea_orm_pool: DatabaseConnection,
) -> Arc<SqliteCqrs<BankAccount>> {
    let bank_account_query = BankAccountReadModelQuery { pool: sea_orm_pool };
    // Create and return an event-sourced `CqrsFramework`.
    let queries: Vec<Box<dyn Query<BankAccount>>> = vec![Box::new(bank_account_query)];
    let services = BankAccountServices::new(Box::new(HappyPathBankAccountServices));
    Arc::new(sqlite_es::sqlite_cqrs(sqlx_pool, queries, services))
}
