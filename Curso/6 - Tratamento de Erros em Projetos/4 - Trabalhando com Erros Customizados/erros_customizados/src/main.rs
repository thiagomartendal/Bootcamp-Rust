use regex::Regex;
use std::error::Error;
use std::fmt;

#[derive(Debug)]
enum ValidationError {
    EmptyName(String),
    NonUniqueName(String),
    InvalidFormat(String),
}

// Implementando Display para o erro customizado
impl fmt::Display for ValidationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

// Implementação para que ValidationError possa ser tratado como um erro
impl Error for ValidationError {}

fn validar_nome(nome: &str, nomes_existentes: &[&str]) -> Result<(), Box<dyn Error>> {
    // Validar se o nome não está vazio
    if nome.is_empty() {
        return Err(Box::new(ValidationError::EmptyName(String::from(
            "Erro de validação: o nome não pode ser vazio.",
        ))));
    }

    // Validar se o nome é unico
    if nomes_existentes.contains(&nome) {
        return Err(Box::new(ValidationError::NonUniqueName(String::from(
            "Erro de validação: o nome não é único, use outro.",
        ))));
    }

    // Validar o formato do nome com regex
    let regex = Regex::new(r"^[a-zA-Z\s]+$").unwrap();
    if !regex.is_match(nome) {
        return Err(Box::new(ValidationError::InvalidFormat(String::from(
            "Erro de validação: o formato do nome é inválido.",
        ))));
    }

    Ok(())
}

fn main() {
    let nomes_existentes = vec!["Alice", "Roberto"];

    match validar_nome("Roberto", &nomes_existentes) {
        Ok(_) => println!("Nome válido"),
        Err(e) => println!("Erro de validação: {}", e),
    }

    match validar_nome("", &nomes_existentes) {
        Ok(_) => println!("Nome válido"),
        Err(e) => println!("Erro de validação: {}", e),
    }

    match validar_nome("123 Um Nome", &nomes_existentes) {
        Ok(_) => println!("Nome válido"),
        Err(e) => println!("Erro de validação: {}", e),
    }

    match validar_nome("Um Nome", &nomes_existentes) {
        Ok(_) => println!("Nome válido"),
        Err(e) => println!("Erro de validação: {}", e),
    }

    match validar_nome("Alice", &nomes_existentes) {
        Ok(_) => println!("Nome válido"),
        Err(e) => match e.downcast_ref::<ValidationError>().unwrap() {
            ValidationError::EmptyName(err) => println!("Erro de nulidade: {}", err),
            ValidationError::NonUniqueName(err) => println!("Erro de uniquidade: {}", err),
            ValidationError::InvalidFormat(err) => println!("Erro de formato: {}", err),
        },
    }
}
