use anyhow::Result;
use std::net::SocketAddr;
use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
    net::TcpListener,
};

#[tokio::main]

async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    let addr = "0.0.0.0:8080";
    let listener = TcpListener::bind(addr).await?;
    let (tx, _rx): (Sender<(String, SocketAddr)>, Receiver<(String, SocketAddr)>) =
                    tokio::sync::broadcast::channel(30);

    loop {
        let tx2 = tx.clone();
        let mut rx2 = tx.subscribe();

        let (mut stream, sock_addr) = listener.accept().await?;
        tracing::info!("INFO: new client connected successfully");

        tokio::spawn(async move {
            let (s_reader, mut s_writer) = stream.split();
            let mut stream_buff_reader = BufReader::new(s_reader);

            loop {
                let mut client_inp = String::new();
                tokio::select! {
                    _ = stream_buff_reader.read_line(&mut client_inp)=>{
                        tx2.send((client_inp.clone(), sock_addr)).unwrap();
                        client_inp.clear();
                    },

                    Ok((message, message_addr)) = rx2.recv() =>  {
                        if message_addr == sock_addr {
                            continue;
                        }
                        tracing::info!("Receieved {}", message);
                        s_writer.write_all(message.as_bytes()).await.unwrap();
                    }

                }
            }
        });
    }
}
