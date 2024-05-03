use axum::{
    routing::{get, post},
    Router,
};
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod controllers;
mod db;
mod dto;
mod models;
mod services;
mod state;

#[tokio::main]
async fn main() {
    // initialize tracing
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .init();

    let user_service = services::user::User::new();
    let app_state = state::app_state::AppState { user_service };
    // build our application with a route
    let app = Router::new()
        .route("/healthz", get(healthcheck))
        .route("/users", post(controllers::user::create_user))
        .layer(TraceLayer::new_for_http())
        .with_state(app_state);

    // run our app with hyper, listening globally on port 3000
    println!("listening on 0.0.0.0:3000");
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

// basic handler that responds with a static string
async fn healthcheck() -> &'static str {
    "OK"
}
