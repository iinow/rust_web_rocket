use std::time::Instant;

use api::foo_api;

#[macro_use]
extern crate rocket;

mod api;
mod model;

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![foo_api::foo])
}
