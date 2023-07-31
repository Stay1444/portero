use axum::Json;
use axum::extract::{State, Path};
use axum::http::StatusCode;
use axum::response::{Response, IntoResponse};
use axum::{Router, routing::get};
use tracing::{info, error};

use crate::models::ApiResponse;
use crate::models::user::{User, self, UserForCreate, UserForUpdate};
use crate::services::UserService;
use crate::state::AppState;

use crate::error::Result;

pub fn routes(state: AppState) -> Router {
    Router::new()
        .route("/", 
            get(get_users)
            .post(create_user)
        )
        .route("/:id", 
            get(get_user)
            .delete(delete_user)
            .patch(update_user)
        )
        .with_state(state)
}

async fn get_users(
    State(us): State<UserService>
) -> Result<Json<ApiResponse<Vec<User>>>> {
    let users = us.list_users().await?;

    Ok(Json(ApiResponse {
        success: true,
        data: users
    }))
}

async fn get_user(
    State(us): State<UserService>,
    Path(id): Path<i64>
) -> Result<Response> {
    let user = us.get_user(id).await?;

    if user.is_none() {
        return Ok((
            StatusCode::NOT_FOUND,
            Json(ApiResponse {
                success: false,
                data: format!("User with id {id} not found.")
            })
        ).into_response());
    }

    Ok((StatusCode::OK, Json(ApiResponse {
        success: true,
        data: user
    })).into_response())
}

async fn create_user(
    State(us): State<UserService>,
    Json(user_fc): Json<UserForCreate>
) -> Result<Response> {
    let user = us.get_user_by_name(&user_fc.name).await
        .map_err(|err| {
            error!("create_user: {err}");
            err
        })?;

    if user.is_some() {
        return Ok((StatusCode::BAD_REQUEST, Json(ApiResponse {
            success: false,
            data: format!("User with name {name} already exists!", name = user_fc.name)
        })).into_response());
    }

    let user = us.create_user(user_fc).await
        .map_err(|err| {
            error!("create_user: {err}");
            err
        })?;

    info!("Created user {user:#?}");

    Ok(Json(ApiResponse {
        success: true,
        data: user
    }).into_response())
}

async fn delete_user(
    State(us): State<UserService>,
    Path(id): Path<i64>
) -> Result<Response> {
    let user = us.get_user(id).await?;

    if user.is_none() {
        return Ok((
            StatusCode::NOT_FOUND,
            Json(ApiResponse {
                success: false,
                data: format!("User with id {id} not found.")
            })
        ).into_response());
    }

    us.delete_user(id).await?;

    info!("Deleted user {id}");

    Ok((StatusCode::OK, Json(ApiResponse {
        success: true,
        data: user
    })).into_response())
}

async fn update_user(
    State(us): State<UserService>,
    Path(id): Path<i64>,
    Json(user_fu): Json<UserForUpdate>
) -> Result<Response> {
    let user = us.get_user(id).await
        .map_err(|err| {
            error!("update_user: {err}");
            err
        })?;

    if user.is_none() {
        return Ok((
            StatusCode::NOT_FOUND,
            Json(ApiResponse {
                success: false,
                data: format!("User with id {id} not found.")
            })
        ).into_response());
    }

    let user = us.update_user(id, user_fu).await?;
    
    info!("Updated user {user:#?}");

    Ok(Json(ApiResponse {
        success: true,
        data: user
    }).into_response())
}