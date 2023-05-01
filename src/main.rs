extern crate rocket;
use rocket::{launch, routes};
use rocket_dyn_templates::Template;
pub mod models;
pub mod schema;
mod services;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/api", routes![services::create_post])
        .mount("/api", routes![services::posts_as_json])
        .mount("/api", routes![services::get_post])
        .mount("/api", routes![services::delete_post])
        .mount("/", routes![services::list])
        .attach(Template::fairing())
}
