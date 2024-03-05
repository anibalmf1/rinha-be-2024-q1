use surrealdb::engine::remote::ws::{Client, Ws};
use surrealdb::{Error, Surreal};
use surrealdb::opt::auth::Root;
use crate::config::Config;
use crate::models::Transaction;
use crate::models::transaction::Customer;

const CUSTOMER_TABLE: &'static str = "customers";
const TRANSACTIONS_TABLE: &'static str = "transactions";

#[derive(Clone)]
pub struct Database {
    pub client: Surreal<Client>,
}

impl Database {
    pub async fn init(config: &Config) -> Result<Self, Error> {
        let client = Surreal::new::<Ws>(&config.db_host).await?;
        client.signin(Root {
            username: &config.db_user,
            password: &config.db_pass,
        }).await.expect("Failed to connect to the database");

        client.use_ns(&config.db_namespace).use_db(&config.db_name).await.unwrap();

        let db = Database{ client };

        db.setup().await;

        Ok(db)
    }

    async fn setup(&self) {
        let customer1 = self.client.select::<Option<Customer>>((CUSTOMER_TABLE, 1)).await.unwrap();

        if !customer1.is_none() {
            return
        }

        self.client
            .create::<Option<Customer>>((CUSTOMER_TABLE, 1))
            .content(Customer { limit: 100000, balance: 0, transactions: Vec::new() }
        ).await.expect("couldn't create customer record");

        self.client
            .create::<Option<Customer>>((CUSTOMER_TABLE, 2))
            .content(Customer { limit: 80000, balance: 0, transactions: Vec::new() }
        ).await.expect("couldnt create customer record");

        self.client
            .create::<Option<Customer>>((CUSTOMER_TABLE, 3))
            .content(Customer { limit: 1000000, balance: 0, transactions: Vec::new() }
        ).await.expect("couldnt create customer record");

        self.client
            .create::<Option<Customer>>((CUSTOMER_TABLE, 4))
            .content(Customer { limit: 10000000, balance: 0, transactions: Vec::new() }
        ).await.expect("couldnt create customer record");

        self.client
            .create::<Option<Customer>>((CUSTOMER_TABLE, 5))
            .content(Customer { limit: 500000, balance: 0, transactions: Vec::new() }
        ).await.expect("couldnt create customer record");
    }

    pub async fn get_customer_by_id(&self, customer_id: i32) -> Option<Customer> {
        self.client.select((CUSTOMER_TABLE, customer_id)).await.ok()?
    }

    pub async fn create_transaction(
        &self,
        transaction: Transaction,
    )  {
        self.client
            .create::<Vec<Transaction>>(TRANSACTIONS_TABLE)
            .content(transaction)
            .await
            .expect("couldn't create transaction record");
    }

    pub async fn update_customer(
        &self,
        customer_id: i32,
        customer: &Box<Customer>,
    ) {
        self.client
            .update::<Option<Customer>>((CUSTOMER_TABLE, customer_id))
            .content(customer)
            .await
            .expect("couldn't update customer table");
    }
}