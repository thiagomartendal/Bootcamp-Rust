use crate::controles::acao_cliente;
use crate::modelos::cliente::Cliente;
use crate::viesoes::{ler, menu, operacoes_basicas};

fn executar_menu(clientes: &mut Vec<Cliente>) {
    loop {
        operacoes_basicas::limpar_tela();

        let opcao = menu::menu();

        match opcao {
            1 => {
                operacoes_basicas::limpar_tela();
                acao_cliente::cadastrar_cliente(clientes);
            }
            2 => {
                operacoes_basicas::limpar_tela();
                acao_cliente::alterar_cliente(clientes);
            }
            3 => {
                operacoes_basicas::limpar_tela();
                acao_cliente::excluir_cliente(clientes);
            }
            4 => {
                operacoes_basicas::limpar_tela();
                acao_cliente::exibir_clientes(clientes);
                println!("Pressione enter para continuar.");
                ler::entrada_dados();
            }
            0 => {
                println!("Encerrando aplicação.");
                break;
            }
            _ => println!("Opção invália"),
        }

        operacoes_basicas::esperar(2);
    }
}

pub fn controle() {
    let mut clientes: Vec<Cliente> = Vec::new();
    executar_menu(&mut clientes);
}
