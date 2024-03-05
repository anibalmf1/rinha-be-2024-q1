use serde::Deserialize;

#[derive(Deserialize)]
pub struct Config {
    pub db_host: String,
    pub db_user: String,
    pub db_pass: String,
    pub db_namespace: String,
    pub db_name: String,
}