trait Pessoa {
    fn registro(&self);
}

struct PessoaFisica {
    id: i32,
    nome: String,
    cpf: String,
}

struct PessoaJuridica {
    id: i32,
    nome: String,
    cnpj: String,
}

impl Pessoa for PessoaFisica {
    fn registro(&self) {
        println!("ID: {}", self.id);
        println!("Nome: {}", self.nome);
        println!("CPF: {}", self.cpf);
    }
}

impl Pessoa for PessoaJuridica {
    fn registro(&self) {
        println!("ID: {}", self.id);
        println!("Nome: {}", self.nome);
        println!("CNPJ: {}", self.cnpj);
    }
}

fn exibe_documento(pessoa: &dyn Pessoa) {
    pessoa.registro();
}

fn main() {
    let pessoa1 = PessoaFisica {
        id: 1,
        nome: String::from("Pessoa 1"),
        cpf: String::from("123.456.789-00"),
    };

    let pessoa2 = PessoaJuridica {
        id: 2,
        nome: String::from("Pessoa 2"),
        cnpj: String::from("12.345.678/9000-00"),
    };

    exibe_documento(&pessoa1);
    println!("--------------------");
    exibe_documento(&pessoa2);

    // pessoa1.registro();
    // println!("--------------------");
    // pessoa2.registro();
}
