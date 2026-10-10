use std::io::{self, Write};

fn main() {
    menu();
}

fn menu() {
    loop {
        println!("Seleciona uma das opções abaixo:");
        println!(r#"
            1 - Adição
            2 - Subtração
            3 - Tabuada de um número
            4 - Sair
        "#);

        let mut opcao = String::new();
        io::stdin().read_line(&mut opcao).expect("Erro ao ler a linha.");

        let opcao: i8 = opcao.trim().parse().expect("Erro ao converter a entrada.");
        match opcao {
            1 => {
                let (x, y) = entrada(false);
                let add = adicao(x, y);
                println!("{} + {} = {}", x, y, add);
            },
            2 => {
                let (x, y) = entrada(false);
                let sub = subtracao(x, y);
                println!("{} - {} = {}", x, y, sub);
            },
            3 => {
                let (x, _) = entrada(true);
                tabuada(x);
            },
            4 => {
                println!("Saindo");
                break;
            }
            _ => println!("Opção Inválida")
        }
        println!("---------------------");
    }
}

fn entrada(entrada_unica: bool) -> (i32, i32) {
    let mut valor_x = String::new();
    let mut valor_y = String::new();

    print!("x = ");
    io::stdout().flush().expect("Erro ao limpar o buffer."); // Limpa o buffer
    io::stdin().read_line(&mut valor_x).expect("Erro ao ler a linha.");

    let mut y: i32 = 0;

    if !entrada_unica {
        print!("y = ");
        io::stdout().flush().expect("Erro ao limpar o buffer."); // Limpa o buffer
        io::stdin().read_line(&mut valor_y).expect("Erro ao ler a linha.");
        y = valor_y.trim().parse().expect("Erro ao converter a entrada.");
    }

    let x: i32 = valor_x.trim().parse().expect("Erro ao converter a entrada.");

    (x, y)
}

fn adicao(x: i32, y: i32) -> i32 {
    x + y
}

fn subtracao(x: i32, y: i32) -> i32 {
    x - y
}

fn tabuada(valor_tabuada: i32) {
    for multiplicador in 1..=10 {
        let calculo = valor_tabuada * multiplicador;
        println!("{} * {} = {}", valor_tabuada, multiplicador, calculo);
    }
}