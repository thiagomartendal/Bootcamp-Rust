// Definição de uma struct
#[derive(Debug)] // Utilizado para exibir inteiramente uma struct com {:?}
struct Funcionario {
    id: i32,
    nome: String,
    salario: f32
}

fn exibir_struct(funcionario: &Funcionario) {
    println!("ID: {}", funcionario.id);
    println!("Nome: {}", funcionario.nome);
    println!("Salário: {}", funcionario.salario);
}

fn main() {
    usando_struct();
    matriz();
    matriz_vec();
    matriz_struct();
}

fn usando_struct() {
    // Utilização da struct
    let funcionario1 = Funcionario {
        id: 1,
        nome: String::from("Funcionário 1"),
        salario: 1300.00
    };

    println!("{:?}", funcionario1);

    exibir_struct(&funcionario1);

    // Struct mutável
    let mut funcionario2 = Funcionario {
        id: 2,
        nome: String::from("Funcionário"),
        salario: 0.00
    };

    funcionario2.nome = "Funcionário 2".to_string();
    funcionario2.salario = 1550.75;

    exibir_struct(&funcionario2);

    // Builder de struct
    let mut funcionario3 = formar_funcionario(3, String::from("Funcionário 3"), 2400.00);
    funcionario3.salario += 0.55;

    exibir_struct(&funcionario3);

    // Preenchimento com spread-operator
    let funcionario4 = Funcionario {
        nome: String::from("Funcionário 4"), // Altera o nome
        ..funcionario3 // Reaproveita demais valores já definidos
    };

    exibir_struct(&funcionario4);

    // Struct vazia
    // let vazio = Funcionario {
    //     id: 0,
    //     nome: String::new(),
    //     salario: 0.0
    // };
}

fn formar_funcionario(id: i32, nome: String, salario: f32) -> Funcionario {
    Funcionario {
        id: id,
        nome: nome,
        salario: salario
    }
}

fn matriz() {
    // Representação por matriz
    let matriz: [[String; 3]; 2] = [
        [String::from("1"), String::from("Funcionário 1"), String::from("1200")],
        [String::from("2"), String::from("Funcionário 2"), String::from("700")]
    ];

    for funcionario in matriz {
        println!("{} - {} - {}", funcionario[0], funcionario[1], funcionario[2]);
    }
}

fn matriz_vec() {
    // Matriz de vec
    let matriz_vec: Vec<(i32, String, f32)> = vec![
        (1, String::from("Funcionário 1"), 1200.00),
        (2, String::from("Funcionário 2"), 700.00)
    ];

    for funcionario in matriz_vec {
        println!("{} - {} - {}", funcionario.0, funcionario.1, funcionario.2);
    }
}

fn matriz_struct() {
    // Matriz de struct
    let funcionarios: Vec<Funcionario> = vec![
        Funcionario {
            id: 1,
            nome: String::from("Funcionário 1"),
            salario: 1200.00
        },
        Funcionario {
            id: 2,
            nome: String::from("Funcionário 2"),
            salario: 700.00
        }
    ];

    for funcionario in funcionarios {
        println!("{} - {} - {}", funcionario.id, funcionario.nome, funcionario.salario);
    }
}