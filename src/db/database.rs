use deadpool_postgres::{Pool};
use tokio_postgres::NoTls;
use uuid::Uuid;

use crate::errors::Error;
use crate::config::Config;
use crate::errors::Error::{Default, NotFound};
use crate::models::{Transaction, TransactionCache};
use crate::models::transaction::{Customer, CustomerLean};

#[derive(Clone)]
pub struct Database {
    pub pool: Pool
}

impl Database {
    pub async fn init(config: &Config) -> Result<Database, ()> {
        let mut pg_cfg = deadpool_postgres::Config::new();
        let host = config.db_host.split(":").collect::<Vec<&str>>();
        pg_cfg.host = Option::from(String::from(host[0]));
        if host.len() > 1 {
            pg_cfg.port = Option::from(host[1].parse::<u16>().unwrap());
        }
        pg_cfg.user = Option::from(config.db_user.clone());
        pg_cfg.password = Option::from(config.db_pass.clone());
        pg_cfg.dbname = Option::from(config.db_name.clone());
        pg_cfg.get_pool_config().max_size = 16;

        let pool = pg_cfg.create_pool(None, NoTls).unwrap();

        let db = Database{ pool };

        Ok(db)
    }

    pub async fn get_customer_by_id(&self, customer_id: i32) -> Result<Customer, Error> {
        let pg_client = self.pool.get().await.unwrap();

        let row = pg_client.query_one(
            "select credit_limit, balance, latest_transactions \
            from customer \
            where id = $1",
            &[&customer_id],
        ).await;

        if row.is_err() {
            return Err(NotFound)
        }

        let row = row.unwrap();

        let customer = Customer::from(row);

        Ok(customer)
    }

    pub async fn create_transaction(
        &self,
        transaction: Transaction,
    ) -> Result<CustomerLean, Error> {
        let mut pg_client = self.pool.get().await.unwrap();
        let db_transaction = pg_client.transaction().await.unwrap();

        let mut operation_amount = transaction.amount;
        if transaction.transaction_type == "d" {
            operation_amount = operation_amount * -1;
        }

        let transaction_json = serde_json::to_value(
            TransactionCache::from_transaction(&transaction),
        ).unwrap();

        let result = db_transaction.query_one("\
            update customer \
            set balance = balance + $1::bigint, \
                latest_transactions = $2::jsonb || \
                case \
                    when jsonb_array_length(latest_transactions) >= 10 then coalesce(latest_transactions - (-1), '[]'::jsonb) \
                    else coalesce(latest_transactions, '[]') \
                end \
            where id = $3::bigint \
            returning \
            credit_limit, balance",
   &[&operation_amount, &transaction_json, &transaction.customer_id]
        ).await;

        if result.is_err() {
            db_transaction.rollback().await.expect("fail to rollback");

            return Err(Default)
        }

        let customer_row = result.unwrap();

        let result = db_transaction.execute(
            "insert into transactions (\
            id, customer_id, amount, transaction_type, description\
            ) values (\
            $1::uuid, $2::bigint, $3::bigint, $4::varchar, $5::varchar\
            )",
            &[
                &Uuid::new_v4(),
                &transaction.customer_id,
                &transaction.amount,
                &transaction.transaction_type.to_string(),
                &transaction.description,
            ]
        ).await;

        if result.is_err() {
            db_transaction.rollback().await.expect("fail to rollback");
            return Err(Default)
        }
        
        db_transaction.commit().await.expect("fail commit");

        Ok(CustomerLean{
            limit: customer_row.get(0),
            balance: customer_row.get(1),
        })
    }
}