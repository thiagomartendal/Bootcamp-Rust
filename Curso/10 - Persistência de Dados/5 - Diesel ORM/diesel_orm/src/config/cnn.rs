use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

pub fn obter_conexao() -> MysqlConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("A variável DATABASE_URL não foi definida.");

    MysqlConnection::establish(&database_url)
        .expect(&format!("Erro ao conectar-se em {}", database_url))
}
