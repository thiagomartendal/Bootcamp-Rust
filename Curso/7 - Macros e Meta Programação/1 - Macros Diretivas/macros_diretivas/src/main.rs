// Define a macro
macro_rules! diz_ola {
    () => {
        println!("Olá Mundo!");
    };
}

// Definindo um trait com métodos para forçar a implementação
trait ExemploTrait {
    fn metodo_exemplo(&self);
}

// Criando uma macro para implementar automaticamente o trait para qualquer struct
macro_rules! implementa_trait {
    ($t:ty) => {
        impl ExemploTrait for $t {
            fn metodo_exemplo(&self) {
                println!("Método de exemplo chamado para {:?}", self);
            }
        }
    };
}

// Definindo structs
#[derive(Debug)]
struct MinhaStruct;

#[derive(Debug)]
struct MinhaStruct2;

// Usando a macro para aplicar a implementação do trait para structs
implementa_trait!(MinhaStruct);
implementa_trait!(MinhaStruct2);

fn main() {
    diz_ola!();

    println!("-----------------");

    let minha_instancia = MinhaStruct;
    let minha_instancia2 = MinhaStruct2;

    minha_instancia.metodo_exemplo();
    minha_instancia2.metodo_exemplo();
}
