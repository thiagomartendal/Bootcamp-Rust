fn main() {
    let x: i32 = 4;
    let y: i32 = x; // copia o valor de x
    let z: &i32 = &x; // referência de memória para o valor de x
    // let z = &x;
    let w = *z; // desreferência do ponteiro z

    println!("Valor de x: {} - {:p}", x, &x);
    println!("Valor de y: {} - {:p}", y, &y);
    println!("Valor de z: {} - {:p}", z, z);
    println!("Valor de w: {} - {:p}", w, &w);

    let mut valor: i32 = 4;
    println!("Valor: {} - {:p}", valor, &valor);
    imutavel(&mut valor);
    mutavel(&mut valor);
    println!("Valor alterado: {} - {:p}", valor, &valor);
}

fn imutavel(valor: &mut i32) {
    // valor += 1; // Não permitido - o valor não é alterado dessa forma
    println!("{} - {:p}", valor, &valor);
}

fn mutavel(valor: &mut i32) {
    *valor += 1; // Permitido - o valor pode ser modificado ao desreferenciar o ponteiro
    println!("{} - {:p}", valor, &valor);
}