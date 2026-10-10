use macro_derive::MeuTrait;
use macro_derive_traits::MeuTrait;

#[derive(MeuTrait)]
struct MinhaStruct {}

fn main() {
    let minha_instancia = MinhaStruct {};

    println!("{}", minha_instancia.minha_funcao());
}
