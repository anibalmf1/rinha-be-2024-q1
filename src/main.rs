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
mod errors;
mod serializers;

use models::{CustomerURL};
use requests::{TransactionPayload};
use config::{Config, LOGO};
use db::Database;
use crate::errors::Error;
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

    let time_now = Utc::now();

    let customer_lean = db.create_transaction(
        payload.to_model(customer_id as i64, time_now),
    ).await;

    match customer_lean {
        Ok(response) => {
            HttpResponse::Ok().json(CreateTransactionResponse::from_model(&response))
        }
        Err(err) => {
            match err {
                Error::NotFound => {HttpResponse::NotFound().into()}
                Error::Default => {HttpResponse::UnprocessableEntity().into()}
            }
        }
    }
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
    if customer_opt.is_err() {
        return HttpResponse::NotFound().into()
    }

    let customer = customer_opt.unwrap();

    HttpResponse::Ok().json(GetStatementResponse::from_customer(&customer))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    println!("{}", LOGO);

    let config = envy::from_env::<Config>().unwrap_or_else(|err|
        panic!("{}", format!("invalid config - {:?}", err))
    );

    let server_url = &config.server_url;

    let db = Database::init(&config).await.unwrap();


    let server = HttpServer::new(move || App::new()
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
        .bind(server_url)?
        .workers(4)
        .run();

    println!("listening on {}", server_url);

    server.await
}
