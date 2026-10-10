struct Pedido {
    nome_cliente: String,
    valor: f64,
    entregue: bool,
}

fn main() {
    let pedidos = vec![
        Pedido {
            nome_cliente: String::from("Cliente 1"),
            valor: 150.0,
            entregue: true,
        },
        Pedido {
            nome_cliente: String::from("Cliente 2"),
            valor: 250.0,
            entregue: false,
        },
        Pedido {
            nome_cliente: String::from("Cliente 3"),
            valor: 100.0,
            entregue: true,
        },
    ];

    let clientes_atendidos = pedidos
        .iter()
        .filter(|pedido| pedido.entregue)
        .map(|pedido| &pedido.nome_cliente);

    let valor_total: f64 = pedidos
        .iter()
        .filter(|pedido| pedido.entregue)
        .map(|pedido| pedido.valor)
        .sum();

    println!("Clientes Atendidos");

    for cliente in clientes_atendidos {
        println!("{}", cliente);
    }

    println!("Valor total dos pedidos entregues: {:.2}", valor_total);

    /*
        Métodos em Programação Funcional utilizados:
        iter() -> Cria um iterador sobre a coleção
        filter(|pedido| pedido.entregue) -> Filtra os clientes entregues
        map(|pedido| pedido.valor) -> Mapeia os itens retornando um array de f64
        sum() -> Soma os dados retornados pelo array
    */
}
