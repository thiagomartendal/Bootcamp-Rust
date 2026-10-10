/*
    Instalação do cargo-watch:

    cargo install cargo-watch

    Execução em modo watch:

    cargo watch -x run

    Build e execução em modo watch:

    cargo watch -x build
*/

fn main() {
    let x: String = String::from("Depuração");
    let x_modificada = alterando(x);
    println!("{}", x_modificada);
}

fn alterando(mut str: String) -> String {
    str += " - alterando";
    str
}