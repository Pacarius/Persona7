use std::{error::Error, ops::Deref, sync::Arc, time::Duration};

use futures::future::join_all;
use rand::Rng;
use serde_json::json;
use tokio::{
    io::{AsyncBufReadExt, AsyncReadExt, AsyncWriteExt, BufReader, BufWriter},
    net::{
        tcp::{OwnedReadHalf, OwnedWriteHalf},
        TcpListener, TcpStream,
    },
    sync::{broadcast, watch::Sender, Mutex},
    time::{self, sleep},
};

use crate::{
    misc::ollama::ollama::Ollama,
    world::{navigation::Navigator, world::World},
};

use super::client::Client;

pub struct Server {
    llama: Ollama,
    world: Arc<Mutex<World>>,
    clients: Arc<Mutex<Vec<Arc<Mutex<Client>>>>>,
    endpoint: String,
    // relay_to_client: Sender<String>,
    // relay_to_server
    //Tests will run with UUID 0, Live runs will hold positive uuids, Replays will hold negative uuids
    uuid: i64,
    // listener: TcpListener,
    // inbound: tokio::sync::mpsc::Receiver<String>,
    // outbound: tokio::sync::watch::Receiver<String>,
    from_client: (
        tokio::sync::mpsc::Receiver<String>,
        tokio::sync::mpsc::Sender<String>,
    ),
    to_client: (
        tokio::sync::watch::Receiver<String>,
        tokio::sync::watch::Sender<String>,
    ),
    // from_world: tokio::sync::watch
}
impl Server {
    pub async fn main(&mut self) -> Result<(), Box<dyn Error>> {
        // let world_clone = Arc::new(self.world);
        // loop {
        //     let (mut socket, _) = self.listener.accept().await?;
        //     println!("New client connected");
        //     let client = Arc::new(Mutex::new(Client::new(socket, self.clients.len())));
        //     self.clients.push(client.clone());
        //     let world_clone = self.world.clone();
        //     tokio::spawn(async move {
        //         let mut client = client.lock().await;
        //         if let Err(e) = client.init(&Navigator::new(world_clone.lock().await.get_map())).await {
        //             eprintln!("Failed to initialize client: {}", e);
        //         }
        //     });
        // let navigator = Navigator::new(world_clone.get_map());
        // let init = join_all(self.clients.iter_mut().map(|c| c.init(&navigator)));
        // init.await
        let to_client_rx = self.to_client.0.clone();
        let from_client_tx = self.from_client.1.clone();
        let clients = self.clients.clone();
        let world = self.world.clone();
        let endpoint = self.endpoint.clone();
        tokio::spawn(async move {
            Server::accept_clients(to_client_rx, from_client_tx, clients, world, &endpoint)
                .await
                .unwrap_or_else(|e| {
                    eprintln!("Error accepting clients: {}", e);
                });
        });
        loop {
            let world_arc = self.world.clone();
            let mut world = world_arc.lock().await;
            while let Ok(command) = self.from_client.0.try_recv() {
                println!("{}", command);
                // if command.contains("throbbing") {
                //     world.toggle_running();
                //     self.to_client.1.send("throbbing".to_string());
                // }
                match &command.trim() {
                    val if val == &"throbbing" => {
                        world.toggle_running();
                        self.to_client.1.send("throbbing".to_string());
                    }
                    val if val == &"NEED" => {
                        self.to_client.1.send(
                            // format!("{:?}", world.get_map().get_characters().iter().map(|c| json!({"name": c.name(),"position": c.position().to_string(),"plan": format!("{:?}", c.short_term_mem().plan_vague), "sprite": c.sprite()})).collect::<Vec<_>>()));
                            json!({
                                "type": "init",
                                "content": format!("{:?}", world.get_map().get_characters().iter().map(|c| json!({"name": c.name(),"position": c.position().to_string(),"plan": format!("{:?}", c.short_term_mem().plan_vague), "sprite": c.sprite()})).collect::<Vec<_>>())
                            }).to_string());
                    }
                    _ => {}
                }
            }
            let (new_day, relay) = world.tick(&self.llama, false).await;
            if new_day {
                // self.to_client.send()
                // self.to_client.1.send(
                //             format!("DAY_INIT: {:?}", world.get_map().get_characters().iter().map(|c| json!({"name": c.name(),"position": c.position().to_string(),"plan": format!("{:?}", c.short_term_mem().plan_vague), "sprite": c.sprite()})).collect::<Vec<_>>()));
                        self.to_client.1.send(
                            // format!("{:?}", world.get_map().get_characters().iter().map(|c| json!({"name": c.name(),"position": c.position().to_string(),"plan": format!("{:?}", c.short_term_mem().plan_vague), "sprite": c.sprite()})).collect::<Vec<_>>()));
                            json!({
                                "type": "init",
                                "content": format!("{:?}", world.get_map().get_characters().iter().map(|c| json!({"name": c.name(),"position": c.position().to_string(),"plan": format!("{:?}", c.short_term_mem().plan_vague), "sprite": c.sprite()})).collect::<Vec<_>>())
                            }).to_string());
            }
            if let Some(output) = relay {
                if let Ok(_) = self.to_client.1.send(output.clone()) {
                    // println!("Sent {} through channel.", output);
                }
            };
            sleep(Duration::from_millis(100)).await;
        }
        // }
        Ok(())
    }
    async fn accept_clients(
        mut rx: tokio::sync::watch::Receiver<String>,
        mut tx: tokio::sync::mpsc::Sender<String>,
        client_list: Arc<Mutex<Vec<Arc<Mutex<Client>>>>>,
        world: Arc<Mutex<World>>,
        endpoint: &str,
    ) -> Result<(), Box<dyn Error>> {
        if let Ok(listener) = TcpListener::bind(endpoint).await {
            println!("Server listening on {}", endpoint);
            loop {
                let (mut socket, _) = listener.accept().await?;
                println!("New client connected");
                let mut client_list = client_list.lock().await;
                let client = Arc::new(Mutex::new(Client::new(socket, client_list.len())));
                client_list.push(client.clone());
                let (rx, tx) = (rx.clone(), tx.clone());
                let world_clone = world.clone();
                tokio::spawn(async move {
                    let mut client = client.lock().await;
                    let navigator = Navigator::new(world_clone.lock().await.get_map());
                    if let Err(e) = client.main(rx.clone(), tx.clone(), &navigator).await {
                        eprintln!("Failed to initialize client: {}", e);
                    }
                });
            }
        };
        Ok(())
    }
    // async fn run_world(mut )
    pub async fn new(
        endpoint: &str,
        world: World,
        uuid: Option<i64>,
    ) -> Result<Self, Box<dyn Error>> {
        let endpoint = format!("{}:1234", endpoint);
        let (from_client_tx, from_client_rx) = tokio::sync::mpsc::channel(100);
        let (to_client_tx, to_client_rx) = tokio::sync::watch::channel(String::new());
        let output = Ok(Self {
            llama: Ollama::new(false),
            world: Arc::new(world.into()),
            clients: Arc::new(Mutex::new(vec![])),
            uuid: match uuid {
                Some(i) => i,
                None => 0,
            },
            // listener: TcpListener::bind(endpoint.clone()).await?,
            endpoint,
            from_client: (from_client_rx, from_client_tx),
            to_client: (to_client_rx, to_client_tx),
        });
        // if output.is_ok() {
        //     println!("Server listening on {}", endpoint)
        // }
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
