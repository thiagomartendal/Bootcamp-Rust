fn main() {
    // Tupla com três elementos diferentes
    let tupla: (i32, f64, u8) = (500, 6.4, 1);

    // Acessando os elementos por desestruturação
    let (x, y, z) = tupla;

    println!("{:?}", tupla);

    println!("x: {}", x);
    println!("y: {}", y);
    println!("z: {}", z);

    println!("0: {}", tupla.0);
    println!("1: {}", tupla.1);
    println!("2: {}", tupla.2);

    /*
        Tuplas podem ter seus valores alterados, mas novos valores
        não podem ser adicionados
        (1, 2.2, 3000) -> (2, 3.3, 4000) - Correto
        (1, 2.2, 3000) -> (2, 3.3, 4000, 5000) - Incorreto
    */

    let (largura, altura) = dimensoes();
    println!("{}x{}", largura, altura);

    let tupla_aninhada: ((i32, i32), String) = ((5, 10), String::from("String"));
    println!("{:?}", tupla_aninhada)
}

// Função que retorna uma tupla
fn dimensoes() -> (i32, i32) {
    let largura = 30;
    let altura = 50;
    (largura, altura)
}