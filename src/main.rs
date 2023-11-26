#![allow(dead_code)]
// use axum;
// use std::env;
// use std::net::SocketAddr;

use std::{net::SocketAddr, env};

use dotenv::dotenv;
use env_logger::{Builder, Env};
use log::{info, LevelFilter};

mod database;
mod router;

#[tokio::main]
async fn main() {
    // TODO: don't need to panic here, info in .env could 
    //  just be set in actual shell env
    dotenv().expect("secrets file could not be loaded");

    info!(".env file loaded");

    Builder::from_env(Env::default().default_filter_or("debug"))
        .format_timestamp(None)
        .filter_module("sqlx::query", LevelFilter::Off)
        .format_module_path(false)
        .filter(None, LevelFilter::Info)
        .init();

    info!("Logger initialized.");

    let db_pool = database::connection::connect().await;

    info!("database connected!");

    let port = env::var("PORT")
        .unwrap_or(String::from("3000"))
        .parse::<u16>()
        .expect("could not load PORT from environment");

    let addr = SocketAddr::from(([127, 0, 0, 1], port));

    info!("starting server on {addr}");

    axum::Server::bind(&addr)
        .serve(router::get_routes().into_make_service())
        .await
        .expect("could not start axum server");
}
