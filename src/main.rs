#![allow(dead_code)]
// use axum;
// use std::env;
// use std::net::SocketAddr;

use dotenv::dotenv;
use env_logger::{Builder, Env};
use log::{info, LevelFilter};

mod database;

#[tokio::main]
async fn main() {
    dotenv().expect("secrets file could not be loaded");

    info!(".env file loaded.");

    Builder::from_env(Env::default().default_filter_or("debug"))
        .format_timestamp(None)
        // .filter_module("sqlx::query", LevelFilter::Off)
        .format_module_path(false)
        .filter(None, LevelFilter::Info)
        .init();

    info!("Logger initialized.");

    let state = database::connection::connect().await;

    info!("database connected!")
}
