use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let contador = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let contador_clonado = Arc::clone(&contador);
        let handle = thread::spawn(move || {
            // Bloqueia o mutex
            let mut num = contador_clonado.lock().unwrap();

            // Acesso exclusivo dentro do mutex
            *num += 1;
        });
        handles.push(handle);
    }

    // Espera todas as threads terminarem
    for handle in handles {
        handle.join().unwrap();
    }

    println!("Resultado: {}", *contador.lock().unwrap());
}
