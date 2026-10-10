use crate::schema::cliente;
use diesel::mysql::MysqlConnection;
use diesel::prelude::*;
use diesel::result::Error;

use crate::modelo::cliente::Cliente;
use crate::modelo::novo_cliente::NovoCliente;

pub fn cadastrar(conn: &MysqlConnection, nome_str: &str, telefone_str: &str) -> Result<(), Error> {
    let novo_cliente = NovoCliente {
        nome: nome_str.to_string(),
        telefone: telefone_str.to_string(),
    };

    diesel::insert_into(cliente::table)
        .values(&novo_cliente)
        .execute(conn)?;

    Ok(())
}

pub fn listar(conn: &MysqlConnection) -> Result<Vec<Cliente>, Error> {
    cliente::table.load::<Cliente>(conn)
}

pub fn atualizar(
    conn: &MysqlConnection,
    id: i32,
    novo_nome: &str,
    novo_telefone: &str,
) -> Result<(), Error> {
    diesel::update(cliente::table.find(id))
        .set((
            cliente::nome.eq(novo_nome),
            cliente::telefone.eq(novo_telefone),
        ))
        .execute(conn)?;

    Ok(())
}

pub fn excluir(conn: &MysqlConnection, id: i32) -> Result<(), Error> {
    diesel::delete(cliente::table.find(id)).execute(conn)?;

    Ok(())
}
