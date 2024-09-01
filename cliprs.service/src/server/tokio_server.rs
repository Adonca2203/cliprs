use std::sync::{Arc, Mutex};

use clipboard::{ClipboardContext, ClipboardProvider};
use mini_redis::{Connection, Frame};
use tokio::net::{TcpListener, TcpStream};

use crate::clipboard_managers::clipboard_manager::{self, ClipboardManager};
type Manager = Arc<Mutex<Box<dyn ClipboardManager>>>;

pub struct ApplicationServer {
    ip: String,
    port: String,
    mngr: Manager,
}

impl ApplicationServer {
    pub fn new() -> Self {
        Self {
            ip: "127.0.0.1".to_string(),
            port: "6379".to_string(),
            mngr: Arc::new(Mutex::new(clipboard_manager::initialize())),
        }
    }

    pub async fn run(&self) {
        let listener = TcpListener::bind(format!("{}:{}", self.ip, self.port))
            .await
            .unwrap();

        loop {
            let mngr = self.mngr.clone();

            tokio::spawn(async move {
                mngr.lock().unwrap().run();
            });

            let (socket, _) = listener.accept().await.unwrap();

            let other = self.mngr.clone();
            tokio::spawn(async move {
                let ctx: ClipboardContext = ClipboardProvider::new().unwrap();
                Self::process(socket, ctx, other).await;
            });
        }
    }

    async fn process(socket: TcpStream, mut ctx: ClipboardContext, manager: Manager) {
        use mini_redis::Command::{self, Get, Set};

        let mut connection = Connection::new(socket);

        while let Some(frame) = connection.read_frame().await.unwrap() {
            let response = match Command::from_frame(frame).unwrap() {
                Set(cmd) => {
                    ctx.set_contents(cmd.key().to_string().to_owned()).unwrap();
                    Frame::Simple("OK".to_string())
                }
                Get(cmd) => {
                    let history = manager.lock().unwrap().get_history();
                    let value: Vec<&String> = history
                        .iter()
                        .filter(|val| val.as_str() == cmd.key())
                        .collect();
                    if value.len() > 0 {
                        Frame::Simple(value.first().unwrap().to_string());
                        return;
                    } else {
                        Frame::Null
                    }
                }
                cmd => panic!("Unimplemented {:?}", cmd),
            };

            connection.write_frame(&response).await.unwrap();
        }
    }
}
