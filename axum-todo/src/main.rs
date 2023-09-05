use std::net::SocketAddr;
use std::sync::Arc;
use std::time::Duration;

use axum::{
    error_handling::HandleErrorLayer,
    http::StatusCode,
    Router,
    routing::{get, patch},
};
use tower::{BoxError, ServiceBuilder};
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};


use models::{DashMapRepo, TodoRepository};
use handlers::{todos_create, todos_delete, todos_index, todos_update};
mod handlers;
mod models;

#[tokio::main]
async fn main() {
    // db does not live long enough, so it must be explicitly annotated
    let db =  DashMapRepo::default();
    let db = Arc::new(db);

    // Compose the routes
    let app = Router::new()
        .route("/todos", get(todos_index).post(todos_create))
        .route("/todos/:id", patch(todos_update).delete(todos_delete))
        // Add middleware to all routes
        .layer(
            ServiceBuilder::new()
                .layer(HandleErrorLayer::new(|error: BoxError| async move {
                    if error.is::<tower::timeout::error::Elapsed>() {
                        Ok(StatusCode::REQUEST_TIMEOUT)
                    } else {
                        Err((
                            StatusCode::INTERNAL_SERVER_ERROR,
                            format!("Unhandled internal error: {}", error),
                        ))
                    }
                }))
                .timeout(Duration::from_secs(10))
                .layer(TraceLayer::new_for_http())
                .into_inner(),
        )
        .with_state(db);


    let address = SocketAddr::from(([127, 0, 0, 1], 3002));
    axum::Server::bind(&address).serve(app.into_make_service()).await.unwrap()

}


