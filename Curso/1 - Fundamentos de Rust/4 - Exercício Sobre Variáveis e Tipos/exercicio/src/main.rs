fn main() {
    let nome: &str = "João";

    let ano_nascimento: u16 = 1980;
    let mes_nascimento: u16 = 6;
    let dia_nascimento: u16 = 20;

    let ano_atual: u16 = 2026;
    let mes_atual: u16 = 5;
    let dia_atual: u16 = 29;
    let mut idade: u16 = ano_atual - ano_nascimento;

    if mes_nascimento > mes_atual {
        idade -= 1;
    } else {
        if dia_nascimento > dia_atual {
            idade -= 1;
        }
    }

    println!("A idade de {} é: {}/{}/{} - {}/{}/{} = {}", nome, dia_atual, mes_atual, ano_atual, dia_nascimento, mes_nascimento, ano_nascimento, idade);
}