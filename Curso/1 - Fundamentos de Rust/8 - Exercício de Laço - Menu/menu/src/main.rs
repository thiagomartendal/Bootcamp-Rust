use std::io;

fn main() {
    loop {
        println!("Seleciona uma das opções abaixo:");
        println!(r#"
            1 - Opção 1
            2 - Opção 2
            3 - Opção 3
            4 - Opção 4
        "#);

        let mut opcao = String::new();
        io::stdin().read_line(&mut opcao).expect("Erro ao ler a linha.");

        let opcao: i8 = opcao.trim().parse().expect("Erro ao converter a entrada.");
        match opcao {
            1 => println!("Opção 1"),
            2 => println!("Opção 2"),
            3 => println!("Opção 3"),
            4 => {
                println!("Opção 4");
                break;
            }
            _ => println!("Opção Inválida")
        }
    }
}
