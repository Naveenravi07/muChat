extern crate colored;
use anyhow::Result;
use colored::Colorize;
use std::{
    collections::HashMap,
    net::SocketAddr,
    sync::{Arc, RwLock},
};
use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
    net::TcpListener,
    select,
    sync::broadcast::{Receiver, Sender},
};

#[tokio::main]

async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    let addr = "0.0.0.0:8080";
    let listener = TcpListener::bind(addr).await?;
    let (tx, _rx): (Sender<(String, SocketAddr)>, Receiver<(String, SocketAddr)>) =
        tokio::sync::broadcast::channel(30);
    let client_data: Arc<RwLock<HashMap<SocketAddr, String>>> =
        Arc::new(RwLock::new(HashMap::new()));

    loop {
        let tx2 = tx.clone();
        let mut rx2 = tx.subscribe();

        let (mut stream, sock_addr) = listener.accept().await?;
        tracing::info!("INFO: new client connected successfully");

        let client_data_clo = Arc::clone(&client_data);
        tokio::spawn(async move {
            let (s_reader, mut s_writer) = stream.split();
            let mut stream_buff_reader = BufReader::new(s_reader);
            let mut client_inp = String::new();

            s_writer
                .write("Please Enter your username".as_bytes())
                .await
                .unwrap();

            stream_buff_reader.read_line(&mut client_inp).await.unwrap();
            
                client_data_clo
                    .write()
                    .unwrap()
                    .insert(sock_addr, client_inp.trim().to_string());
           
            let welcome = format!("{} Joined the club",client_inp.trim());
            client_inp.clear();

            tx2.send((welcome, sock_addr)).unwrap();
            loop {
                select! {
                    _ = stream_buff_reader.read_line(&mut client_inp)=>{
                        tx2.send((client_inp.clone(), sock_addr)).unwrap();
                        client_inp.clear();
                    },

                    Ok((message, message_addr)) = rx2.recv() =>  {
                        let client_name = {
                            let client_nam_reader = client_data_clo.read().unwrap();
                            client_nam_reader.get(&message_addr).unwrap().clone()
                        };
                        tracing::info!("Receieved {:?} from {}", message.as_bytes(),client_name);
                        if message_addr == sock_addr {
                            continue;
                        }
                        let fm_str = format!(" {} : {}",client_name.blue(),message.green());
                        s_writer.write_all(fm_str.as_bytes()).await.unwrap();
                    }
                }
            }
        });
    }
}
