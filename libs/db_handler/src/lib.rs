pub mod db_handler;
mod queries;
use sqlx::{Pool, Postgres};
use std::{error::Error, sync::Arc};

pub trait IDBHandler {
    fn init(&self) -> impl std::future::Future<Output = Result<(), Box<dyn Error>>> + Send;
    fn add_plant(&self) -> impl std::future::Future<Output = Result<(), Box<dyn Error>>> + Send;
}

#[derive(Debug)]
pub struct DBHandler {
    pool: Arc<Pool<Postgres>>,
}
