
// #[macro_use] extern crate rocket;


#[catch(404)]
fn not_found() -> String {
    format!("Not Found!")
}

#[get("/")]
pub fn index() -> &'static str {
    "Hello, world!"
}

// #[launch]
pub fn rocket() -> rocket::Rocket {
    rocket::ignite()
        .mount("/", routes![index])
        .register(catchers![not_found])
}