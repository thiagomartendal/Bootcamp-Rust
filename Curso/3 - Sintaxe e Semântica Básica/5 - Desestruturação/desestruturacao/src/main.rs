struct Pessoa {
    nome: String,
    idade: u8
}

enum Mensagem {
    Enviar {id: u32, texto: String}
}

fn desestruturacao_struct() {
    let pessoa = Pessoa {
        nome: String::from("Pessoa 1"),
        idade: 30
    };

    // Desestruturação de Struct
    let Pessoa {nome, idade} = pessoa;

    println!("{} - {}", nome, idade);
}

fn desestruturacao_array() {
    let arr = [1, 2, 3, 4, 5];

    // Desestruturação de Array
    let [v1, v2, v3, v4, v5] = arr;
    // Não é obrigatório fazer a desestruturação de todos os elementos do array
    // let [v1, v2, ..] = arr;

    println!("{} - {} - {} - {} - {}", v1, v2, v3, v4, v5);
}

fn desestruturacao_tupla() {
    let tupla = ("Alô", 12345, 4.5);

    // Desestruturação de Tupla
    let (str, int, float) = tupla;
    // let (str, int, ..) = tupla; // Também é possível ignorar valores

    println!("{} - {} - {}", str, int, float);
}

fn desestruturacao_enum() {
    let msg = Mensagem::Enviar {
        id: 1,
        texto: String::from("Olá Mundo!")
    };

    match msg {
        Mensagem::Enviar {id, texto} => {
            println!("{} - {}", id, texto);
        }
    }
}

fn main() {
    desestruturacao_struct();
    println!("---------------");
    desestruturacao_array();
    println!("---------------");
    desestruturacao_tupla();
    println!("---------------");
    desestruturacao_enum();
}