use anyhow::Result;
use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
    net::TcpListener,
    sync::broadcast::{Receiver, Sender},
};

#[tokio::main]

async fn main() -> Result<()> {
    let addr = "0.0.0.0:8080";
    let listener = TcpListener::bind(addr).await?;
    let (tx, _rx): (Sender<String>, Receiver<String>) = tokio::sync::broadcast::channel(30);

    loop {
        let tx2 = tx.clone();
        let mut rx2 = tx2.subscribe();

        let (mut stream, _sock_addr) = listener.accept().await?;
        println!("INFO: new client connected successfully");

        tokio::spawn(async move {
            let (s_reader, mut s_writer) = stream.split();
            let mut stream_buff_reader = BufReader::new(s_reader);
            loop {
                let mut client_inp = String::new();
                client_inp.clear();
                stream_buff_reader.read_line(&mut client_inp).await.unwrap();
                s_writer.write(client_inp.as_bytes()).await.unwrap();
                tx2.send(client_inp).unwrap();
            }
        });

        tokio::spawn(async move {
            match rx2.recv().await {
                Ok(message) => {
                    println!("Receieved data {} over threads", message);
                }
                Err(err) => {
                    println!("ERR occured");
                }
            };
        });
    }
}
