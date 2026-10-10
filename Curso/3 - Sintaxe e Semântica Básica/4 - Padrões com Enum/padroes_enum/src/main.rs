enum Comando {
    Iniciar(String),
    Parar
}

fn executar_comando(comando: Comando) {
    match comando {
        Comando::Iniciar(mensagem) => println!("{}", mensagem),
        Comando::Parar => println!("Parando")
    }
}

fn exemplo1() {
    let comando_iniciar = Comando::Iniciar(String::from("Motor elétrico"));
    let comando_parar = Comando::Parar;

    executar_comando(comando_iniciar);
    executar_comando(comando_parar);
}

enum Tipo {
    Fisica {mascara_cpf: String, cpf_governo: bool},
    Juridica {mascara_cnpj: String, cnpj_governo: bool}
}

struct Pessoa {
    nome: String,
    documento: String,
    tipo: Tipo
}

impl Pessoa {
    fn registro(&self) {
        match &self.tipo {
            Tipo::Fisica {mascara_cpf, cpf_governo} => {
                println!("Pessoa física: {} - {}", self.nome, self.documento);
                println!("Mácara CPF: {} - Governo: {}", mascara_cpf, cpf_governo);
            },
            Tipo::Juridica {mascara_cnpj, cnpj_governo} => {
                println!("Pessoa jurídica: {} - {}", self.nome, self.documento);
                println!("Mácara CNPJ: {} - Governo: {}", mascara_cnpj, cnpj_governo);
            }
        }
    }
}

fn exemplo2() {
    let pessoa1 = Pessoa {
        nome: String::from("Pessoa 1"),
        documento: String::from("123.456.789-00"),
        tipo: Tipo::Fisica {
            mascara_cpf: String::from("999.999.999-99"),
            cpf_governo: true
        }
    };

    let pessoa2 = Pessoa {
        nome: String::from("Pessoa 2"),
        documento: String::from("12345.6789000.000.000"),
        tipo: Tipo::Juridica {
            mascara_cnpj: String::from("99999.9999999.999.999"),
            cnpj_governo: true
        }
    };

    pessoa1.registro();
    pessoa2.registro();
}

enum Tipo2 {
    Fisica(String),
    Juridica(String)
}

struct Pessoa2 {
    nome: String,
    documento: String,
    tipo: Tipo2
}

impl Pessoa2 {
    fn registro(&self) {
        match &self.tipo {
            Tipo2::Fisica(descricao) => {
                println!("Pessoa física: {} - {} - {}", &self.nome, &self.documento, descricao);
            },
            Tipo2::Juridica(descricao) => {
                println!("Pessoa jurídica: {} - {} - {}", &self.nome, &self.documento, descricao);
            }
        }
    }
}

fn exemplo3() {
    let pessoa1 = Pessoa2 {
        nome: String::from("Pessoa 1"),
        documento: String::from("123.456.789-00"),
        tipo: Tipo2::Fisica(String::from("Funcionário"))
    };

    let pessoa2 = Pessoa2 {
        nome: String::from("Pessoa 2"),
        documento: String::from("12345.6789000.000.000"),
        tipo: Tipo2::Juridica(String::from("Empresa"))
    };

    pessoa1.registro();
    pessoa2.registro();
}

fn main() {
    exemplo1();
    println!("-----------------------");
    exemplo2();
    println!("-----------------------");
    exemplo3();
}