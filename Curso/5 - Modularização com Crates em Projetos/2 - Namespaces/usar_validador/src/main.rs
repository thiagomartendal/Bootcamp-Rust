use std::io;
use validador as vd;

fn validar(opcao: u8) {
    let titulo = if opcao == 1 { "CPF" } else { "CNPJ" };

    println!("Digite seu {}:", titulo);

    let mut entrada = String::new();
    io::stdin().read_line(&mut entrada).unwrap();

    entrada = entrada.trim().to_string();

    let validado: bool = if opcao == 1 {
        vd::validadores::cpf(entrada.as_str())
    } else {
        vd::validadores::cnpj(entrada.as_str())
    };

    if validado {
        println!("O {} {} é válido.", titulo, entrada);
    } else {
        println!("O {} {} é inválido.", titulo, entrada);
    }
}

fn main() {
    println!("Validar:");
    println!("1 - CPF");
    println!("2 - CNPJ");

    let mut opcao = String::new();
    io::stdin().read_line(&mut opcao).unwrap();

    let opcao: u8 = opcao.trim().parse().unwrap();

    if opcao != 1 && opcao != 2 {
        println!("Opção inválida.");
        return;
    }

    validar(opcao);
}
