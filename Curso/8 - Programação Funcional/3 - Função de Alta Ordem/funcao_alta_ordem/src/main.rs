// Função de Alta Ordem e Closure
fn soma(x: i32, y: i32) -> impl Fn(i32) -> i32 {
    let r = x + y;
    move |multiplicador| r * multiplicador // Closure
}

fn soma2(x: i32, y: i32) -> impl Fn(i32) -> Box<dyn Fn(i32) -> i32> {
    let r = x + y;
    move |multiplicador| {
        let r_mult = r * multiplicador;
        Box::new(move |sub| r_mult - sub)
    }
}

fn criar_somador(x: i32) -> Box<dyn Fn(i32) -> i32> {
    Box::new(move |y| x + y)
}

fn main() {
    let resultado_soma = soma(5, 3); // 5+ 3 = 8
    let resultado_final = resultado_soma(2); // 8 * 2 = 16

    println!("Resultado da multiplicação: {}", resultado_final);

    println!("-----------------");

    let multiplica = soma2(5, 3); // 5 + 3 = 8
    let subtrai = multiplica(2); // 8 * 2 = 16
    let resultado = subtrai(4); // 16 - 4 = 12

    println!("O resultado é: {}", resultado);

    println!("-----------------");

    let somador_10 = criar_somador(10);
    let somador_20 = criar_somador(20);

    println!("10 + 5 = {}", somador_10(5));
    println!("20 + 10 = {}", somador_20(10));
}
