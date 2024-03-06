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
    #[serde(rename(serialize = "data_extrato"), with = "rinha_date_format")]
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
    #[serde(rename(serialize = "realizada_em"), with = "rinha_date_format")]
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

mod rinha_date_format {
    use chrono::{DateTime, Utc, NaiveDateTime};
    use serde::{self, Deserialize, Serializer, Deserializer};

    const FORMAT: &'static str = "%Y-%m-%dT%H:%M:%S%.6fZ";

    // The signature of a serialize_with function must follow the pattern:
    //
    //    fn serialize<S>(&T, S) -> Result<S::Ok, S::Error>
    //    where
    //        S: Serializer
    //
    // although it may also be generic over the input types T.
    pub fn serialize<S>(
        date: &DateTime<Utc>,
        serializer: S,
    ) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        let s = format!("{}", date.format(FORMAT));
        serializer.serialize_str(&s)
    }

    // The signature of a deserialize_with function must follow the pattern:
    //
    //    fn deserialize<'de, D>(D) -> Result<T, D::Error>
    //    where
    //        D: Deserializer<'de>
    //
    // although it may also be generic over the output types T.
    pub fn deserialize<'de, D>(
        deserializer: D,
    ) -> Result<DateTime<Utc>, D::Error>
        where
            D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let dt = NaiveDateTime::parse_from_str(&s, FORMAT).map_err(serde::de::Error::custom)?;
        Ok(DateTime::<Utc>::from_naive_utc_and_offset(dt, Utc))
    }
}