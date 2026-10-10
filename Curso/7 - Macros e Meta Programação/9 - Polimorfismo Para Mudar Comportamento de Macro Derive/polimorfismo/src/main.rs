use macro_derive::MeuTrait;
use macro_derive_traits::MeuTrait;

#[derive(MeuTrait)]
struct TipoPadrao {}

struct TipoCustomizado {}

// Uma segunda implementação de minha_funcao
impl MeuTrait for TipoCustomizado {
    fn minha_funcao(&self) -> String {
        String::from("Implementação customizada de MeuTrait")
    }
}

fn imprime_mensagem<T: MeuTrait>(item: &T) {
    println!("{}", item.minha_funcao());
}

fn main() {
    let padrao = TipoPadrao {};
    let customizado = TipoCustomizado {};

    imprime_mensagem(&padrao);
    imprime_mensagem(&customizado);
}
