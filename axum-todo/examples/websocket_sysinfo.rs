use axum::{
    error_handling::HandleErrorLayer,
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, patch},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
    time::Duration,
};
use std::net::SocketAddr;
use axum::extract::WebSocketUpgrade;
use axum::extract::ws::Message;
use axum::http::{HeaderValue, Method};
use sysinfo::{CpuExt, System, SystemExt};
use tokio::sync::Mutex;
use tower::{BoxError, ServiceBuilder};
use tower_http::cors::CorsLayer;
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use uuid::Uuid;


#[tokio::main]
async fn main() {
    let (tx, _) = tokio::sync::broadcast::channel(1);
    let state = AppState {
        tx: tx.clone(),
    };
    tokio::task::spawn_blocking( move ||{
        let mut system = System::new();
        loop {
            std::thread::sleep(std::time::Duration::from_millis(500));
            system.refresh_all();
            let cpu_metrics = system.cpus().iter().map(|c| c.cpu_usage()).collect::<Vec<f32>>();
            tx.send(cpu_metrics);
        }
    });
    let address = SocketAddr::from(([127, 0, 0, 1], 3003));
    //route
    let app = Router::new()
        .route("/realtime/cpuinfo", get(get_cpuinfo))
        .layer(
            CorsLayer::new()
                .allow_origin("http://localhost:4200".parse::<HeaderValue>().unwrap())
                .allow_methods([Method::GET, Method::POST, Method::DELETE, Method::PATCH])
        )
        .with_state(state);

    axum::Server::bind(&address).serve(app.into_make_service()).await.unwrap()
}

type Snapshot = Vec<f32>;

#[derive(Clone)]
struct AppState {
    tx: tokio::sync::broadcast::Sender<Snapshot>,
}

async fn get_cpuinfo( State(state): State<AppState>, ws: WebSocketUpgrade,) -> impl IntoResponse {
    ws.on_upgrade(|mut socket| async move {
        let mut rx = state.tx.subscribe();
        while let Ok(cpu_metrics) = rx.recv().await {
            // send as json
            let json = serde_json::to_string(&cpu_metrics).unwrap();
            socket.send(Message::from(json)).await.unwrap();
        }
    })
}