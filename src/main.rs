use axum::{
    handler::Handler,
    http::StatusCode,
    routing::{get, post},
    Extension, Json, Router,
};
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod controllers;
mod dto;
mod models;
mod services;

#[tokio::main]
async fn main() {
    // initialize tracing
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .init();

    let user_service = services::user::User::new();
    let user_controller = controllers::user::User::new(user_service);
    // build our application with a route
    let app = Router::new()
        .route("/healthz", get(healthcheck))
        .route("/users", post(user_controller.create_user()))
        .layer(TraceLayer::new_for_http());

    // run our app with hyper, listening globally on port 3000
    println!("listening on 0.0.0.0:3000");
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

// basic handler that responds with a static string
async fn healthcheck() -> &'static str {
    "OK"
}
