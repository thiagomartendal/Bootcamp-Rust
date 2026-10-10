use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    // Cria um canal
    let (tx, rx) = mpsc::channel();
    // tx -> envia o resultado | rx -> recebe o resultado

    // Cria uma nova thread
    thread::spawn(move || {
        let msg = String::from("Olá Mundo!");

        thread::sleep(Duration::from_millis(1000));

        // Envia a mensagem pelo canal
        tx.send(msg).unwrap();
    });

    // Recebe a mensagem na thread principal
    let received = rx.recv().unwrap(); // Espera até a thread ser resolvida
    println!("Mensagem: {}", received);
}
