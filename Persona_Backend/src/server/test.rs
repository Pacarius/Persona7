use std::{error::Error, sync::Arc, time::Duration};

use futures::{future::join_all, lock::Mutex};
use rand::{rng, Rng};
use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
    net::{
        tcp::{OwnedReadHalf, OwnedWriteHalf},
        TcpListener, TcpStream,
    },
    sync::watch::{self, Receiver, Sender},
    time::{sleep, timeout},
};

pub struct Server {
    clients: Vec<Arc<Mutex<Client>>>,
    listener: TcpListener,
}
pub struct Client {
    // stream: TcpStream,
    writer: OwnedWriteHalf,
    reader: BufReader<OwnedReadHalf>,
}
impl Server {
    pub async fn new() -> Self {
        let listener = TcpListener::bind("0.0.0.0:1234").await.unwrap();
        Self {
            clients: vec![],
            listener,
        }
    }
    pub async fn main(&mut self) -> Result<(), Box<dyn Error>> {
        let (tx, mut rx) = watch::channel(String::new());
        tokio::join!(Server::numbers(tx.clone()), self.accept_clients(rx.clone()));
        Ok(())
    }
    async fn numbers(tx: Sender<String>) {
        let mut rng = rng();
        let (mut i, mut j) = (0, 0);
        loop {
            for mut num in [&mut i, &mut j] {
                *num += rng.random_range(1..=3);
                if *num > 10 {
                    *num = 0;
                }
            }
            let message= format!("i:  {}, j: {}\n", i, j);
            // println!("Sent: {}", message);
            tx.send(message);
            sleep(Duration::from_secs(3)).await;
        }
    }
    async fn accept_clients(&mut self, rx: Receiver<String>) -> Result<(), Box<dyn Error>> {
        loop {
            if let Ok((socket, _)) = self.listener.accept().await {
                let client = Arc::new(Mutex::new(Client::new(socket)));
                self.clients.push(client.clone());
                let rx_clone = rx.clone();
                tokio::spawn(async move {
                    let mut client = client.lock().await;
                    if let Err(e) = client.main(rx_clone).await {
                        eprintln!("Error handling client: {}", e);
                    }
                });
            }
        }
        Ok(())
    }
}
impl Client {
    pub fn new(stream: TcpStream) -> Self {
        let (reader, writer) = stream.into_split();
        let reader = BufReader::new(reader);
        Self { writer, reader }
    }
    pub async fn main(&mut self, rx: Receiver<String>) -> Result<(), Box<dyn Error>> {
        let mut buffer = String::new();
        loop {
            // let received = rx.borrow().clone();
            let received = rx.borrow().clone();
            // println!("Received Signal : {}; Buffer: {}", received, buffer);
            // buffer = match received {
            //     buffer => buffer,
            //     output => {
            //         if let Err(e) = self.writer.write_all(output.as_bytes()).await {
            //             eprintln!("Failed to send message to client: {}", e);
            //             return Err(Box::new(e));
            //             break;
            //         }
            //         else{
            //             self.writer.flush().await;
            //         }
            //         output
            //     }
            // };
            if buffer != received{
                buffer = received.clone();
                    if let Err(e) = self.writer.write_all(buffer.as_bytes()).await {
                        eprintln!("Failed to send message to client: {}", e);
                        return Err(Box::new(e));
                        break;
                    }
                    else{
                        self.writer.flush().await.unwrap();
                    }
            }
            // let mut client_message = String::new();
            let mut read_line = String::new();
            let read = timeout(Duration::from_secs(1), self.reader.read_line(&mut read_line)).await;
            match read{
                Err(_) => continue,
                Ok(result) => {
                    match result{
                        Err(e) => {
                            eprintln!("Failed to send message to client: {}", e);
                            return Err(Box::new(e));
                            break;
                        }
                        Ok(o) => {
                            if o > 0{
                                    println!("Received {}", read_line);
                                    read_line.clear();
                            }
                            else {break}
                        }
                    }
                }
            }
            // let reader = &mut self.reader;
            // let mut read_lines = reader.lines();
            // let mut output = String::new();
            // while let Some(line) = timeout(Duration::from_secs(1), ){
            //     output.push('\n');
            //     output.push_str(&line);
            // }
            // if !output.is_empty(){println!("{}", output)}
            // // buffer = received;

            sleep(Duration::from_secs(1)).await;
        }
        Ok(())
    }
}
