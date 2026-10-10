use std::io;

pub fn menu() -> u8 {
    println!("CRUD Clientes");
    println!("1 - Cadastrar Cliente");
    println!("2 - Listar Clientes");
    println!("3 - Atualizar Cliente");
    println!("4 - Excluir Cliente");
    println!("5 - Sair");

    let mut opcao = String::new();
    io::stdin()
        .read_line(&mut opcao)
        .expect("Falha ao ler a entrada.");

    opcao
        .trim()
        .parse()
        .expect("Falha ao converter o tipo de dado.")
}
