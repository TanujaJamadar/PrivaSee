use std::{env, sync::Arc};
use axum::Router;
use tower_http::services::ServeFile;
use tracing::{error, info};
use socketioxide::{extract::{Data, SocketRef}, SocketIo};

mod analyzer;
mod geo;
mod types;
mod worker;

use crate::types::AppState;
use crate::worker::run_node_worker;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();

    let geolite_path = env::var("GEOLITE_DB").unwrap_or_else(|_| "/app/GeoLite2-City.mmdb".into());
    let index_path = env::var("INDEX_HTML").unwrap_or_else(|_| "/app/dist/index.html".into());

    info!("Loading GeoIP Database from {geolite_path}...");
    let geo_reader = maxminddb::Reader::open_readfile(&geolite_path)
        .expect("Failed to load GeoLite2-City.mmdb.");

    let state = Arc::new(AppState { geo_reader });

    let (layer, io) = SocketIo::new_layer();

    io.ns("/", move |socket: SocketRef| {
        let state = state.clone();
        async move {
            info!("Client connected: {}", socket.id);

            socket.on("start-tracking", move |socket: SocketRef, Data::<String>(url)| {
                let state = state.clone();
                async move {
                    info!("Starting audit for: {}", url);
                    tokio::spawn(async move {
                        if let Err(e) = run_node_worker(socket, url, state).await {
                            error!("Worker failed: {}", e);
                        }
                    });
                }
            });
        }
    });

    let app = Router::new()
        .route_service("/", ServeFile::new(index_path))
        .layer(layer);

    let port: u16 = env::var("PORT").unwrap_or_else(|_| "3000".into()).parse()?;
    let addr = std::net::SocketAddr::from(([0, 0, 0, 0], port));

    info!("Server running on http://0.0.0.0:{port}");
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}

