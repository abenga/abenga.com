
// #[macro_use] extern crate rocket;

use rocket_contrib::templates::Template;
use rocket_contrib::serve::StaticFiles;

use crate::lib::db::utils::posts as posts_utils;

#[derive(serde::Serialize)]
struct IndexTemplateContext {
    title: &'static str,
    name: &'static str,
    items: Vec<&'static str>,
    // This key tells handlebars which template is the parent.
    parent: &'static str,
}


#[catch(404)]
fn not_found() -> String {
    format!("Not Found!")
}

#[get("/")]
pub fn index() -> Template {

    Template::render("pages/index", &IndexTemplateContext {
        title: "Home Page",
        name: "Horace",
        items: vec!["One", "Two", "Three"],
        parent: "base",
    })
}

// #[launch]
pub fn rocket() -> rocket::Rocket {
    rocket::ignite()
        .mount("/", routes![index])
        .mount("/static", StaticFiles::from("static"))
        .register(catchers![not_found])
        .attach(Template::fairing())
}