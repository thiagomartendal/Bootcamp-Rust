use rayon::prelude::*;
use std::fs::File;
use std::io::{Result, Write};
use std::thread;
use std::time::Duration;

#[derive(Debug)]
struct Cliente {
    id: u32,
    nome: String,
}

fn salvar_cliente_transformado(cliente: &Cliente) -> Result<()> {
    let arquivo_nome = format!("cliente_{}.txt", cliente.id);
    let mut arquivo = File::create(arquivo_nome)?;
    writeln!(
        arquivo,
        "ID: {}\nNome: {}",
        cliente.id,
        cliente.nome.to_uppercase()
    )
}

fn main() -> Result<()> {
    let clientes = vec![
        Cliente {
            id: 1,
            nome: String::from("Cliente 1"),
        },
        Cliente {
            id: 2,
            nome: String::from("Cliente 2"),
        },
        Cliente {
            id: 3,
            nome: String::from("Cliente 3"),
        },
    ];

    clientes.par_iter().try_for_each(|cliente| {
        thread::sleep(Duration::from_millis(500));
        salvar_cliente_transformado(cliente)
    })?;

    Ok(())
}
