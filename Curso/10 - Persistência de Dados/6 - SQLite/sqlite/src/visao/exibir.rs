use crate::modelo::cliente::Cliente;
use std::io;
use std::process::Command;
use std::thread;
use std::time::Duration;

fn limpar_tela() {
    if cfg!(target_os = "windows") {
        Command::new("cls").args(&["/C", "cls"]).status().unwrap();
    } else {
        Command::new("clear").status().unwrap();
    }
}

fn pausa(segundos: u64) {
    thread::sleep(Duration::from_secs(segundos));
}

fn tecla_enter() {
    println!("Pressione Enter para continuar.");
    let mut tecla = String::new();
    io::stdin()
        .read_line(&mut tecla)
        .expect("Falha ao ler entrada do teclado.");
    limpar_tela();
}

pub fn cadastrar_cliente() -> (String, String) {
    limpar_tela();
    let mut nome = String::new();
    let mut telefone = String::new();

    println!("Cadastrar Cliente");

    println!("Nome:");
    io::stdin()
        .read_line(&mut nome)
        .expect("Falha ao ler o nome");

    println!("Telefone:");
    io::stdin()
        .read_line(&mut telefone)
        .expect("Falha ao ler o telefone");

    (nome, telefone)
}

pub fn mostrar_clientes(clientes: Vec<Cliente>) -> Result<(), std::io::Error> {
    limpar_tela();
    for cliente in &clientes {
        println!("--------------------");
        println!("ID: {}", cliente.id);
        println!("Nome: {}", cliente.nome);
        println!("Telefone: {}", cliente.telefone);
    }
    println!("--------------------");

    tecla_enter();

    Ok(())
}

pub fn atualizar_cliente() -> Result<(i32, String, String), Box<dyn std::error::Error>> {
    limpar_tela();
    let mut id = String::new();
    let mut nome = String::new();
    let mut telefone = String::new();

    println!("Informe o ID do cliente a ser atualizado:");
    io::stdin().read_line(&mut id).expect("Falha ao ler o ID.");
    let id = id.trim().parse::<i32>().expect("ID inválido.");

    println!("Novo nome:");
    io::stdin()
        .read_line(&mut nome)
        .expect("Falha ao ler o nome.");

    println!("Novo telefone:");
    io::stdin()
        .read_line(&mut telefone)
        .expect("Falha ao ler o telefone.");

    Ok((id, nome, telefone))
}

pub fn excluir_cliente() -> Result<i32, Box<dyn std::error::Error>> {
    limpar_tela();
    let mut id = String::new();

    println!("Informe o ID do cliente a ser removido:");
    io::stdin().read_line(&mut id).expect("Falha ao ler o ID.");
    let id = id.trim().parse::<i32>().expect("ID inválido.");

    Ok(id)
}

pub fn msg_sucesso(msg: &str) {
    println!("{}", msg);
    pausa(2);
    limpar_tela();
}

pub fn opcao_invalida() {
    println!("Opção inválida.");
    pausa(2);
    limpar_tela();
}
