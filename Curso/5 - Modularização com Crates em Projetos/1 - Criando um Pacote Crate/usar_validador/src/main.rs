use std::io;
use validador as vd;

fn main() {
    println!("Digite seu CPF:");

    let mut cpf = String::new();
    io::stdin().read_line(&mut cpf).unwrap();

    cpf = cpf.trim().to_string();

    let validado: bool = vd::validar_cpf(cpf.as_str());
    if validado {
        println!("O CPF {} é válido.", cpf);
    } else {
        println!("O CPF {} é inválido.", cpf);
    }
}
