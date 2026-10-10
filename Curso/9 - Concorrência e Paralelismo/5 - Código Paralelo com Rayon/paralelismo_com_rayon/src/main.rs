use rayon::{ThreadPoolBuilder, prelude::*};

fn exemplo1() {
    let nums = vec![1, 2, 3, 4, 5];

    // Paralelizando a operação de mapeamento com Rayon
    let squares: Vec<_> = nums.par_iter().map(|&num| num + 1).collect();

    println!("Soma mais 1: {:?}", squares);
}

fn exemplo2() {
    // Limita a execução para 4 threads
    let num_threads = 4;
    let _ = ThreadPoolBuilder::new()
        .num_threads(num_threads)
        .build_global();

    let nums = vec![1, 2, 3, 4, 5];

    let mais_um: Vec<_> = nums.par_iter().map(|&num| num + 1).collect();

    println!("Soma mais 1: {:?}", mais_um);
}

fn main() {
    exemplo1();
    println!("----------------");
    exemplo2();
}
