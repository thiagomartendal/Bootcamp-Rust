fn mensagem() -> String {
    let local = String::from("Mensagem teste");
    local // Ao retornar a variável, ocorre uma transferência de propriedade (ownership)
}

// 'a = anotação de lifetime (tempo de vida)
// 'a indica que as referências anotadas devem sobreviver por mais tempo
// que o escopo da função
fn quem_e_maior<'a>(x: &'a str, y: &'a str) -> &'a str {
    println!("Endereço de memória de x: {:?}", x.as_ptr());
    println!("Endereço de memória de y: {:?}", y.as_ptr());

    if x.len() > y.len() { x } else { y }
}

fn main() {
    let msg = mensagem();
    println!("{}", msg);

    println!("-----------------");

    let string1 = String::from("abcd");
    let string2 = String::from("xyz");

    println!("Endereço de memória de string1: {:?}", string1.as_ptr());
    println!("Endereço de memória de string2: {:?}", string2.as_ptr());

    let resultado = quem_e_maior(string1.as_str(), string2.as_str());

    println!("A maior string é {}", resultado);
    println!("Endereço de memória de resultado: {:?}", resultado.as_ptr());
}
