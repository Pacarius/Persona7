use std::{error::Error, ops::Deref, sync::Arc, time::Duration};

use futures::future::join_all;
use rand::Rng;
use tokio::{
    io::{AsyncBufReadExt, AsyncReadExt, AsyncWriteExt, BufReader, BufWriter},
    net::{
        tcp::{OwnedReadHalf, OwnedWriteHalf},
        TcpListener, TcpStream,
    },
    sync::Mutex,
    time,
};

use crate::{
    misc::ollama::ollama::Ollama,
    world::{navigation::Navigator, world::World},
};

use super::client::Client;

pub struct Server {
    llama: Ollama,
    world: Arc<World>,
    clients: Vec<Arc<Mutex<Client>>>,
    //Tests will run with UUID 0, Live runs will hold positive uuids, Replays will hold negative uuids
    uuid: i64,
    listener: TcpListener,
}
impl Server {
    pub async fn main(&mut self) -> Result<(), Box<dyn Error>> {
        // let world_clone = Arc::new(self.world);
        loop {
            let (mut socket, _) = self.listener.accept().await?;
            println!("New client connected");
            let client = Arc::new(Mutex::new(Client::new(socket, self.clients.len())));
            self.clients.push(client.clone());
            let world_clone = self.world.clone();
            tokio::spawn(async move {
                let mut client = client.lock().await;
                if let Err(e) = client.init(&Navigator::new(world_clone.get_map())).await {
                    eprintln!("Failed to initialize client: {}", e);
                }
            });
            // let navigator = Navigator::new(world_clone.get_map());
            // let init = join_all(self.clients.iter_mut().map(|c| c.init(&navigator)));
            // init.await;
        }
    }
    pub async fn new(
        endpoint: &str,
        world: World,
        uuid: Option<i64>,
    ) -> Result<Self, Box<dyn Error>> {
        let endpoint = format!("{}:1234", endpoint);
        let output = Ok(Self {
            llama: Ollama::new(false),
            world: Arc::new(world),
            clients: vec![],
            uuid: match uuid {
                Some(i) => i,
                None => 0,
            },
            listener: TcpListener::bind(endpoint.clone()).await?,
        });
        if output.is_ok() {
            println!("Server listening on {}", endpoint)
        }
        output
    }
    // pub async fn new_client(&mut self, navigator: &Navigator) -> Result<(), Box<dyn Error>>{
    //     Ok(())
    // }
}
//Event based client
//On Connect ( On Next Tick ):{
//  Send RunUUID | Regions | Rooms | Objects | Characters
//}
