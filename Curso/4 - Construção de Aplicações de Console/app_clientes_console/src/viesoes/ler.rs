use std::io;

pub fn entrada_dados() -> String {
    let mut dados = String::new();
    io::stdin()
        .read_line(&mut dados)
        .expect("Falha ao ler os dados do teclado.");
    dados.trim().to_string()
}
