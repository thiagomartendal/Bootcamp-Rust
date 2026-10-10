use crate::modelos::cliente::Cliente;
use crate::viesoes::servico_cliente;

fn possui_clientes(clientes: &mut Vec<Cliente>) -> bool {
    if clientes.len() == 0 {
        println!("Nenhum cliente foi cadastrado até o momento.");
        return false;
    }
    true
}

pub fn cadastrar_cliente(clientes: &mut Vec<Cliente>) {
    println!("Cadastro de Cliente");

    let (nome, cpf, endereco) = servico_cliente::entrada_dados_cliente();
    let cliente = Cliente {
        id: clientes.len() + 1,
        nome: nome,
        cpf: cpf,
        endereco: endereco,
    };

    clientes.push(cliente);

    println!("Cliente cadastrado.")
}

pub fn alterar_cliente(clientes: &mut Vec<Cliente>) {
    if !possui_clientes(clientes) {
        return;
    }

    let id = servico_cliente::capturar_id();
    let mut contem_id = false;

    for cliente in clientes {
        if cliente.id == id {
            contem_id = true;
            let (nome, cpf, endereco) = servico_cliente::entrada_dados_cliente();
            cliente.nome = nome;
            cliente.cpf = cpf;
            cliente.endereco = endereco;
            break;
        }
    }

    if !contem_id {
        println!("O ID {} não está cadastrado.", id)
    } else {
        println!("Cliente alterado.")
    }
}

pub fn excluir_cliente(clientes: &mut Vec<Cliente>) {
    if !possui_clientes(clientes) {
        return;
    }

    let id = servico_cliente::capturar_id();
    let mut contem_id = false;
    let mut indice: isize = -1;

    // iter_mut: metódo para iterar com mutação de valores permitida
    for cliente in clientes.iter_mut() {
        if cliente.id == id {
            contem_id = true;
            indice += 1;
            break;
        }
    }

    if !contem_id {
        println!("O ID {} não está cadastrado.", id)
    } else {
        clientes.remove(indice as usize);
        println!("Cliente removido.")
    }
}

pub fn exibir_clientes(clientes: &mut Vec<Cliente>) {
    println!("Clientes Cadastrados");

    if !possui_clientes(clientes) {
        return;
    }

    println!("");

    for cliente in clientes {
        let dados: (usize, &String, &String, &String) =
            (cliente.id, &cliente.nome, &cliente.cpf, &cliente.endereco);
        servico_cliente::exibir_cliente(dados);
    }
}
