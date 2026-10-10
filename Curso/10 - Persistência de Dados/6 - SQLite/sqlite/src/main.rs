/*
    Comando para criar banco sql a partir de script sql:
    sqlite3 "db/rusqlite.db" < "db/script.sql"
*/

mod config;
mod controle;
mod modelo;
mod repositorio;
mod visao;

use crate::controle::controle_aplicacao;

fn main() {
    controle_aplicacao::controle();
}
