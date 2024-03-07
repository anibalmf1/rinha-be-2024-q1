use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use tokio_postgres::Row;
use validator::{Validate};

use crate::serializers::rinha_date_format;

#[derive(Deserialize, Serialize, Clone)]
pub struct Transaction {
    pub customer_id: i64,
    pub amount: i64,
    pub transaction_type: String,
    pub description: String,
    #[serde(with = "rinha_date_format")]
    pub created_at: DateTime<Utc>,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct TransactionCache {
    pub amount: i64,
    pub transaction_type: String,
    pub description: String,
    #[serde(with = "rinha_date_format")]
    pub created_at: DateTime<Utc>,
}

#[derive(Validate, Deserialize, Serialize)]
pub struct CustomerURL {
    pub customer_id: i32,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct Customer {
    pub limit: i64,
    pub balance: i64,
    pub transactions: Vec<TransactionCache>
}

#[derive(Deserialize, Serialize, Clone)]
pub struct CustomerLean{
    pub limit: i64,
    pub balance: i64,
}

impl TransactionCache {
    pub fn from_transaction(transaction: &Transaction) -> TransactionCache {
        TransactionCache{
            amount: transaction.amount,
            transaction_type: transaction.transaction_type.clone(),
            description: transaction.description.clone(),
            created_at: transaction.created_at,
        }
    }
}

impl From<Row> for Customer {
    fn from(row: Row) -> Self {
        let latest_transactions: Option<serde_json::Value> = row.get("latest_transactions");

        let mut transactions = vec![];

        if latest_transactions.is_some() {
            transactions = serde_json::from_value(latest_transactions.unwrap()).unwrap();
        }

        Self {
            limit: row.get("credit_limit"),
            balance: row.get("balance"),
            transactions,
        }
    }
}
