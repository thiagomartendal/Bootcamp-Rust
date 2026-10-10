create_struct_and_metadata! {
    "cliente" => Cliente {
        id: u64, "autoincrement",
        nome: String, "varchar(100)",
        telefone: String, "varchar(15)",
    }
}
