#[macro_use]
extern crate rocket;

mod controllers;
mod dtos;
mod middlewares;
mod models;
mod services;
mod views;

use controllers::{home_controller, login_controller, resource_controller};

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(middlewares::auth_guard::JwtFairing)
        .mount(
            "/",
            routes![
                home_controller::index,
                home_controller::unauthorized,
                resource_controller::index,
                resource_controller::register,
                resource_controller::update,
                resource_controller::get_resource,
                resource_controller::delete,
                login_controller::login
            ],
        )
}
