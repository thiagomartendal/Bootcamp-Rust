use regex::Regex;

fn main() {
    let s1: String = "Olá Mundo!".to_string();
    println!("{}", s1);
    
    /*
    Outras formas de inicializar strings vazias
    let s2: String = String::new();
    let s3: String = String::from("");
    let s4: String = "".to_string();
    */

    // s1.trim(); // Remove espaços das bordas
    // s1.trim_start(); // Remove espaços da borda esquerda
    // s1.trim_end(); // Remove espaços da borda direita

    let s2 = s1.to_uppercase(); // Converte as letras para maiúsculas
    println!("{}", s2);

    let s3 = s1.to_lowercase(); // Converte as letras para minúsculas
    println!("{}", s3);

    let s4 = s1.replace("Mundo!", "Alô"); // Substitui uma substring por outra
    println!("{}", s4);

    let s5: String = s1.chars().rev().collect(); // Inverte a string
    println!("{}", s5);

    let contem: bool = s1.contains("Mundo"); // Checa se a string contém uma substring
    println!("{}", contem);

    let palavras: Vec<&str> = s1.split(' ').collect(); // Quebra a string a partir de uma substring ou caractere
    println!("{:?}", palavras);

    let substring: &str = &s1[0..6]; // Forma uma substring em um slice
    println!("{}", substring);

    let s6: String = format!("{}", substring); // Converte uma slice de string para String
    println!("{}", s6);

    // Exoressões Regulares
    let email_regex: Regex = Regex::new(r"^\w+@\w+\.\w+$").unwrap();
    let email = "email@email.com";

    if email_regex.is_match(email) {
        println!("{} é um email válido.", email);
    } else {
        println!("{} é um email inválido.", email);
    }

    let mut s7: String = String::from("Olá");
    s7.push_str(" Mundo");
    s7 += "!";
    println!("{}", s7);
}
