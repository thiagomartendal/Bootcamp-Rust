fn main() {
    // Loop infinito
    // loop {
    //     println!("Olá Mundo!");
    // }

    // Laço while (enquanto)
    let mut x = 0;
    while x < 20 {
        x += 1; // Antes de continue, senão, o incremento não é feito
        if x == 5 || x == 10 {
            continue;
        }
        println!("{}", x);
    }

    println!("-------------------");

    // Laço for (para)
    for numero in 1..4 {
        println!("{}", numero);
    }
    for numero in 1..=4 {
        println!("{}", numero);
    }
}
