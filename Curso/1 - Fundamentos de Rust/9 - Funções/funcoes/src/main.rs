fn main() {
    println!("{}", retorna_string());

    let n1 = 99;
    let n2 = {
        let x = 99;
        x + 1
    };
    let r = soma(n1, n2);

    println!("{}", r);

    mostra_na_tela(1)
}

fn soma(x:i32, y:i32) -> i32 {
    x + y
}

fn retorna_string() -> String {
    String::from("Olá Mundo!")
}

fn mostra_na_tela(i: i32) {
    println!("{}", i);
    if i < 10 {
        mostra_na_tela(i+1);
    }
}