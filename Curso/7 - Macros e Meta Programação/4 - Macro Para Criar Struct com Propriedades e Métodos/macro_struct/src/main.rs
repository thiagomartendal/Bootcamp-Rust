#![allow(warnings)] // Suprime os avisos em todo o arquivo

macro_rules! cria_struct {
    // Captura o nome da struct e seus campos
    ($nome_struct:ident {$($campo:ident: $tipo:ty),* $(,)?}) => {
        // Define a struct

        #[derive(Debug)]
        struct $nome_struct {
            $($campo: $tipo,)*
        }

        // Implementação para a struct
        impl $nome_struct {
            // Método público para criar uma instância da struct
            pub fn new($($campo: $tipo),*) -> Self {
                Self {
                    $($campo),*
                }
            }

            // Método genérico de exemplo
            pub fn exemplo() {
                println!("Método de exemplo.")
            }
        }
    };
}

// Uso da macro para definir a struct com métodos
cria_struct! {
    Cliente {
        id: u32,
        nome: String,
        cpf: String
    }
}

fn macro_cliente() {
    let cliente = Cliente::new(1, String::from("Cliente 1"), String::from("123.456.789-00"));

    Cliente::exemplo();

    println!("{:?}", cliente);
}

macro_rules! cria_outra_struct {
    (
        $nome_struct:ident {
            $($campo:ident: $tipo:ty),* $(,)?
        }
        $(fn $nome_metodo:ident(&$nome_metodo_struct:ident $(, $param_nome:ident: $param_tipo:ty)*) -> $ret_tipo:ty $corpo_metodo:block)*
    ) => {
        struct $nome_struct {
            $($campo: $tipo,)*
        }

        impl $nome_struct {
            $(
                fn $nome_metodo(&$nome_metodo_struct $(, $param_nome: $param_tipo)*) -> $ret_tipo $corpo_metodo
            )*
        }
    };
}

cria_outra_struct! {
    Cliente2 {
        id: u32,
        nome: String,
        cpf: String
    }

    // Definição dos métodos
    fn mostra_id(&self) -> String {
        format!("ID: {}", self.id)
    }

    fn mostra_nome(&self) -> String {
        format!("Nome: {}", self.nome)
    }

    fn mais_numero_no_id(&self, numero: u32) -> String {
        format!("Id + Numero {}: {}", numero, self.id + numero)
    }
}

fn macro_cliente2() {
    let cliente = Cliente2 {
        id: 2,
        nome: String::from("Cliente 2"),
        cpf: String::from("987.654.321-00"),
    };

    println!("{}", cliente.mostra_id());
    println!("{}", cliente.mostra_nome());
    println!("{}", cliente.mais_numero_no_id(11));
}

fn main() {
    macro_cliente();
    println!("--------------------");
    macro_cliente2();
}
