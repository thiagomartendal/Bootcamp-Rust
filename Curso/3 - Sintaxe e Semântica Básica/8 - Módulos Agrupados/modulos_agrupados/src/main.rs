mod enums;
mod models;

use enums::*;
use models::Pessoa;

fn main() {
    let pessoa1 = Pessoa::new("Pessoa 1", "123.456.789-00", Tipo::Fisica);
    let pessoa2 = Pessoa::new("Pessoa 2", "123.456.78900-00", Tipo::Juridica);

    pessoa1.show();
    println!("{}", '-'.to_string().repeat(20));
    pessoa2.show();
}
