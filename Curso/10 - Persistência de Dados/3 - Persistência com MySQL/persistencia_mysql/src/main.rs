mod config;
mod controle;
mod modelo;
mod repositorio;
mod visao;

use crate::controle::controle_aplicacao;

fn main() {
    controle_aplicacao::controle();
}
