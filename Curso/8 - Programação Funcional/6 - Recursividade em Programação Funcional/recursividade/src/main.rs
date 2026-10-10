fn soma_recursiva(n: i32) -> i32 {
    if n == 0 { 0 } else { n + soma_recursiva(n - 1) }
}

fn main() {
    let n = 5;
    println!("Soma recursiva: {}", soma_recursiva(n));
}
