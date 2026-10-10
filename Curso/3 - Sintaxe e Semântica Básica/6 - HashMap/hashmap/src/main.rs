use std::collections::HashMap;

struct Pessoa {
    nome: String,
    idade: u32,
    cidade: String
}

fn main() {
    let mut dados: HashMap<String, i32> = HashMap::new();

    dados.insert(String::from("Valor 1"), 10);
    dados.insert(String::from("Valor 2"), 50);

    let valor: Option<&i32> = dados.get(&String::from("Valor 1"));

    if let Some(v) = valor {
        println!("{}", v);
    } else {
        println!("Chave não encontrada");
    }

    // Resgata o valor diretamente, sem a checagem manual
    println!("{}", valor.unwrap());

    // Atribuição direta a variável
    let v: &i32 = dados.get(&String::from("Valor 2")).unwrap();
    println!("{}", v);

    // Desreferenciando o ponteiro
    let v2: i32 = *dados.get(&String::from("Valor 1")).unwrap();
    let v3: i32 = *dados.get(&String::from("Valor 2")).unwrap();
    println!("{} - {}", v2, v3);

    // Iterando HashMap
    for (k, v) in &dados {
        println!("{}: {}", k, v);
    }
    
    for (k, _) in &dados {
        println!("Chave: {}", k);
    }

    for (_, v) in &dados {
        println!("Valor: {}", v);
    }

    // entry verifica se a chave possui valor
    // caso não possua um valor definido, or_insert insere um valor nesta chave
    let v4 = dados.entry("Valor 3".to_string()).or_insert(30);
    println!("{}", v4);

    // Exibindo a integralidade do HashMap
    println!("{:?}", dados);

    // Iniciando um HashMap por construtor
    let valores: HashMap<&str, i32> = [
        ("Valor 1", 10),
        ("Valor 2", 20),
    ].into(); // HashMap imutável (sem a palavra-chave mut)

    let valores2: HashMap<&str, i32> = HashMap::from([
        ("Valor 1", 10),
        ("Valor 2", 20),
        ("Valor 3", 30)
    ]);

    println!("{:?}", valores);
    println!("{:?}", valores2);

    // Converção de Struct para HashMap
    let pessoa = Pessoa {
        nome: String::from("Pessoa"),
        idade: 30,
        cidade: String::from("Cidade Atual")
    };

    let mut propriedades_pessoa: HashMap<String, String> = [
        (String::from("nome"), pessoa.nome),
        (String::from("idade"), pessoa.idade.to_string()),
        (String::from("cidade"), pessoa.cidade)
    ].into();

    propriedades_pessoa.insert(String::from("ocupação"), String::from("Estudante"));

    println!("{:?}", propriedades_pessoa);
}