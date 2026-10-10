use std::sync::mpsc;
use std::thread;
use std::time::Duration;

struct Cliente {
    id: u32,
    nome: String,
}

struct Produto {
    id: u32,
    nome: String,
}

fn main() {
    let (tx_clientes, rx_clientes) = mpsc::channel();
    let (tx_produtos, rx_produtos) = mpsc::channel();

    // Thread para criar clientes
    let cliente_thread = thread::spawn(move || {
        let clientes = vec![
            Cliente {
                id: 1,
                nome: "Cliente 1".into(),
            },
            Cliente {
                id: 2,
                nome: "Cliente 2".into(),
            },
        ];

        for cliente in clientes {
            println!("Enviando cliente {}", cliente.nome);
            tx_clientes.send(cliente).unwrap();
            thread::sleep(Duration::from_millis(100));
        }
    });

    // Thread para criar produtos
    let produto_thread = thread::spawn(move || {
        let produtos = vec![
            Produto {
                id: 1,
                nome: "Produto 1".into(),
            },
            Produto {
                id: 2,
                nome: "Produto 2".into(),
            },
        ];

        for produto in produtos {
            println!("Enviando produto {}", produto.nome);
            tx_produtos.send(produto).unwrap();
            thread::sleep(Duration::from_millis(100));
        }
    });

    // Espera as thread terminarem para garantir que os
    // dados foram enviados
    cliente_thread.join().unwrap();
    produto_thread.join().unwrap();

    // Coleta os clientes e produtos após o fim das threads
    let clientes: Vec<Cliente> = rx_clientes.try_iter().collect();
    let produtos: Vec<Produto> = rx_produtos.try_iter().collect();

    if !clientes.is_empty() && !produtos.is_empty() {
        for i in 0..2 {
            println!(
                "Pedido criado para o cliente de ID {} com o produto de ID {}",
                clientes[i].id, produtos[i].id
            );
        }
    } else {
        println!("Não foram recebidos clientes ou produtos.")
    }
}
