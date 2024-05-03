use crate::controllers;
use crate::dto;
use crate::models;
use crate::state;
use axum::extract::State;
use axum::{http::StatusCode, Json};

pub async fn create_user(
    State(state::AppState { user_service }): State<state::AppState>,
    Json(payload): Json<dto::user_dto::CreateUser>,
) -> (
    StatusCode,
    Json<controllers::types::Response<models::user::User>>,
) {
    let rtn = user_service.create_user(payload);
    let resp = match rtn {
        Ok(user) => controllers::types::Response::new_ok(user),
        Err(error) => controllers::types::Response::new_error(100, StatusCode::BAD_REQUEST, error),
    };
    (StatusCode::OK, Json(resp))
}
