struct CalculadoraSalario {
    total_descontos: f64,
    salario_bruto: f64,
}

impl CalculadoraSalario {
    fn new(salario_bruto: f64) -> Self {
        CalculadoraSalario {
            salario_bruto: salario_bruto,
            total_descontos: 0.0,
        }
    }

    fn desconto_plano_saude(mut self) -> Self {
        self.total_descontos += self.salario_bruto * 0.10;
        self
    }

    fn desconto_plano_dentario(mut self) -> Self {
        self.total_descontos += self.salario_bruto * 0.05;
        self
    }

    fn desconto_vale_refeicao(mut self) -> Self {
        self.total_descontos += self.salario_bruto * 0.03;
        self
    }

    fn valor(self) -> f64 {
        self.salario_bruto - self.total_descontos
    }
}

fn exemplo_struct() {
    let salario_bruto = 5000.0;
    let salario_liquido = CalculadoraSalario::new(salario_bruto)
        .desconto_plano_saude()
        .desconto_plano_dentario()
        .desconto_vale_refeicao()
        .valor();

    println!("Salário líquido:  {:.2}", salario_liquido);
}

fn desconto_plano_saude(salario: f64) -> f64 {
    salario * 0.10
}

fn desconto_plano_dentario(salario: f64) -> f64 {
    salario * 0.05
}

fn desconto_vale_refeicao(salario: f64) -> f64 {
    salario * 0.03
}

fn aplicar_descontos(salario: f64, descontos: Vec<fn(f64) -> f64>) -> f64 {
    let total_descontos = descontos
        .iter()
        .fold(0.0, |valor, funcao_closure| valor + funcao_closure(salario));
    salario - total_descontos
}

fn exemplo_funcoes_livres() {
    let salario_bruto = 5000.0;
    let salario_liquido = aplicar_descontos(
        salario_bruto,
        vec![
            desconto_plano_saude,
            desconto_plano_dentario,
            desconto_vale_refeicao,
        ],
    );

    println!("Salário líquido:  {:.2}", salario_liquido);
}

fn main() {
    exemplo_funcoes_livres();
    println!("-------------------");
    exemplo_struct();
}
