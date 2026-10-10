use crate::models::client::Client;

pub fn get_clients() -> Vec<Client> {
    vec![
        Client {
            id: 1,
            name: String::from("Cliente 1"),
            cpf: String::from("123.456.789-00"),
        },
        Client {
            id: 2,
            name: String::from("Cliente 2"),
            cpf: String::from("987.654.321-00"),
        },
        Client {
            id: 3,
            name: String::from("Cliente 3"),
            cpf: String::from("111.222.333-00"),
        },
    ]
}

pub fn get_client_by_id(id: u32) -> Client {
    let index: usize = (id - 1) as usize;
    Client {
        id: get_clients()[index].id,
        name: get_clients()[index].name.clone(),
        cpf: get_clients()[index].cpf.clone(),
    }
}

pub fn create_client(name: String, cpf: String) -> bool {
    println!("Cliente");
    println!("Nome: {}", name);
    println!("CPF: {}", cpf);
    true
}

pub fn update_client(id: u32, name: String, cpf: String) -> bool {
    println!("Cliente {}", id);
    println!("Nome: {}", name);
    println!("CPF: {}", cpf);
    true
}

pub fn delete_client(id: u32) -> bool {
    println!("Excluindo o Cliente {}", id);
    true
}
