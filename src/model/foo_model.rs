use rocket::{response::Responder, serde::Serialize};

#[derive(Responder, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct FooModel {
    name: String,
}

impl FooModel {
    pub fn new(name: String) -> FooModel {
        FooModel { name }
    }
}
