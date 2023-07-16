use crate::bank_account::aggregate::BankAccount;
use crate::config::cqrs_framework;
use sea_orm::{ConnectOptions, Database, DatabaseConnection};
use sqlite_es::{default_sqlite_pool, SqliteCqrs};
use std::sync::Arc;
use std::time::Duration;
use tracing::log;

#[derive(Clone)]
pub struct ApplicationState {
    pub cqrs: Arc<SqliteCqrs<BankAccount>>,
    pub db: Arc<DatabaseConnection>,
}

pub async fn new_application_state() -> ApplicationState {
    // Configure the CQRS framework, backed by an SQLite database, along with two queries:
    // - a simply-query prints events to stdout as they are published
    // - `account_query` stores the current state of the account in a ViewRepository that we can access
    //
    // The needed database tables are automatically configured with `docker-compose up -d`,
    // see init file at `/db/init.sql` for more.
    let uri = "sqlite://demo.db";
    let pool = default_sqlite_pool(uri).await;
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
    let db_conn = Database::connect(opt).await.unwrap();

    let cqrs = cqrs_framework(pool, db_conn.clone());
    let db = Arc::new(db_conn);

    ApplicationState { cqrs, db }
}
