use async_trait::async_trait;
use cqrs_es::{EventEnvelope, Query};
use sea_orm::{
    ActiveModelTrait, ActiveValue::Set, DatabaseConnection, EntityTrait, TransactionTrait,
};
use std::default::Default;

use crate::bank_account::aggregate::BankAccount;
use crate::bank_account::events::BankAccountEvent;
use crate::bank_account::models::{accounts, checks, ledger_entries, prelude::Accounts};

pub struct BankAccountReadModelQuery {
    pub pool: DatabaseConnection,
}

// Our simplest query, this is great for debugging but absolutely useless in production.
// This query just pretty prints the events as they are processed.
#[async_trait]
impl Query<BankAccount> for BankAccountReadModelQuery {
    async fn dispatch(&self, aggregate_id: &str, events: &[EventEnvelope<BankAccount>]) {
        for event in events {
            match &event.payload {
                BankAccountEvent::AccountOpened { account_id } => {
                    let account = accounts::ActiveModel {
                        id: Set(account_id.to_string()),
                        balance: Set(0_f64),
                    };
                    account.insert(&self.pool).await.unwrap();
                }

                BankAccountEvent::CustomerDepositedMoney { amount, balance } => {
                    let txn = self.pool.begin().await.unwrap();

                    let account_result: Option<accounts::Model> =
                        Accounts::find_by_id(aggregate_id.to_string())
                            .one(&txn)
                            .await
                            .unwrap();
                    let mut account: accounts::ActiveModel = account_result.unwrap().into();
                    account.balance = Set(*balance);
                    account.update(&txn).await.unwrap();

                    let entry = ledger_entries::ActiveModel {
                        account_id: Set(aggregate_id.to_string()),
                        description: Set("deposit".to_string()),
                        amount: Set(*amount),
                        ..Default::default()
                    };
                    entry.insert(&txn).await.unwrap();

                    txn.commit().await.unwrap();
                }

                BankAccountEvent::CustomerWithdrewCash { amount, balance } => {
                    let txn = self.pool.begin().await.unwrap();
                    let account_result: Option<accounts::Model> =
                        Accounts::find_by_id(aggregate_id.to_string())
                            .one(&txn)
                            .await
                            .unwrap();
                    let mut account: accounts::ActiveModel = account_result.unwrap().into();
                    account.balance = Set(*balance);
                    account.update(&txn).await.unwrap();

                    let entry = ledger_entries::ActiveModel {
                        account_id: Set(aggregate_id.to_string()),
                        description: Set("atm_withdrawal".to_string()),
                        amount: Set(*amount),
                        ..Default::default()
                    };
                    entry.insert(&txn).await.unwrap();

                    txn.commit().await.unwrap();
                }

                BankAccountEvent::CustomerWroteCheck {
                    check_number,
                    amount,
                    balance,
                } => {
                    let txn = self.pool.begin().await.unwrap();

                    let account_result: Option<accounts::Model> =
                        Accounts::find_by_id(aggregate_id.to_string())
                            .one(&txn)
                            .await
                            .unwrap();
                    let mut account: accounts::ActiveModel = account_result.unwrap().into();
                    account.balance = Set(*balance);
                    account.update(&txn).await.unwrap();

                    let entry = ledger_entries::ActiveModel {
                        account_id: Set(aggregate_id.to_string()),
                        description: Set(check_number.clone()),
                        amount: Set(*amount),
                        ..Default::default()
                    };
                    entry.insert(&txn).await.unwrap();

                    let check = checks::ActiveModel {
                        account_id: Set(aggregate_id.to_string()),
                        check: Set(check_number.clone()),
                        ..Default::default()
                    };
                    check.insert(&txn).await.unwrap();

                    txn.commit().await.unwrap();
                }
            }
        }
    }
}
