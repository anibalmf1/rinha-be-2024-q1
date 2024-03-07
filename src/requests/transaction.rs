use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use validator::{Validate, ValidationError};

use crate::models::{Transaction};

#[derive(Validate, Deserialize, Serialize, Clone)]
pub struct TransactionPayload {

    #[validate(range(min=1))]
    #[serde(rename(deserialize = "valor"))]
    pub amount: i64,

    #[validate(custom(function = "validate_transaction_type"))]
    #[serde(rename(deserialize = "tipo"))]
    pub transaction_type: char,

    #[validate(length(min=1, max=10))]
    #[serde(rename(deserialize = "descricao"))]
    pub description: String,

}

fn validate_transaction_type(value: &char) -> Result<(), ValidationError> {
    match value {
        'c' => Ok(()),
        'd' => Ok(()),
        &_ => Err(ValidationError::new("INVALID_TYPE"))
    }
}

impl TransactionPayload {
    pub fn to_model(&self, customer_id: i64, created_at: DateTime<Utc>) -> Transaction {
        Transaction{
            customer_id,
            amount: self.amount,
            transaction_type: String::from(self.transaction_type),
            description: self.description.clone(),
            created_at,
        }
    }
}