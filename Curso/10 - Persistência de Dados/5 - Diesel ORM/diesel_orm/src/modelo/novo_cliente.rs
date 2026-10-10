use crate::schema::cliente;

#[derive(Insertable)]
#[table_name = "cliente"]
pub struct NovoCliente {
    pub nome: String,
    pub telefone: String,
}
