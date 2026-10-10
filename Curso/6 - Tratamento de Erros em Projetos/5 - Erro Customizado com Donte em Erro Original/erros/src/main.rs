use std::error::Error;
use std::fs::File;
use std::io::{self, Read};

#[derive(Debug)]
enum ErroDeTransacao {
    SaldoInsuficiente {
        saldo_atual: f64,
        tentativa_saque: f64,
    },
    ErroDeAutenticacao,
    ErroDeRede,
    ErroComFonte {
        mensagem: String,
        fonte: Box<dyn Error>,
    },
}

impl std::fmt::Display for ErroDeTransacao {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ErroDeTransacao::SaldoInsuficiente {
                saldo_atual,
                tentativa_saque,
            } => {
                write!(
                    f,
                    "Saldo insuficiente: saldo atual R${}, tentativa de saque R${}.",
                    saldo_atual, tentativa_saque
                )
            }
            ErroDeTransacao::ErroDeAutenticacao => {
                write!(f, "Erro de autenticação: usuário não pode ser autenticado.")
            }
            ErroDeTransacao::ErroDeRede => {
                write!(f, "Erro de rede: não foi possível conectar ao servidor.")
            }
            ErroDeTransacao::ErroComFonte { mensagem, fonte } => {
                write!(f, "{}: {}", mensagem, fonte)
            }
        }
    }
}

// Implementando o trait Error
impl std::error::Error for ErroDeTransacao {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            ErroDeTransacao::ErroComFonte { fonte, .. } => Some(fonte.as_ref()),
            _ => None,
        }
    }
}

fn processar_transacao(
    valor: f64,
    autenticado: bool,
    falha_na_rede: bool,
    com_fonte: bool,
) -> Result<(), ErroDeTransacao> {
    // Simula um erro de transferência
    let saldo = 100.0; // Valor fictício
    if valor > saldo {
        return Err(ErroDeTransacao::SaldoInsuficiente {
            saldo_atual: saldo,
            tentativa_saque: valor,
        });
    }

    // Simula um erro de autenticação
    if !autenticado {
        return Err(ErroDeTransacao::ErroDeAutenticacao);
    }

    // Simula um erro de rede
    if !falha_na_rede {
        return Err(ErroDeTransacao::ErroDeRede);
    }

    if com_fonte {
        let ler_arquivo: Result<String, io::Error> = ler_contaudo_arquivo("arquivo.txt");
        match ler_arquivo {
            Ok(_) => {}
            Err(erro) => {
                return Err(ErroDeTransacao::ErroComFonte {
                    mensagem: "Erro ao abrir o arquivo".to_string(),
                    fonte: Box::new(erro),
                });
            }
        }
    }

    // Caso não tenham ocorrido erros, a função retorna um Ok,
    // indicando uma transação bem-sucedida
    Ok(())
}

fn ler_contaudo_arquivo(nome_arquivo: &str) -> Result<String, io::Error> {
    let mut f = File::open(nome_arquivo)?; // Se falhar retorna o erro
    let mut conteudo = String::new();
    f.read_to_string(&mut conteudo)?; // Se falhar retorna o erro
    Ok(conteudo) // Retorna o conteúdo do arquivo em caso de sucesso
}

fn main() {
    match processar_transacao(50.0, true, true, true) {
        Ok(_) => println!("Transação aprovada."),
        Err(e) => {
            println!("Falha ao processar a transação: {}", e);

            // Acessando a causa do erro
            if let Some(source) = e.source() {
                println!("Causa do erro: {}", source);
            }
        }
    }
}
