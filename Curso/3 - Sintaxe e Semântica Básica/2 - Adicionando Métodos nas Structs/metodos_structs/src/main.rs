// Definição da Struct
struct Cliente {
    id: u32,
    nome: String,
    cpf: String,
    valor_disponivel: f32
}

// Definição dos Métodos da Struct
impl Cliente {
    fn cpf_valido(&self) -> bool {
        if self.cpf.is_empty() {
            return false;
        }

        true
    }

    fn adiciona_sobrenome(&mut self) {
        self.nome += " Sobrenome";
    }

    fn aumento(&mut self, valor: f32) {
        self.valor_disponivel += valor;
    }
}

fn main() {
    let mut cliente = Cliente {
        id: 1,
        nome: String::from("Cliente 1"),
        cpf: String::from("123.456.789-00"),
        valor_disponivel: 5000.00
    };

    let checa_cpf = if cliente.cpf_valido() {
        "Sim"
    } else {
        "Não"
    };

    cliente.adiciona_sobrenome();
    cliente.aumento(276.55);

    println!("O cpf {} do cliente {} é válido: {}",
        cliente.cpf,
        cliente.nome,
        checa_cpf
    );

    println!("Valor disponível do cliente #{} é: {}", cliente.id, cliente.valor_disponivel)
}
