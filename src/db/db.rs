use std::time::Duration;

use sea_orm::{ConnectOptions, Database, DatabaseConnection};

pub async fn new() -> DatabaseConnection {
    let mut opt = ConnectOptions::new("postgres://test:test@localhost/test".to_owned());
    opt.max_connections(100)
        .min_connections(5)
        .connect_timeout(Duration::from_secs(8))
        .idle_timeout(Duration::from_secs(8))
        .max_lifetime(Duration::from_secs(8))
        .sqlx_logging(true);

    let rtn = Database::connect(opt).await;
    let db = rtn.unwrap();

    return db;
}
