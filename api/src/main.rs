#![allow(unused)]

use std::{net::SocketAddr, env};

use axum::{Router, response::{Html, IntoResponse}, routing::get, extract::Query};
use models::AppConfig;
use serde::Deserialize;
use sqlx::postgres::PgPoolOptions;
use tracing::{info, trace, debug, Level, error, warn};

use crate::{state::AppState, services::UserService};

pub use self::error::{Error, Result};

mod models;
mod error;
mod controllers;
mod services;
mod state;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .pretty()
        .init();

    let db = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://postgres:root@localhost/portero").await.unwrap();

    let admin_password = match env::var("ADMIN_PASSWORD") {
        Ok(v) => v,
        Err(err) => {
            warn!("Error getting ADMIN_PASSWORD: {err}.
            Please specify a valid ADMIN_PASSWORD environment variable!

            Using default password: ADMIN1234");
            
            "ADMIN1234".to_string()
        }
    };

    let state = AppState {
        config: AppConfig { admin_password },
        user_service: UserService::new(db.clone())
    };

    let app = Router::new()
        .merge(controllers::routes(state));

    let addr = SocketAddr::from(([0,0,0,0], 8080));

    info!("Starting server");
    
    let server = axum::Server::bind(&addr)
        .serve(app.into_make_service_with_connect_info::<SocketAddr>());

    let server = tokio::spawn(server);
    
    info!("Ready, listening on http://{addr}");

    server.await.unwrap();
}