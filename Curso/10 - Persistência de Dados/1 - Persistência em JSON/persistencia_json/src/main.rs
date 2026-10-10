mod controle;
mod modelo;
mod servico;
mod visao;

use crate::controle::controle_aplicacao;

fn main() {
    controle_aplicacao::controle();
}
