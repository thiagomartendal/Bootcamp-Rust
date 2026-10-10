// Shadowing: redefinição de uma variável

fn main() {
    let x: i32 = 5; // Primeira definição
    println!("Valor de x: {} - Endereço de memória de x: {:p}", x, &x);

    let x: i32 = x + 1; // Segunda definição
    println!("Valor de x: {} - Endereço de memória de x: {:p}", x, &x);
    
    let x: i32 = x * 2; // Terceira definição
    println!("Valor final de x: {} - Endereço de memória de x: {:p}", x, &x);
}