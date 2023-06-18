use axum::{
    routing::MethodRouter,
    Router,
};


pub fn build_router(path: &str, method_router: MethodRouter<()>) -> Router {
    Router::new().route(path, method_router)
}
