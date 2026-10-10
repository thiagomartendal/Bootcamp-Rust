fn main() {
    let y = 5; // Imutável
    let mut x: i64 = 5; // Mutável (palavra chave mut)
    x += 6;
    println!("y = {}", y);
    println!("x = {}", x);
}
