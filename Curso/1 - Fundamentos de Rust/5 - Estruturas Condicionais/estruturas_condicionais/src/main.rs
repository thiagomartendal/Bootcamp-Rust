fn main() {
    let numero = 3;
    let resultado: i32 = if numero > 3 {
        4
    } else {
        2
    };

    println!("{}", resultado);

    if numero == 3 {
        println!("O número era 3");
    } else {
        println!("O número não era 3");
    }

    match numero {
        1 => println!("Um"),
        2 => println!("Dois"),
        3 => println!("Três"),
        _ => println!("Outro Número"),
    }
}