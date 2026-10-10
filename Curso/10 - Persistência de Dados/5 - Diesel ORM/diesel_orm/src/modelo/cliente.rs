use crate::schema::cliente;

#[derive(Queryable, Insertable, AsChangeset)]
#[table_name = "cliente"]
pub struct Cliente {
    pub id: i32,
    pub nome: String,
    pub telefone: String,
}
