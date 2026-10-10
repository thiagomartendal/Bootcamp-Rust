use std::io::{self, Write};

use crate::viesoes::ler;

pub fn entrada_dados_cliente() -> (String, String, String) {
    print!("Nome: ");
    io::stdout()
        .flush()
        .expect("Falha ao limpar o buffer de saída");

    let nome = ler::entrada_dados();

    print!("CPF: ");
    io::stdout()
        .flush()
        .expect("Falha ao limpar o buffer de saída");

    let cpf = ler::entrada_dados();

    print!("Endereço: ");
    io::stdout()
        .flush()
        .expect("Falha ao limpar o buffer de saída");

    let endereco = ler::entrada_dados();

    (nome, cpf, endereco)
}

pub fn capturar_id() -> usize {
    print!("ID do cliente: ");
    io::stdout()
        .flush()
        .expect("Falha ao limpar o buffer de saída");

    ler::entrada_dados()
        .parse()
        .expect("Falha ao converter a entrada para tipo numérico")
}

pub fn exibir_cliente((id, nome, cpf, endereco): (usize, &String, &String, &String)) {
    println!("ID: {}", id);
    println!("Nome: {}", nome);
    println!("CPF: {}", cpf);
    println!("Endereço: {}", endereco);
    println!("{}", String::from("-").repeat(20));
}
