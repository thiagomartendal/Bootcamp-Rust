use crate::config;
use crate::repositorio::cliente_repositorio;
use crate::visao::exibir;
use crate::visao::menu;

pub fn controle() {
    let conn = config::cnn::obter_conexao();

    loop {
        let opcao = menu::menu();

        match opcao {
            1 => {
                let (nome, telefone) = exibir::cadastrar_cliente();
                cliente_repositorio::cadastrar(&conn, &nome.trim(), &telefone.trim())
                    .expect("Falha ao cadastrar o cliente.");
                exibir::msg_sucesso("Cadastro concluído com sucesso.");
            }
            2 => match cliente_repositorio::listar(&conn) {
                Ok(clientes) => {
                    if let Err(e) = exibir::mostrar_clientes(clientes) {
                        println!("Erro ao listar clientes: {}", e);
                    }
                }
                Err(e) => println!("{}", e),
            },
            3 => match exibir::atualizar_cliente() {
                Ok((id, nome, telefone)) => {
                    cliente_repositorio::atualizar(&conn, id, &nome.trim(), &telefone.trim())
                        .expect("Falha ao atualizar cliente.");
                    exibir::msg_sucesso("Cliente atualizado com sucesso.");
                }
                Err(e) => println!("Erro ao atualizar cliente: {}", e),
            },
            4 => match exibir::excluir_cliente() {
                Ok(id) => {
                    cliente_repositorio::excluir(&conn, id).expect("Falha ao excluir cliente.");
                    exibir::msg_sucesso("Cliente removido com sucesso.");
                }
                Err(e) => println!("Erro ao excluir cliente: {}", e),
            },
            5 => {
                println!("Encerrando aplicação.");
                break;
            }
            _ => exibir::opcao_invalida(),
        }
    }
}
