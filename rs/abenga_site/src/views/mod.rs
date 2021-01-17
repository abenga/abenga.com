
// #[macro_use] extern crate rocket;

use std::str::FromStr;

use rocket::response as rocket_response;

use rocket_contrib::templates::Template;
use rocket_contrib::serve::StaticFiles;
use rocket_contrib::uuid::Uuid;
use rocket_contrib::uuid::uuid_crate;

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

#[get("/post/<post_uuid_str>")]
pub fn show_post(post_uuid_str: String) -> Template {
    // let post_uuid = uuid_crate::Uuid::parse_str(&post_uuid_str).unwrap();
    let post_uuid = Uuid::from_str(&post_uuid_str).unwrap();
    let post = posts_utils::get_post(post_uuid);
    Template::render("pages/post", &PostTemplateContext {
        title: "Posts",
        name: "Horace",
        post: post.unwrap(),
        parent: "base",
    })
}


// /2014/11/13/button-widgets/
#[get("/post/<year_posted>/<month_posted>/<day_posted>/<joined_title>")]
pub fn visit_old_url(year_posted: i32, month_posted: i32,
                     day_posted: i32, joined_title: String) -> rocket_response::Redirect {
    let post_uuid_str = posts_utils::get_uid_from_ymd_and_title(
        year_posted, month_posted, day_posted, joined_title
    ).unwrap();
    // let post_uuid = uuid_crate::Uuid::parse_str(&post_uuid_str).unwrap();
    rocket_response::Redirect::to(uri!(show_post: post_uuid_str))
}


pub fn rocket() -> rocket::Rocket {
    rocket::ignite()
        .mount("/", routes![index, show_recent_posts, show_post, visit_old_url])
        .mount("/static", StaticFiles::from("static"))
        .register(catchers![not_found])
        .attach(Template::fairing())
}
