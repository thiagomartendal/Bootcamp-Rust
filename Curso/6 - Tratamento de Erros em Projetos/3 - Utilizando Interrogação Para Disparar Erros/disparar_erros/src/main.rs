use std::fs::File;
use std::io;
use std::io::Read;

fn ler_arquivo(nome: &str) -> Result<String, io::Error> {
    let mut f = File::open(nome)?; // ? dispara erro caso este ocorra
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}

fn arquivo() {
    match ler_arquivo("arquivo.txt") {
        Ok(conteudo) => println!("Conteúdo do arquivo: {}", conteudo),
        Err(e) => println!("Falha ao abrir o arquivo: {}", e),
    }
}

fn calcular_raiz_quadrada(numero: f64) -> Result<f64, String> {
    if numero >= 0.0 {
        Ok(numero.sqrt()) // Retorna a raiz do númeor
    } else {
        Err(String::from("Números negativos não possuem raiz real")) // Retorna uma mensagem de erro
    }
}

fn raiz_quadrada() -> Result<f64, String> {
    let numero = -4.0;
    let r = calcular_raiz_quadrada(numero)?;
    Ok(r)
}

fn main() {
    arquivo();
    println!("---------------");
    println!("{:?}", raiz_quadrada());
}
