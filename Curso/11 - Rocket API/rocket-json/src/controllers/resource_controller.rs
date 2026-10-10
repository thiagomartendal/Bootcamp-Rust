use crate::dtos::resource_dto::ResourceDto;
use crate::models::resource::Resource;
use crate::services::resource_service;
use crate::views::json_error::JsonError;
use rocket::http::Status;
use rocket::response::status;
use rocket::serde::json::Json;

#[get("/resources")]
pub fn index() -> Json<Vec<Resource>> {
    let resources = resource_service::resources_list();
    Json(resources)
}

#[get("/resources/<id>")]
pub fn get_resource(id: u32) -> status::Custom<Json<Resource>> {
    let resource = resource_service::get_resource(id);

    status::Custom(Status::Ok, Json(resource))
}

#[post("/resources", data = "<resource_dto_json>")]
pub fn register(
    resource_dto_json: Json<ResourceDto>,
) -> Result<status::Custom<Json<Resource>>, status::Custom<Json<JsonError>>> {
    let resource_dto = resource_dto_json.into_inner();

    match resource_service::register_resource(resource_dto) {
        Ok(resource) => Ok(status::Custom(Status::Created, Json(resource))),
        Err(msg) => Err(status::Custom(
            Status::BadRequest,
            Json(JsonError { message: msg }),
        )),
    }
}

#[put("/resources/<id>", data = "<resource_dto_json>")]
pub fn update(
    id: u32,
    resource_dto_json: Json<ResourceDto>,
) -> Result<status::Custom<Json<Resource>>, status::Custom<Json<JsonError>>> {
    let resource_dto = resource_dto_json.into_inner();

    match resource_service::update_resource(id, resource_dto) {
        Ok(resource) => Ok(status::Custom(Status::Ok, Json(resource))),
        Err(msg) => Err(status::Custom(
            Status::BadRequest,
            Json(JsonError { message: msg }),
        )),
    }
}

#[delete("/resources/<id>")]
pub fn delete(id: u32) -> Result<status::Custom<Json<()>>, status::Custom<Json<JsonError>>> {
    match resource_service::delete_resource(id) {
        Ok(_) => Ok(status::Custom(Status::NoContent, Json(()))),
        Err(msg) => Err(status::Custom(
            Status::BadRequest,
            Json(JsonError { message: msg }),
        )),
    }
}
