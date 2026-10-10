#[macro_use]
extern crate rocket;

mod controllers;
mod models;
mod services;

use controllers::client_controller;
use controllers::home_controller;
use rocket::fs::{FileServer, relative};
use rocket_dyn_templates::Template;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount(
            "/",
            routes![
                home_controller::index,
                home_controller::route,
                client_controller::clients,
                client_controller::new_client,
                client_controller::edit_client,
                client_controller::create,
                client_controller::update,
                client_controller::delete_client
            ],
        )
        .mount("/static", FileServer::from(relative!("static")))
        .attach(Template::fairing())
}
