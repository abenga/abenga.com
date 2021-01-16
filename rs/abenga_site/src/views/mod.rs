
// #[macro_use] extern crate rocket;

use rocket_contrib::templates::Template;
use rocket_contrib::serve::StaticFiles;

use rocket_contrib::uuid::Uuid;
use rocket_contrib::uuid::uuid_crate as uuid;

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
struct PostListTemplateContext<'a> {
    title: &'a str,
    name: &'a str,
    posts: Vec<models::Post>,
    // This key tells handlebars which template is the parent.
    parent: &'a str,
}


#[derive(serde::Serialize)]
struct PostTemplateContext<'a> {
    title: &'a str,
    name: &'a str,
    post: models::Post,
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
pub fn show_recent_posts() -> Template {
    let _post_series = posts_utils::post_series();
    let most_recent_posts = posts_utils::posts();
    Template::render("pages/posts", &PostListTemplateContext {
        title: "Posts",
        name: "Horace",
        posts: most_recent_posts,
        parent: "base",
    })
}

#[get("/post/<uid>")]
pub fn show_post(uid: Uuid) -> Template {
    let post = posts_utils::get_post(uid);
    Template::render("pages/post", &PostTemplateContext {
        title: "Posts",
        name: "Horace",
        post: post.unwrap(),
        parent: "base",
    })
}


pub fn rocket() -> rocket::Rocket {
    rocket::ignite()
        .mount("/", routes![index, show_recent_posts, show_post])
        .mount("/static", StaticFiles::from("static"))
        .register(catchers![not_found])
        .attach(Template::fairing())
}
