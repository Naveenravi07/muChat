extern crate colored;
use anyhow::Result;
use censor::*;
use colored::*;
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

const MAX_CLIENT_NAME_LENGTH: usize = 10;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    let addr = "0.0.0.0:8080";
    let listener = TcpListener::bind(addr).await?;
    let (tx, _rx): (
        Sender<(String, Option<SocketAddr>)>,
        Receiver<(String, Option<SocketAddr>)>,
    ) = tokio::sync::broadcast::channel(30);
    let client_data: Arc<RwLock<HashMap<SocketAddr, String>>> =
        Arc::new(RwLock::new(HashMap::new()));
    //let client_handlers: Arc<RwLock<HashMap<SocketAddr, JoinHandle<_>>>> = Arc::new(RwLock::new(HashMap::new()));

    loop {
        let tx2 = tx.clone();
        let mut rx2 = tx.subscribe();

        let (mut stream, sock_addr) = listener.accept().await?;
        tracing::info!("INFO: new client connected successfully");
        let client_data_clo = Arc::clone(&client_data);
        // let client_handlers_clo = Arc::clone(&client_handlers);

        let task = tokio::spawn(async move {
            let censor = Censor::Standard;
            let (s_reader, mut s_writer) = stream.split();
            let mut stream_buff_reader = BufReader::new(s_reader);
            let mut client_inp = String::new();

            s_writer
                .write("\n Please Enter your username \t".yellow().as_bytes())
                .await
                .unwrap();
            stream_buff_reader.read_line(&mut client_inp).await.unwrap();

            // Validations
            if client_inp.len() > MAX_CLIENT_NAME_LENGTH {
                s_writer
                    .write("NICK NAME TOO LONG ! Exiting.. ".bright_red().as_bytes())
                    .await
                    .unwrap();
                return;
                //if let Some(task) = client_handlers_clo.read().unwrap().get(&sock_addr.clone()) {
                //    task.abort();
                //}
            }

            let nickname_exists = {
                let client_data_read = client_data_clo.read().unwrap();
                client_data_read
                    .values()
                    .any(|name| name == client_inp.trim())
            };

            if nickname_exists {
                s_writer
                    .write("Nickname already taken".red().as_bytes())
                    .await
                    .unwrap();
                return;
            }

            client_data_clo
                .write()
                .unwrap()
                .insert(sock_addr, client_inp.trim().to_string());

            let welcome = format!("\n {} Joined the club \n", client_inp.trim());
            client_inp.clear();

            tx2.send((welcome, None)).unwrap();
            loop {
                select! {
                    _ = stream_buff_reader.read_line(&mut client_inp)=>{
                        client_inp = censor.censor(&client_inp);
                        tx2.send((client_inp.clone(), Some(sock_addr))).unwrap();
                        client_inp.clear();
                    },

                    Ok((message, message_addr)) = rx2.recv() =>  {

                        let fm_str = if let Some(address) = message_addr{
                            let client_name = {
                                let client_nam_reader = client_data_clo.read().unwrap();
                                client_nam_reader.get(&address).unwrap().clone()
                            };
                            tracing::info!("Receieved {:?} from {}", message.as_bytes(),client_name);
                            if address == sock_addr {
                                continue;
                            }
                             format!(" {} : {}",client_name.blue(),message.bright_green())
                        }else{
                             format!(" {}",message.bright_green())
                        };
                        s_writer.write_all(fm_str.as_bytes()).await.unwrap();
                        s_writer.flush().await.unwrap();
                    }
                }
            }
        });
        //client_handlers.write().unwrap().insert(sock_addr, task);
    }
}
