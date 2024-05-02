use crate::controllers;
use crate::dto;
use crate::models;
use crate::services;
use axum::handler::Handler;
use axum::{http::StatusCode, Json};

pub struct User {
    svc: services::user::User,
}

impl User {
    pub fn new(svc: services::user::User) -> Self {
        Self { svc }
    }

    pub fn create_user(
        &self,
        Json(payload): Json<dto::user_dto::CreateUser>,
    ) -> (
        StatusCode,
        Json<controllers::types::Response<models::user::User>>,
    ) {
        let rtn = self.svc.create_user(payload);
        let resp = match rtn {
            Ok(user) => controllers::types::Response::new_ok(user),
            Err(error) => {
                controllers::types::Response::new_error(100, StatusCode::BAD_REQUEST, error)
            }
        };
        (StatusCode::OK, Json(resp))
    }
}
