use chrono::Utc;
use serde::{Deserialize, Serialize};

use crate::models::transaction::Customer;
use crate::models::TransactionCache;

#[derive(Deserialize, Serialize)]
pub struct CreateTransactionResponse {
    #[serde(rename(serialize = "limite"))]
    pub limit: u32,
    #[serde(rename(serialize = "saldo"))]
    pub balance: i64,
}

impl CreateTransactionResponse {
    pub fn from_model(customer: Box<Customer>) -> CreateTransactionResponse {
        CreateTransactionResponse{
            limit: customer.limit,
            balance: customer.balance,
        }
    }
}

#[derive(Deserialize, Serialize)]
pub struct GetStatementBalanceResponse{
    #[serde(rename(serialize = "total"))]
    pub balance: i64,
    #[serde(rename(serialize = "data_extrato"))]
    pub date: chrono::DateTime<Utc>,
    #[serde(rename(serialize = "limite"))]
    pub limit: u32,
}

impl GetStatementBalanceResponse {
    pub fn from_model(customer: &Customer) -> GetStatementBalanceResponse {
        GetStatementBalanceResponse{
            balance: customer.balance,
            date: Utc::now(),
            limit: customer.limit,
        }
    }
}

#[derive(Deserialize, Serialize)]
pub struct GetStatementTransactionsCacheResponse {
    #[serde(rename(serialize = "valor"))]
    pub amount: i64,
    #[serde(rename(serialize = "tipo"))]
    pub transaction_type: char,
    #[serde(rename(serialize = "descricao"))]
    pub description: String,
    #[serde(rename(serialize = "realizada_em"))]
    pub created_at: chrono::DateTime<Utc>,
}

impl GetStatementTransactionsCacheResponse {
    pub fn from_model_cache(
        transaction_cache: &TransactionCache,
    ) -> GetStatementTransactionsCacheResponse {
        GetStatementTransactionsCacheResponse{
            amount: transaction_cache.amount,
            transaction_type: transaction_cache.transaction_type,
            description: transaction_cache.description.clone(),
            created_at: *transaction_cache.created_at,
        }
    }
}

#[derive(Deserialize, Serialize)]
pub struct GetStatementResponse {
    #[serde(rename(serialize = "saldo"))]
    pub balance: GetStatementBalanceResponse,
    #[serde(rename(serialize = "ultimas_transacoes"))]
    pub transactions_cache: Vec<GetStatementTransactionsCacheResponse>,
}

impl GetStatementResponse {
    pub fn from_customer(customer: &Customer) -> GetStatementResponse {
        let transactions_cache = customer.transactions
            .iter()
            .map(|transaction| GetStatementTransactionsCacheResponse::from_model_cache(transaction))
            .collect();

        let balance = GetStatementBalanceResponse::from_model(customer);

        return GetStatementResponse{
            balance,
            transactions_cache,
        }
    }
}
