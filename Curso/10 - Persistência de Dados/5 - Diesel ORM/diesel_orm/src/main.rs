#[macro_use]
extern crate diesel;

mod config;
mod controle;
mod modelo;
mod repositorio;
mod schema;
mod visao;

use crate::controle::controle_aplicacao;

fn main() {
    controle_aplicacao::controle();
}
