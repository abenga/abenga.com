
// #[macro_use] extern crate rocket;

use rocket_contrib::templates::Template;
use rocket_contrib::serve::StaticFiles;

use crate::lib::db::utils::posts as posts_utils;
use crate::lib::db::models;

#[derive(serde::Serialize)]
struct IndexTemplateContext {
    title: &'static str,
    name: &'static str,
    //items: Vec<&'static str>,
    // This key tells handlebars which template is the parent.
    parent: &'static str,
}


#[derive(serde::Serialize)]
struct PostsTemplateContext<'a> {
    title: &'a str,
    name: &'a str,
    posts: Vec<models::Post>,
    // This key tells handlebars which template is the parent.
    parent: &'a str,
}


#[catch(404)]
fn not_found() -> String {
    format!("Not Found!")
}


#[get("/")]
pub fn index() -> Template {
    Template::render("pages/index", &IndexTemplateContext {
        title: "Horace Abenga",
        name: "Horace",
        parent: "base",
    })
}


#[get("/posts")]
pub fn posts() -> Template {
    let _post_series = posts_utils::post_series();
    let site_posts = posts_utils::posts();
    Template::render("pages/posts", &PostsTemplateContext {
        title: "Posts",
        name: "Horace",
        posts: site_posts,
        parent: "base",
    })
}


pub fn rocket() -> rocket::Rocket {
    rocket::ignite()
        .mount("/", routes![index, posts])
        .mount("/static", StaticFiles::from("static"))
        .register(catchers![not_found])
        .attach(Template::fairing())
}
