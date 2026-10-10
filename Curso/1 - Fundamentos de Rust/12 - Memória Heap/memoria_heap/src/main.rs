fn main() {
    // Variáveis na memória heap

    let s1: String = String::from("Olá Mundo!"); // s1 é proprietária da string
    let s2 = s1; // s2 passa a ser a nova proprietária da string (Borrowing)
    // let s2 = s1.clone(); // Faz uma cópia de s1 - não altera a propriedade de s1

    /*
        A propriedade e transferida devido ao elevado tamanho da memória heap,
        fazendo com que a variável s1 deixe de existir
    */
    
    // println!("s1: {}", s1); // Causa erro pois a variável s1 não é mais válida

    println!("s2: {} - {:p}", s2, &s2);

    // String -> tipo para strings
    // &str -> slice de string

    let s3: &str = "Olá Mundo!";
    let s4 = s3;
    println!("{} - {:p}", s3, s3);
    println!("{} - {:p}", s4, s4);

    let s5 = format!("{}", s3); // Converte slice para String
    let s6 = s5.as_str(); // Converte String para slice de string
    println!("{} - {:p}", s5, &s5);
    println!("{} - {:p}", s6, &s6);
    
    let substring: &str = &s5[0..4]; // Formou uma substring em um slice - de 0 a 3
    println!("{} - {:p}", substring, &substring);

    let s7: String = substring.to_string(); // Também converte slice para String
    println!("{} - {:p}", s7, &s7);
}
