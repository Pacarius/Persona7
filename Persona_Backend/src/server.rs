use std::error::Error;

use tokio::{io::AsyncWriteExt, net::TcpStream};

use crate::{misc::ollama::ollama::Ollama, world::{navigation::Navigator, world::World}};

pub struct Server {
    llama: Ollama,
    world: World,
    clients: Vec<TcpStream>
}
pub struct Client {
    stream: TcpStream,
    initialised: bool,
}
impl Server {
    pub async fn main(&self) {
        loop{
            
        }
    }
    pub fn new(endpoint: &str, world: World) -> Self {
        Self {
            llama: Ollama::new(endpoint.into(), false),
            world,
            clients: vec![]
        }
    }
}
//Event based client
//On Connect ( On Next Tick ):{
//  Send RunUUID | Regions | Rooms | Objects | Characters
//}
impl Client{
    pub fn new(stream: TcpStream) -> Self{
        Self{stream, initialised: false}
    }
    pub async fn init(&mut self, navigator: &Navigator) -> Result<(), Box<dyn Error>>{
        // if self.stream.readable().await{
        //     let mut buffer = b"";
        //     self.stream.try_read(buffer)
        // }
        self.stream.write_all(todo!()).await?;
    }
}
