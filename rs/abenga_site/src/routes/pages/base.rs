use askama::Template;
use axum::{
    response::IntoResponse,
};


#[derive(Template)]
#[template(path = "pages/home.html")]
pub struct HomeTemplate {
    author: String,
    description: String,
    title: String,
}


pub async fn home_page() -> impl IntoResponse {
    let template = HomeTemplate {
        author: "Horace Abenga".to_string(),
        description: "Software engineer from Nairobi, Kenya.".to_string(),
        title: "Home page".to_string(),

    };
    crate::templating::HtmlTemplate(template)
}
