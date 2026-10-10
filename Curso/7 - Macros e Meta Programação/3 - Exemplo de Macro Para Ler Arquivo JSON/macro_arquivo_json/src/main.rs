use serde::{Deserialize, Serialize};
use serde_json::Result;
use std::fs;

macro_rules! cria_struct {
    ($nome:ident {$($campo:ident: $tipo:ty),* $(,)?}) => {
        #[derive(Debug, Serialize, Deserialize)]
        struct $nome {
            $($campo: $tipo),*
        }
    };
}

cria_struct! {
    Cliente {
        id: u32,
        nome: String,
        cpf: String
    }
}

fn main() -> Result<()> {
    // Lendo arquivo JSON
    let dados = fs::read_to_string("clientes.json").expect("Falha ao ler o arquivo.");

    // Deserializa o JSON para um Vec
    let clientes: Vec<Cliente> = serde_json::from_str(&dados)?;

    // Iterando sobre o vetor de clientes
    for cliente in clientes {
        println!("{:?}", cliente);
    }

    Ok(())
}
