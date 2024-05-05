use crate::services;
use sea_orm::DatabaseConnection;
#[derive(Clone)]
pub struct AppState {
    pub user_service: services::user::User,
    pub db: DatabaseConnection,
}
