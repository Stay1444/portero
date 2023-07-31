use std::{time::Duration, net::SocketAddr};

use axum::{Json, Router, routing::{post, get}, http::StatusCode, response::IntoResponse, extract::{State, ConnectInfo}, debug_handler};
use rand::{rngs::OsRng, distributions::Uniform, prelude::Distribution, Rng, SeedableRng};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use tracing::{debug, info};

use crate::{Error, Result, models::{ApiResponse, AppConfig}, state::AppState};

pub fn routes(state: AppState) -> Router {
    Router::new()
        .route("/login", post(login))
        .with_state(state)
}

#[derive(Debug, Deserialize)]
struct LoginRequest {
    password: String
}

#[derive(Debug, Serialize)]
struct LoginFailResponse {
    reason: String
}

#[derive(Debug, Serialize)]
struct LoginSuccessResponse {
    token: String
}

#[debug_handler]
async fn login(
    State(config): State<AppConfig>, 
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    Json(login_request): Json<LoginRequest>
) -> Result<impl IntoResponse> {
    if login_request.password != config.admin_password {

        info!("Admin login failed! Client address: {addr}");

        // Sleep during X random milliseconds to prevent time based attacks.
        // Not needed for a portero application, but since there are some "Kali Graciosos" in muevetef, 
        // I might as well do it        
        tokio::time::sleep(Duration::from_millis({
            rand::rngs::StdRng::from_entropy().gen_range(15..250)
        })).await;

        let body = ApiResponse {
            success: false,
            data: LoginFailResponse {
                reason: "Incorrect Password".to_string()
            }
        };

        let response = (
            StatusCode::UNAUTHORIZED, 
            serde_json::to_string(&body).map_err(|_| Error::SerializationFail)?
        ).into_response();

        return Ok(response);
    }

    info!("Client {addr} logged in as an administrator.");

    let body = ApiResponse {
        success: true,
        data: LoginSuccessResponse {
            token: "AWDADAW".to_string()
        }
    };

    Ok(Json(body).into_response())
}