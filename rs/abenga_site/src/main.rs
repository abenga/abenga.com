mod routes;
mod templating;

use std::net::SocketAddr;

use axum::{
    routing::get,
    Router,
};
use tower_http::services::ServeDir;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};


#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
    .with(
        tracing_subscriber::EnvFilter::try_from_default_env()
            .unwrap_or_else(|_| "example_templates=debug".into()),
    )
    .with(tracing_subscriber::fmt::layer())
    .init();

    let app = Router::new().route(
        "/", get(routes::pages::base::home_page)
    ).nest_service(
        "/assets", ServeDir::new("assets")
    );

    // run it
    let addr = SocketAddr::from(([127, 0, 0, 1], 8300));
    tracing::info!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
