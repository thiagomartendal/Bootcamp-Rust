#[macro_use]
extern crate minha_macro_lib;

cria_struct! {
    Cliente {
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

    fn mostra_cpf(&self) -> String {
        format!("CPF: {}", self.cpf)
    }
}

fn main() {
    let cliente1 = Cliente {
        id: 1,
        nome: String::from("Cliente 1"),
        cpf: String::from("123.456.789-00"),
    };

    let cliente2 = Cliente {
        id: 2,
        nome: String::from("Cliente 2"),
        cpf: String::from("987.654.321-00"),
    };

    mostra_cliente(&cliente1);
    println!("---------------");
    mostra_cliente(&cliente2);
}

fn mostra_cliente(cliente: &Cliente) {
    println!("{}", cliente.mostra_id());
    println!("{}", cliente.mostra_nome());
    println!("{}", cliente.mostra_cpf());
}
