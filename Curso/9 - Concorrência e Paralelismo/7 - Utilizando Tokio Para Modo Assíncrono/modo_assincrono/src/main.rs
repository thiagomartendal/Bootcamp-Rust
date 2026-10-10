/*
    Testes:
    curl localhost:8080 // Acessa e libera
    telnet localhost 8080 // Bloqueia a requisição
    curl localhost:8080 // Trava pois telnet bloqueou
*/

use tokio::io::{self, AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080").await?;

    println!("Servidor executando em 127.0.0.1:8080");

    loop {
        // Aceita a conexão
        let (mut socket, _) = listener.accept().await?;

        println!("Cliente conectado");

        // tokio::spawn lida com a conexão em uma nova tarefa assíncrona
        tokio::spawn(async move {
            let mut buf = vec![0; 1024]; // Buffer de armazenamento dos dados recebidos

            // Loop para ler dados do socket e escrever de volta
            loop {
                match socket.read(&mut buf).await {
                    Ok(n) if n == 0 => {
                        println!("Conecão encerrada pelo cliente");
                        return;
                    }
                    Ok(n) => {
                        if socket.write_all(&buf[..n]).await.is_err() {
                            println!("Erro ao enviar dados de volta");
                            return;
                        }
                    }
                    Err(e) => {
                        println!("Erro ao ler do socket: {:?}", e);
                        return;
                    }
                }
            }
        });
    }
}
