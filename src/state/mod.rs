pub mod app_state;

use crate::services;

#[derive(Clone)]
pub struct AppState {
    pub user_service: services::user::User,
}
