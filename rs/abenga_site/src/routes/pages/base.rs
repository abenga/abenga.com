use askama::Template;
use axum::{
    response::IntoResponse,
    Router,
    routing::get
};

use crate::routes::utils::route;


#[derive(Template)]
#[template(path = "pages/home.html")]
pub struct HomeTemplate {
    author: String,
    description: String,
    title: String,
}


pub fn base_pages() -> Router {
    async fn home() -> impl IntoResponse {
        let template = HomeTemplate {
            author: "Horace Abenga".to_string(),
            description: "Software engineer from Nairobi, Kenya.".to_string(),
            title: "Home page".to_string(),

        };
        crate::templating::HtmlTemplate(template)
    }

    route("/", get(home))
}
