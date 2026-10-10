fn main() {
    let x = 4;
    let exemplo_closure = |parametro: i32| parametro + x;

    println!("Exemplo Closure: {}", exemplo_closure(6));

    let igual_a_x = |z| z == x;
    let y = 4;

    println!("{}", igual_a_x(y));
}
