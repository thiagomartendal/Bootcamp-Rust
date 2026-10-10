mod config;
mod modelos;

use modelos::cliente::Cliente;
use mysql::prelude::Queryable;

fn main() -> Result<(), mysql::Error> {
    let mut cnn = config::cnn::obter_conexao()?;

    let clientes: Vec<Cliente> = cnn.query_map(
        "SELECT id, nome, telefone FROM cliente",
        |(id, nome, telefone)| Cliente {
            id: id,
            nome: nome,
            telefone: telefone,
        },
    )?;

    for cliente in clientes {
        println!("----------------------------");
        println!("ID: {}", cliente.id);
        println!("Nome: {}", cliente.nome);
        println!("Telefone: {}", cliente.telefone);
    }
    println!("----------------------------");

    Ok(())
}
