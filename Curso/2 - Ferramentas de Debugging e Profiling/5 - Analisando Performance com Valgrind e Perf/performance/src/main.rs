/*
    Valgrind -> Ferramenta para detecção de vazamentos de memória

    Perf -> Ferramenta para análise de desempenho no nível do sistema

    Instalação Valgrind:

        sudo apt install valgrind
    
    Instalação Perf:

        sudo apt-get install linux-tools-common linux-tools-generic linux-tools-$(uname -r)

    Utilizando o Valgrind e o Perf:

    1) Fazer o build da aplicação:

        cargo build
    
    2) Executando o valgrind e o perf no binário de debug:

        valgrind target/debug/performance -> avalia a memória

        perf record target/debug/performance -> avalia a performance
        -> sudo perf record target/debug/performance
        perf report -> Fornece o relatório do Perf
*/

fn main() {
    let x: String = String::from("Depuração");
    let x_modificada = alterando(x);
    println!("{}", x_modificada);
}

fn alterando(mut str: String) -> String {
    str += " - alterando";
    str
}