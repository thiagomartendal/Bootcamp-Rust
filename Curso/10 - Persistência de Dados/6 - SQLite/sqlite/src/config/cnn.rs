extern crate rusqlite;

use dotenv::dotenv;
use rusqlite::Connection;
use std::env;

pub fn obter_conexao() -> Connection {
    dotenv().ok();

    let database_path =
        env::var("DATABASE_PATH").expect("A variável DATABASE_PATH não está definida.");

    Connection::open(database_path).expect("Erro ao se conectar ao banco de dados.")
}
