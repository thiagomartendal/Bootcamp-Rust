#[path = "./models/pessoa.rs"] // Inclui a localização do módulo diretamente
mod pessoa;

use pessoa::Pessoa;

fn main() {
    let pessoa = Pessoa::new("Pessoa", "123.456.789-00");

    pessoa.show();
}
