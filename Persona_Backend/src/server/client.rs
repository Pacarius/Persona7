use std::error::Error;

use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
    net::{
        tcp::{OwnedReadHalf, OwnedWriteHalf},
        TcpStream,
    },
};

use crate::world::navigation::Navigator;

pub struct Client {
    // stream: TcpStream,
    // initialised: bool,
    reader: OwnedReadHalf,
    writer: OwnedWriteHalf,
    cid: usize
}
impl Client {
    pub fn new(stream: TcpStream, id: usize) -> Self {
        let (read, write) = stream.into_split();
        Self {
            reader: read,
            writer: write,
            cid: id
            // initialised: false,
        }
    }
    // pub async fn read_all(&mut self) -> Result<Option<Vec<String>>, Box<dyn Error>>{
    //     let mut reader = BufReader::new(&mut self.reader);
    //     let mut output = None;
    //     let mut line = String::new();
    //     let result = reader.poll_read(line);
    //     // while reader.read_line(&mut line).await? > 0{
    //     //     if line.len() != 0{
    //     //         output.push(line.clone());
    //     //     }
    //     //     line.clear();
    //     // }
    //     Ok(output)
    // }
    pub async fn read_messages(&mut self) -> Result<(), Box<dyn Error>> {
        let mut reader = BufReader::new(&mut self.reader);
        let mut line = String::new();

        loop {
            let result = reader.read_line(&mut line).await?;
            if result > 0 {
                println!("Received from client: {}", line.trim());
                println!("{}", result);
                line.clear();
            } else {
                break;
            }
        }

        Ok(())
    }
    pub async fn init(&mut self, navigator: &Navigator) -> Result<(), Box<dyn Error>> {
        let targets = vec![
            format!("{:?}", navigator.size()),
            format!("{:?}", navigator.regions()),
            format!("{:?}", navigator.objects()),
        ];
        for t in targets {
            self.writer.write_all(t.as_bytes()).await?;
            // self.stream.write_all(b"\n").await?;
        }
        return Ok(());
    }
}
