mod routes;
mod templating;
mod routing;
mod db;

use std::env;
use std::net::SocketAddr;

use axum::{
    Router,
};
use tower_http::services::ServeDir;


#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    let app = Router::new()
        .merge(routes::pages::base::base_pages())
        .nest_service("/assets", ServeDir::new("assets"));

    // TODO: Config management can be fancier.
    let port: u16 = env::var("SITE_PORT")
        .unwrap_or_else(|_| String::from("8300"))
        .parse().expect("Invalid port number received!");

    let addr = SocketAddr::from(([127, 0, 0, 1], port));

    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
