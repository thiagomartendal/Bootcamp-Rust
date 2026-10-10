use crate::modelo::cliente::Cliente;
use crate::repositorio::generico_repositorio;
use crate::visao::exibir;
use crate::visao::menu;

pub fn controle() {
    loop {
        let opcao = menu::menu();

        match opcao {
            1 => {
                let (nome, telefone) = exibir::cadastrar_cliente();
                let cliente = Cliente {
                    id: 0,
                    nome: nome.trim().to_string(),
                    telefone: telefone.trim().to_string(),
                };
                generico_repositorio::inserir(&cliente).expect("Falha ao cadastrar o cliente.");
                exibir::msg_sucesso("Cadastro concluído com sucesso.");
            }
            2 => match generico_repositorio::listar() {
                Ok(clientes) => {
                    if let Err(e) = exibir::mostrar_clientes(clientes) {
                        println!("Erro ao listar clientes: {}", e);
                    }
                }
                Err(e) => println!("{}", e),
            },
            3 => match exibir::atualizar_cliente() {
                Ok((id, nome, telefone)) => {
                    let cliente = Cliente {
                        id: id,
                        nome: nome.trim().to_string(),
                        telefone: telefone.trim().to_string(),
                    };
                    generico_repositorio::atualizar(id, &cliente)
                        .expect("Falha ao atualizar cliente.");
                    exibir::msg_sucesso("Cliente atualizado com sucesso.");
                }
                Err(e) => println!("Erro ao atualizar cliente: {}", e),
            },
            4 => match exibir::excluir_cliente() {
                Ok(id) => {
                    generico_repositorio::excluir::<Cliente>(id)
                        .expect("Falha ao excluir cliente.");
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
