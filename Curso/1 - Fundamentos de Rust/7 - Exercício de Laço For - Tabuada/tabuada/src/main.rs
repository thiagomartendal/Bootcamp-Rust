use std::io::{self, Write};

fn main() {
    print!("Informe a tabuada: ");

    io::stdout().flush().expect("Erro ao limpar o buffer."); // Limpa o buffer de saída

    let mut valor_tabuada = String::new();
    io::stdin().read_line(&mut valor_tabuada).expect("Erro ao ler a linha.");

    let valor_tabuada: i32 = valor_tabuada.trim().parse().expect("Erro ao converter a entrada.");
    for multiplicador in 1..=10 {
        let calculo = valor_tabuada * multiplicador;
        println!("{} * {} = {}", valor_tabuada, multiplicador, calculo);
    }
}
