// transaction.rs
use serde::{Deserialize, Serialize};
use surrealdb::sql::Datetime;
use validator::{Validate};

#[derive(Deserialize, Serialize, Clone)]
pub struct Transaction {
    pub customer_id: i64,
    pub amount: i64,
    pub transaction_type: char,
    pub description: String,
    pub created_at: Datetime,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct TransactionCache {
    pub amount: i64,
    pub transaction_type: char,
    pub description: String,
    pub created_at: Datetime,
}

#[derive(Validate, Deserialize, Serialize)]
pub struct CustomerURL {
    pub customer_id: i32,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct Customer {
    pub limit: u32,
    pub balance: i64,
    pub transactions: Vec<TransactionCache>
}
