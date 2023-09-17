use std::net::SocketAddr;
use axum::{response::Html, routing::get, Router};
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() {
    // build our application with a route
    let app = Router::new().route("/", get(handler));
    tracing_subscriber::fmt::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();
    // run it
    let address = SocketAddr::from(([127, 0, 0, 1], 3003));
    axum::Server::bind(&address).serve(app.into_make_service()).await.unwrap()

}
#[tracing::instrument]
async fn handler() -> Html<&'static str> {
    Html("<h1>Hello, World!</h1>")
}