use crate::modelo::cliente::Cliente;
use rusqlite::{Connection, Result, params};

pub fn cadastrar(conn: &Connection, nome: &str, telefone: &str) -> Result<()> {
    conn.execute(
        "insert into cliente (nome, telefone) values (?1, ?2)",
        params![nome, telefone],
    )?;

    Ok(())
}

pub fn listar(conn: &Connection) -> Result<Vec<Cliente>> {
    let mut stmt = conn.prepare("select * from cliente")?;
    let clientes_iter = stmt.query_map(params![], |row| {
        Ok(Cliente {
            id: row.get(0)?,
            nome: row.get(1)?,
            telefone: row.get(2)?,
        })
    })?;

    clientes_iter.collect()
}

pub fn atualizar(conn: &Connection, id: i32, novo_nome: &str, novo_telefone: &str) -> Result<()> {
    conn.execute(
        "update cliente set nome = ?1, telefone = ?2 where id = ?3",
        params![novo_nome, novo_telefone, id],
    )?;

    Ok(())
}

pub fn excluir(conn: &Connection, id: i32) -> Result<()> {
    conn.execute("delete from cliente where id = ?1", params![id])?;

    Ok(())
}
