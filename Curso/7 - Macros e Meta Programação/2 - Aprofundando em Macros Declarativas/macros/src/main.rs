// Macro para implementar structs
macro_rules! cria_structs {
    // Para structs sem campos
    ($nome:ident) => {
        #[derive(Debug)]
        struct $nome;
    };

    // Para structs com um campo
    ($nome:ident, $campo:ident: $tipo:ty) => {
        struct $nome {
            $campo: $tipo,
        }
    };

    // Para structs com 2 campos
    ($nome:ident, $campo1:ident: $tipo1:ty, $campo2:ident: $tipo2:ty) => {
        struct $nome {
            $campo1: $tipo1,
            $campo2: $tipo2,
        }
    };
}

fn structs() {
    // Cria uma struct sem campos
    cria_structs!(Vazia);

    // Criando uma struct com um capo
    cria_structs!(UmCampo, campo1: u32);

    // Criando uma struct com dois campos
    cria_structs!(DoisCampos, campo1: u32, campo2: f64);

    let vazia = Vazia {};
    let um_campo = UmCampo { campo1: 10 };
    let dois_campos = DoisCampos {
        campo1: 10,
        campo2: 4.5,
    };

    println!("Vazia: {:?}", vazia);
    println!("UmCampo: {}", um_campo.campo1);
    println!(
        "Dois Campos: {} - {}",
        dois_campos.campo1, dois_campos.campo2
    );
}

// Macro para implementar métodos
macro_rules! implementa_metodos {
    // Aceita o nome da struct seguido por uma sequência de identificadores
    ($struct:ident, $($metodo:ident),*) => {
        // Para cada identificador dornecido, gera um método que imprime uma mensagem
        impl $struct {
            $(
                fn $metodo(&self) {
                    println!("{}::{} foi chamado", stringify!($struct), stringify!($metodo));
                }
            )*
        }
    };
}

// Definindo uma struct de exemplo
struct ExemploStruct;

// Usando a macro prar adicionar métodos a struct ExemploStruct
implementa_metodos!(ExemploStruct, metodo_a, metodo_b, metodo_c);

fn metodos() {
    let exemplo = ExemploStruct;

    exemplo.metodo_a();
    exemplo.metodo_b();
    exemplo.metodo_c();
}

// Macro para criar atributos
macro_rules! define_struct_com_stributos {
    // A macro aceita o nome da struct seguido por uma lista de pares
    // (nome do atributo: tipo do atributo)
    ($nome:ident, $($campo:ident: $tipo:ty),*) => {
        struct $nome {
            // Gera um campo para cada par nome:tipo fornecido
            $(
                $campo: $tipo,
            )*
        }
    };
}

// Usando a macro para definir uma nova struct com atributos especificados
define_struct_com_stributos!(
    Pessoa,
    nome: String,
    idade: u8,
    email: String
);

fn atributos() {
    let pessoa = Pessoa {
        nome: String::from("Pessoa"),
        idade: 30,
        email: String::from("email@email.com"),
    };

    println!("Nome: {}", pessoa.nome);
    println!("Idade: {}", pessoa.idade);
    println!("Email: {}", pessoa.email);
}

fn main() {
    structs();
    println!("----------------");
    metodos();
    println!("----------------");
    atributos();
}
