use crate::config::cnn;
use crate::modelo::cliente::Cliente;
use mysql::prelude::Queryable;
use mysql::*;

pub fn cadastrar(nome: &str, telefone: &str) -> Result<()> {
    let mut conn = cnn::obter_conexao()?;

    conn.exec_drop(
        r"insert into cliente (nome, telefone) VALUES (:nome, :telefone)",
        params! {
            "nome" => nome,
            "telefone" => telefone
        },
    )?;

    Ok(())
}

pub fn listar() -> Result<Vec<Cliente>> {
    let mut conn = cnn::obter_conexao()?;

    let clientes: Vec<Cliente> = conn.query_map(
        "select id, nome, telefone from cliente",
        |(id, nome, telefone)| Cliente {
            id: id,
            nome,
            telefone,
        },
    )?;

    Ok(clientes)
}

pub fn atualizar(id: u64, novo_nome: &str, novo_telefone: &str) -> Result<()> {
    let mut conn = cnn::obter_conexao()?;

    conn.exec_drop(
        r"update cliente set nome=:nome, telefone=:telefone where id=:id",
        params! {"id" => id, "nome" => novo_nome, "telefone" => novo_telefone},
    )?;

    Ok(())
}

pub fn excluir(id: u64) -> Result<()> {
    let mut conn = cnn::obter_conexao()?;

    conn.exec_drop("delete from cliente where id=:id", params! {"id" => id})?;

    Ok(())
}
