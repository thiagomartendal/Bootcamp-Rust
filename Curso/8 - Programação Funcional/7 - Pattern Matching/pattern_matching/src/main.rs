enum Semafaro {
    Verde,
    Amarelo,
    Vermelho,
}

fn acao(semafaro: Semafaro) {
    match semafaro {
        Semafaro::Verde => println!("Siga"),
        Semafaro::Amarelo => println!("Atenção"),
        Semafaro::Vermelho => println!("Pare"),
    }
}

fn main() {
    acao(Semafaro::Verde);
    acao(Semafaro::Amarelo);
    acao(Semafaro::Vermelho);
}
