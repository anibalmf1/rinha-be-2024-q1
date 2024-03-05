use actix_web::{post, get, HttpResponse, HttpServer, App, web::JsonConfig, web::Json, web::Path};
use actix_web::error::InternalError;
use actix_web::web::{Data, PathConfig};
use chrono::Utc;
use validator::{Validate};

mod models;
mod db;
mod config;
mod responses;
mod requests;

use models::{CustomerURL};
use requests::{TransactionPayload};
use config::Config;
use db::Database;
use crate::responses::{CreateTransactionResponse, GetStatementResponse};

#[post("/clientes/{customer_id}/transacoes")]
async fn create_transaction(
    customer_url: Path<CustomerURL>,
    payload: Json<TransactionPayload>,
    db: Data<Database>,
) -> HttpResponse {
    let is_valid = payload.validate();
    if is_valid.is_err() {
        return HttpResponse::UnprocessableEntity().into()
    }

    let customer_id = customer_url.customer_id;

    if customer_id < 0 {
        return HttpResponse::NotFound().into()
    }

    let customer_opt = db.get_customer_by_id(customer_id).await;
    if customer_opt.is_none() {
        return HttpResponse::NotFound().into()
    }

    let mut customer = customer_opt.unwrap();

    let new_balance = match payload.transaction_type {
        'c' => customer.balance + payload.amount,
        _ => customer.balance - payload.amount
    };

    if new_balance < (customer.limit as i64) * -1 {
        return HttpResponse::UnprocessableEntity().into()
    }

    customer.balance = new_balance;

    let time_now = Utc::now();

    db.create_transaction(payload.to_model(customer_id as i64, time_now)).await;

    let mut customer_box = Box::new(customer);

    if customer_box.transactions.len() >= 10 {
        customer_box.transactions.pop();
    }

    customer_box.transactions.insert(0, payload.to_model_cache(time_now));

    db.update_customer(customer_url.customer_id, &customer_box).await;

    HttpResponse::Ok().json(CreateTransactionResponse::from_model(customer_box))
}

#[get("/clientes/{customer_id}/extrato")]
async fn get_statement(
    customer_url: Path<CustomerURL>,
    db: Data<Database>,
) -> HttpResponse {
    let customer_id = customer_url.customer_id;

    if customer_id < 0 {
        return HttpResponse::NotFound().into()
    }

    let customer_opt = db.get_customer_by_id(customer_id).await;
    if customer_opt.is_none() {
        return HttpResponse::NotFound().into()
    }

    let customer = customer_opt.unwrap();

    HttpResponse::Ok().json(GetStatementResponse::from_customer(&customer))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();

    let config = envy::from_env::<Config>();
    if config.is_err() {
        panic!("{}", format!("invalid config - {:?}", config.err()))
    }

    let db = Database::init(&config.unwrap())
        .await.expect("could not connect to database - {}");

    HttpServer::new(move || App::new()
        .service(create_transaction)
        .service(get_statement)
        .app_data(Data::new(db.clone()))
        .app_data(
            JsonConfig::default().error_handler(|err, _| {
                let e = format!("{:?}", err);
                InternalError::from_response(err, HttpResponse::UnprocessableEntity().body(e)).into()
            })
        )
        .app_data(
            PathConfig::default().error_handler(|err, _| {
                let e = format!("{:?}", err);
                InternalError::from_response(err, HttpResponse::UnprocessableEntity().body(e)).into()
            })
        )
    )
        .bind("localhost:8080")?
        .run()
        .await
}

