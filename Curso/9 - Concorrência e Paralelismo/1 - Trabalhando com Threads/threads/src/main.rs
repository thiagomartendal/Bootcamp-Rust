use std::thread;
use std::time::Duration;

fn main() {
    // Thread que imprime números
    let num_thread = thread::spawn(|| {
        for i in 1..=5 {
            println!("{}", i);
            thread::sleep(Duration::from_millis(500));
        }
    });

    // Thread que imprime letras
    let letter_thread = thread::spawn(|| {
        for letter in 'a'..='e' {
            println!("{}", letter);
            thread::sleep(Duration::from_millis(200));
        }
    });

    // Espera as threads terminarem
    num_thread.join().unwrap();
    letter_thread.join().unwrap();
}
