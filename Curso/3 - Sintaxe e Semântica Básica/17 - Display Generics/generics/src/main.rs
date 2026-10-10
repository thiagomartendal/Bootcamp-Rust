use std::fmt::Display; // Trait padrão do Rust que possui to_string()

fn total_caracteres<T: Display>(valor: T) -> usize {
    valor.to_string().chars().count()
}

fn main() {
    let int: i32 = 12345;
    let float: f64 = 123.456;
    let slice_string: &str = "Olá Mundo!";
    let string: String = String::from("Um dia...");

    println!("Total de caracteres de {}: {}", int, total_caracteres(int));
    println!(
        "Total de caracteres de {}: {}",
        float,
        total_caracteres(float)
    );
    println!(
        "Total de caracteres de {}: {}",
        slice_string,
        total_caracteres(slice_string)
    );
    println!(
        "Total de caracteres de {}: {}",
        string.clone(),
        total_caracteres(string)
    );
}
