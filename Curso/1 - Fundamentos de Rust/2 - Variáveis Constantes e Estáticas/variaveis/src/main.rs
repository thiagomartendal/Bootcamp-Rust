const CONSTANTE:i8 = 2; // Constante Global
static ESTATICA:i8 = 3; // Estatica Global
static mut ESTATICA_MUTAVEL:i8 = 3; // Estatica Global

fn main() {
    println!("Constante: {}", CONSTANTE);
    println!("Estática: {}", ESTATICA);
    imprime();

    unsafe {
        /*
            A definição de variáveis estáticas mutáveis é considerada insegura
            devido a comportamentos anômalos que podem ocorrer em multiplas threads,
            por isso operações que envolvem variáveis estáticas mutáveis são feitas
            em blocos unsafe.
         */
        ESTATICA_MUTAVEL = 4;
        let v = ESTATICA_MUTAVEL;
        println!("Estática Mutável: {}", v);
    }
}

fn imprime() {
    println!("Constante: {}", CONSTANTE);
    println!("Estática: {}", ESTATICA);
}