use crate::models::dtos::client_dto::ClientDTO;
use crate::services::client_service;
use rocket::form::Form;
use rocket::request::FlashMessage;
use rocket::response::{Flash, Redirect};
use rocket_dyn_templates::{Template, context};

fn error_msg(flash: Option<FlashMessage<'_>>) -> String {
    let mut error = String::new();
    if let Some(msg) = flash {
        if msg.kind() == "error" {
            error = msg.message().to_string();
        }
    }
    error
}

#[get("/clients")]
pub fn clients() -> Template {
    let clients_list = client_service::get_clients();
    Template::render("clients/clients", context! {clients: &clients_list})
}

#[get("/clients/new")]
pub fn new_client(flash: Option<FlashMessage<'_>>) -> Template {
    Template::render("clients/new", context! {error: error_msg(flash)})
}

#[get("/clients/<id>/update")]
pub fn edit_client(id: u32, flash: Option<FlashMessage<'_>>) -> Template {
    let client = client_service::get_client_by_id(id);

    Template::render(
        "clients/edit",
        context! {
            client: client,
            error: error_msg(flash)
        },
    )
}

#[get("/clients/<id>/delete")]
pub fn delete_client(id: u32) -> Result<Redirect, Flash<Redirect>> {
    if client_service::delete_client(id) {
        Ok(Redirect::to("/clients"))
    } else {
        Err(Flash::error(
            Redirect::to("/clients/{}/update"),
            "Erro ao excluir o cliente.",
        ))
    }
}

#[post("/clients/create", data = "<client_form>")]
pub fn create(client_form: Form<ClientDTO>) -> Result<Redirect, Flash<Redirect>> {
    let client = client_form.into_inner();

    if client_service::create_client(client.name, client.cpf) {
        Ok(Redirect::to("/clients"))
    } else {
        Err(Flash::error(
            Redirect::to("/clients/new"),
            "Erro ao cadastrar o cliente.",
        ))
    }
}

#[post("/clients/<id>/update", data = "<client_form>")]
pub fn update(id: u32, client_form: Form<ClientDTO>) -> Result<Redirect, Flash<Redirect>> {
    let client = client_form.into_inner();

    if client_service::update_client(id, client.name, client.cpf) {
        Ok(Redirect::to("/clients"))
    } else {
        Err(Flash::error(
            Redirect::to(format!("/clients/{}/update", id)),
            "Erro ao alterar os dados do cliente.",
        ))
    }
}
