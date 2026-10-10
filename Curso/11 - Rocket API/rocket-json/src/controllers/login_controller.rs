use crate::dtos::login_dto::LoginDto;
use crate::models::administrator_token::AdministratorToken;
use crate::services::administrator_service;
use crate::views::json_error::JsonError;
use rocket::http::Status;
use rocket::response::status;
use rocket::serde::json::Json;

#[post("/login", data = "<login_dto_json>")]
pub fn login(
    login_dto_json: Json<LoginDto>,
) -> Result<status::Custom<Json<AdministratorToken>>, status::Custom<Json<JsonError>>> {
    let login_dto = login_dto_json.into_inner();

    match administrator_service::login(login_dto.email.clone(), login_dto.password) {
        Ok(adm_token) => Ok(status::Custom(Status::Ok, Json(adm_token))),
        Err(msg) => Err(status::Custom(
            Status::InternalServerError,
            Json(JsonError { message: msg }),
        )),
    }
}
