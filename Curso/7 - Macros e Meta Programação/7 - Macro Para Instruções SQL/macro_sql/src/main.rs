#![allow(warnings)]

macro_rules! crete_struct_and_metadata {
    ($name:ident {$($field_name:ident: $field_type:ty, $metadata:expr),* $(,)?}) => {
        struct $name {
            $(pub $field_name: $field_type,)*
        }

        impl $name {
            pub fn metadata() -> Vec<(&'static str, &'static str, &'static str)> {
                vec![
                    $((stringify!($field_name), stringify!($field_type), $metadata),)*
                ]
            }

            pub fn generate_sql_create_table() -> String {
                let table_name = stringify!($name).to_lowercase();
                let columns = Self::metadata().iter().map(|(field, _type, meta)| {
                    let sql_type = match *meta {
                        "autoincrement" => "INT AUTO_INCREMENT PRIMARY KEY",
                        _ => meta,
                    };
                    format!("{} {}", field, sql_type)
                }).collect::<Vec<String>>().join(",\n   ");

                format!("CREATE TABLE {} (\n    {}\n);", table_name, columns)
            }
        }
    };
}

crete_struct_and_metadata! {
    Cliente {
        id: i32, "autoincrement",
        nome: String, "varchar(100)",
        telefone: String, "varchar(15)"
    }
}

crete_struct_and_metadata! {
    Pedido {
        id: i32, "autoincrement",
        id_cliente: i32, "int",
        valor_total: f32, "float"
    }
}

fn main() {
    for (field, field_type, metadata) in Cliente::metadata() {
        println!(
            "Campo: {}, Tipo: {}, Metadado: {}",
            field, field_type, metadata
        );
    }

    let tabela_cliente = Cliente::generate_sql_create_table();
    let tabela_pedido = Pedido::generate_sql_create_table();

    println!("\n{}\n", tabela_cliente);
    println!("{}", tabela_pedido);
}
