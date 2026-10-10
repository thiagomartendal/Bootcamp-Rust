use std::io::{self, Write};

use crate::viesoes::ler;

fn ler_entrada() -> i32 {
    let opcao = ler::entrada_dados();

    if opcao == "" {
        -1
    } else {
        opcao
            .parse()
            .expect("Erro ao converter a entrada para um valor inteiro.")
    }
}

pub fn menu() -> i32 {
    println!("========== Menu ==========");
    println!("Escolha uma das opções abaixo:");
    println!("1 - Cadastrar cliente");
    println!("2 - Alterar cliente");
    println!("3 - Excluir cliente");
    println!("4 - Listar clientes");
    println!("0 - Sair\n");

    print!("Opção: ");
    io::stdout()
        .flush()
        .expect("Falha ao limpar o buffer de saída");

    ler_entrada()
}
