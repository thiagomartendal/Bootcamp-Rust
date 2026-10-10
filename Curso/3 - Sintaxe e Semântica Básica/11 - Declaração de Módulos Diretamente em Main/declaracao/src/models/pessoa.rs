use crate::enums::tipo::Tipo;

/*
    Também é possívle:

    use crate::enums::*;
    tipo::Tipo; // Acessando a enum
*/

pub struct Pessoa {
    pub nome: String,
    pub documento: String,
    pub tipo: Tipo,
}

impl Pessoa {
    pub fn new(nome: &str, documento: &str, tipo: Tipo) -> Pessoa {
        Pessoa {
            nome: nome.to_string(),
            documento: documento.to_string(),
            tipo: tipo,
        }
    }

    pub fn show(&self) {
        println!("Nome: {}", self.nome);
        println!("Documento: {}", self.documento);
        println!("Pessoa: {}", self.tipo_string());
    }

    fn tipo_string(&self) -> &str {
        match self.tipo {
            Tipo::Fisica => "Física",
            Tipo::Juridica => "Jurídica",
        }
    }
}
