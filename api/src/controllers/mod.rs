use axum::Router;

use crate::state::AppState;

mod auth;
mod users;

pub fn routes(state: AppState) -> Router {
    Router::new()
        .nest("/auth", auth::routes(state.clone()))
        .nest("/users", users::routes(state.clone()))
    }