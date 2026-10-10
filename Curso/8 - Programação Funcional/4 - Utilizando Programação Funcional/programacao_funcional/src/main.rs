fn descontos_versao1() {
    let calcular_salario_liquido = |salario_bruto: f64| {
        let desconto_plano_saude = |salario: f64| salario * 0.10;
        let desconto_plano_dentario = |salario: f64| salario * 0.05;
        let desconto_vale_refeicao = |salario: f64| salario * 0.03;

        let desconto_saude = desconto_plano_saude(salario_bruto);
        let desconto_dentario = desconto_plano_dentario(salario_bruto);
        let desconto_refeicao = desconto_vale_refeicao(salario_bruto);

        salario_bruto - desconto_saude - desconto_dentario - desconto_refeicao
    };

    let salario_bruto = 5000.0;
    let salario_liquido = calcular_salario_liquido(salario_bruto);

    println!("Salário líquido: {:.2}", salario_liquido);
}

fn descontos_versao2() {
    fn aplicar_descontos(salario: f64, descontos: Vec<fn(f64) -> f64>) -> f64 {
        let total_descontos = descontos
            .iter()
            .fold(0.0, |valor, funcao_closure| valor + funcao_closure(salario));
        salario - total_descontos
    }

    let salario_bruto = 5000.0;
    let salario_liquido = aplicar_descontos(
        salario_bruto,
        vec![
            // Vetor de descontos
            |salario: f64| salario * 0.10, // Desconto 1
            |salario: f64| salario * 0.05, // Desconto 2
            |salario: f64| salario * 0.03, // Desconto 3
        ],
    );

    println!("Salário líquido: {:.2}", salario_liquido);
}

fn main() {
    descontos_versao1();
    println!("------------------");
    descontos_versao2();
}
