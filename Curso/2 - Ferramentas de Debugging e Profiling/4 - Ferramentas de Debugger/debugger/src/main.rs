/*
    Instalação do gdb:

    sudo apt install gdb

    Instalação do lldb:

    sudo apt install lldb

    Para fazer o debug da aplicação:

    1) Fazer o build em modo debug:

        cargo build
    
    2) Executar o lldb com o binário gerado:

        lldb target/debug/debugger

        Também pode-se usar o gdb:

        gdb target/debug/debugger
    
    Comandos básicos do lldb e do gdb:

    b (break) <arquivo.rs>:<linha> -> Adiciona um
    breakpoint em uma linha do arquivo

    r (run): Inicia a execução do programa dentro do LLVM

    n (next): Executa a próxima instrução (sem entrar
    em funções)

    s (step): Executa a próxima instrução, entrando em
    funções

    p (print) <variável>: Imprime o valor de uma variável.

    l (list): Mostra o código fonto do ponto atual

    c (continue): Continua a execução até o próximo ponto
    de interrupção

    exit: sair

    q (quit): sair com um pedido de confirmação
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