mod enums;
mod models;

use crate::enums::*; // Inclui todos os módulos de enums
use crate::models::pessoa::Pessoa; // Inclui especificamente o módulo pessoa de models
use modulos_separados::fim;

fn main() {
    let pessoa1 = Pessoa::new("Pessoa 1", "123.456.789-00", tipo::Tipo::Fisica);
    let pessoa2 = Pessoa::new("Pessoa 2", "123.456.78900-00", tipo::Tipo::Juridica);

    pessoa1.show();
    println!("{}", '-'.to_string().repeat(20));
    pessoa2.show();

    fim();
}
