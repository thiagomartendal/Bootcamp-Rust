fn encontrar_divisor(numero: i32) -> Option<i32> {
    if numero % 2 == 0 {
        Some(2) // Encontrou um divisor
    } else if numero % 3 == 0 {
        Some(3) // Encontrou outro divisor
    } else {
        None // Indica que nenhum divisor foi encontrado
    }
}

fn main() {
    // let numero = 6;
    let numero = 7;
    match encontrar_divisor(numero) {
        Some(divisor) => println!("Divisor encontrado: {}", divisor),
        None => println!("Nenhum divisor encontrado para {}", numero),
    }
}
