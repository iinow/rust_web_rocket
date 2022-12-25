use crate::model::foo_model::FooModel;
use rocket::serde::json::Json;

#[get("/")]
pub fn foo() -> Json<FooModel> {
    Json::from(FooModel::new(String::from("hello world")))
}
