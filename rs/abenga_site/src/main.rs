mod routes;
mod templating;

use std::net::SocketAddr;

use axum::{
    Router,
};
use tower_http::services::ServeDir;


#[tokio::main]
async fn main() {
    // TODO: Change log level depending on release type
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    let app = Router::new()
        .merge(routes::pages::base::base_pages())
        .nest_service("/assets", ServeDir::new("assets"));

    let addr = SocketAddr::from(([127, 0, 0, 1], 8300));
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
