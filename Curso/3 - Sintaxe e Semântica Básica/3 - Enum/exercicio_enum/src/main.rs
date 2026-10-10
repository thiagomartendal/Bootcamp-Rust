// Definição da Enum
enum Tipo {
    Fisica,
    Juridica
}

// Enum com tipos nas variáveis
// enum Tipo {
//     Fisica(i32),
//     Juridica(i32)
// }

// Enum com tipos mais complexos
// enum Tipo {
//     Fisica(i32, String),
//     Juridica(i32, String)
// }

// Definição da Struct
struct Pessoa {
    nome: String,
    documento: String,
    tipo: Tipo
}

// Definição dos Métodos
impl Pessoa {
    fn registro(&self) {
        print!("{} - {} - ", self.nome, self.documento);

        match self.tipo {
            Tipo::Fisica => println!("Pessoa física."),
            Tipo::Juridica => println!("Pessoa jurídica.")
        }
    }
}

fn main() {
    let pessoa1 = Pessoa{
        nome: "Pessoa 1".to_string(),
        documento: String::from("123.456.789-00"),
        tipo: Tipo::Fisica
    };

    let pessoa2 = Pessoa{
        nome: "Pessoa 2".to_string(),
        documento: String::from("12.345.678/9000-00"),
        tipo: Tipo::Juridica
    };

    pessoa1.registro();
    pessoa2.registro();
}