#[derive(Clone)]
pub struct User;

use uuid::Uuid;

use crate::dto;
use crate::models;

impl User {
    pub fn new() -> Self {
        Self
    }
    pub fn create_user(
        &self,
        payload: dto::user_dto::CreateUser,
    ) -> Result<models::user::User, String> {
        let user = models::user::User {
            id: Uuid::new_v4().to_string(),
            username: payload.username,
        };
        Ok(user)
    }
}
