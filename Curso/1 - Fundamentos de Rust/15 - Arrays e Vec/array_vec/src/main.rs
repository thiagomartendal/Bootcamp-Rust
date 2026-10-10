fn main() {
    // Array

    // Array de 5 inteiros
    let mut numeros: [i32; 5] = [1, 2, 3, 4, 5];
    println!("{:?}", numeros);

    // Iterando em um array
    for n in numeros.iter() {
        println!("{}", n);
    }

    // Acessando posições
    numeros[1] = 25;
    println!("{}", numeros[1]);
    println!("{:?}", numeros);

    // Tamanho do array
    println!("Tamanho: {}", numeros.len());

    // Vetor

    // Vetor vazio
    let mut vetor: Vec<i32> = Vec::new();

    // Adicionando elementos
    vetor.push(10);
    vetor.push(20);
    vetor.push(30);
    vetor.push(40);
    vetor.push(50);

    println!("{:?}", vetor);

    // Tamanho do vetor
    println!("Tamanho: {}", vetor.len());

    // Iterando o vetor
    for n in vetor.iter() {
        println!("{}", n);
    }

    // Retirando elementos do vetor
    let valor = vetor.pop();
    if let Some(n) = valor {
        println!("{:?}", n);
    }
    println!("{:?}", vetor);
}
