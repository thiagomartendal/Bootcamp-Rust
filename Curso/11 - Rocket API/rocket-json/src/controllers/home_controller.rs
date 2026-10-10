use crate::views::{home::Home, json_error::JsonError};
use rocket::{http::Status, response::status, serde::json::Json};

#[get("/")]
pub fn index() -> Json<Home> {
    Json(Home {
        message: String::from("Olá Mundo!"),
        endpoints: vec![String::from("/resources")],
    })
}

#[get("/unauthorized")]
pub fn unauthorized() -> status::Custom<Json<JsonError>> {
    let error_message = JsonError {
        message: String::from("Acesso não autorizado sem autenticação."),
    };
    status::Custom(Status::Unauthorized, Json(error_message))
}
