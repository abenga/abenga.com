mod routes;
mod templating;
mod routing;

use std::net::SocketAddr;

use axum::{
    Router,
};
use tower_http::services::ServeDir;


#[tokio::main]
async fn main() {
    // TODO: Read log level from config.
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    let app = Router::new()
        .merge(routes::pages::base::base_pages())
        .nest_service("/assets", ServeDir::new("assets"));

    // TODO: Read address and port from config.
    let addr = SocketAddr::from(([127, 0, 0, 1], 8300));
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
