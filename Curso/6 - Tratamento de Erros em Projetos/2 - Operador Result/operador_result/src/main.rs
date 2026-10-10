/*
    Assinatura da enum Result:
    enum Result<T, E> {
        Ok(T),
        Err(E)
    }
*/

fn calcular_raiz_quadrada(numero: f64) -> Result<f64, String> {
    if numero >= 0.0 {
        Ok(numero.sqrt()) // Retorna a raiz do númeor
    } else {
        Err(String::from("Números negativos não possuem raiz real")) // Retorna uma mensagem de erro
    }
}

fn main() {
    let numero = -4.0;
    // let numero = 4.0;
    match calcular_raiz_quadrada(numero) {
        Ok(raiz) => println!("A raiz quadrada de {} é {}", numero, raiz),
        Err(e) => println!("Erro: {}", e),
    }
}
