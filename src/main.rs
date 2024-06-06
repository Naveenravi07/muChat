use anyhow::Result;
use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
    net::TcpListener,
    sync::broadcast::{Receiver, Sender},
};

#[tokio::main]

async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    let addr = "0.0.0.0:8080";
    let listener = TcpListener::bind(addr).await?;
    let (tx, _rx): (Sender<String>, Receiver<String>) = tokio::sync::broadcast::channel(30);

    loop {
        let tx2 = tx.clone();
        let mut rx2 = tx.subscribe();

        let (mut stream, _sock_addr) = listener.accept().await?;
        tracing::info!("INFO: new client connected successfully");

        tokio::spawn(async move {
            let (s_reader, mut s_writer) = stream.split();
            let mut stream_buff_reader = BufReader::new(s_reader);
            loop {
                let mut client_inp = String::new();
                client_inp.clear();
                stream_buff_reader.read_line(&mut client_inp).await.unwrap();
                tx2.send(client_inp.clone()).unwrap();

                tracing::info!("Trying  to Receieve");
                while let Ok(message) = rx2.try_recv() {
                    tracing::info!("Receieved {}", message);
                    s_writer.write_all(message.as_bytes()).await.unwrap();
                }
                tracing::info!("Tried to Receieve");
            }
        });
    }
}
