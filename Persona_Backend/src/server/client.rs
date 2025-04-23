use std::{error::Error, fmt::format, time::Duration};

use serde_json::{json, Value};
use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
    net::{
        tcp::{OwnedReadHalf, OwnedWriteHalf},
        TcpStream,
    },
    time::timeout,
};

use crate::world::navigation::Navigator;

use super::message::Message;

pub struct Client {
    // stream: TcpStream,
    // initialised: bool,
    reader: BufReader<OwnedReadHalf>,
    writer: OwnedWriteHalf,
    cid: usize,
}
impl Client {
    pub fn new(stream: TcpStream, id: usize) -> Self {
        let (read, write) = stream.into_split();
        let read = BufReader::new(read);
        Self {
            reader: read,
            writer: write,
            cid: id, // initialised: false,
        }
    }
    pub async fn main(
        &mut self,
        rx: tokio::sync::watch::Receiver<String>,
        tx: tokio::sync::mpsc::Sender<String>,
        navigator: &Navigator,
    ) -> Result<(), Box<dyn Error>> {
        //rx as in to client and tx as in to server LOL
        // let mut buffer = String::new();
        let mut buffer = String::new();
        self.init(navigator).await?;
        loop {
            let received = rx.borrow().clone();
            if buffer != received {
                buffer = received.clone();
                if let Err(e) = self.writer.write_all(buffer.as_bytes()).await {
                    eprintln!("Failed to send message to client: {}", e);
                    return Err(Box::new(e));
                    break;
                } else {
                    self.writer.flush().await.unwrap();
                }
            }
            let mut read_line = String::new();
            let read = timeout(
                Duration::from_micros(1),
                self.reader.read_line(&mut read_line),
            )
            .await;
            match read {
                Err(_) => continue,
                Ok(result) => match result {
                    Err(e) => {
                        eprintln!("Failed to read message");
                        return Err(Box::new(e));
                        break;
                    }
                    Ok(o) => {
                        if o > 0 {
                            // println!("Received {}", read_line);
                            tx.send(read_line.clone()).await;
                            read_line.clear();
                        } else {
                            break;
                        }
                    }
                },
            }
        }
        Ok(())
    }

    async fn init(&mut self, navigator: &Navigator) -> Result<(), Box<dyn Error>> {
        // let targets = vec![
        //     format!("{:?}", navigator.size()),
        //     format!("{:?}", navigator.regions()),
        //     format!("{:?}", navigator.objects()),
        // ];
        // for t in targets {
        //     self.writer.write_all(t.as_bytes()).await?;
        //     // self.stream.write_all(b"\n").await?;
        // }

        // let map_data = json!({
        //     "size": format!("{:?}", navigator.size()),
        //     "regions": format!("{:?}", navigator.regions()),
        //     "objects": format!("{:?}", navigator.objects()),
        //     "characters": format!("{:?}", navigator.characters())
        // });
        // self.writer
        //     .write_all(
        //         json!({
        //             "map": map_data
        //         })
        //         .to_string()
        //         .as_bytes(),
        //     )
        //     .await?;

        let datas = vec![
            //Map data doesn't need to be relayed
            Message::new(
                super::message::MessageType::PY,
                json!({
                        "size": format!("{:?}", navigator.size()),
                "regions": format!("{:?}", navigator.regions()),
                "objects": format!("{:?}", navigator.objects()),
                })
                .to_string(),
                // navigator.timestamp().clone(),
                None,
            ),
            Message::new(
                super::message::MessageType::WEB,
                json!({
                    "characters": format!("{:?}", navigator.characters())
                })
                .to_string(),
                None,
            ),
        ];
        for init in datas{
            self.writer.write_all(init.to_string().as_bytes()).await;
        }
        return Ok(());
    }
}
