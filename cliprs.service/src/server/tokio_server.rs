use std::sync::Mutex;

use clipboard::{ClipboardContext, ClipboardProvider};
use lazy_static::lazy_static;
use mini_redis::{Connection, Frame};
use tokio::net::{TcpListener, TcpStream};

use crate::clipboard_managers::clipboard_manager::{self, VectorWrapper};

lazy_static! {
    pub static ref DATA: Mutex<VectorWrapper<String>> =
        Mutex::new(VectorWrapper::<String>::new());
}

pub struct ApplicationServer {
    ip: String,
    port: String,
}

impl ApplicationServer {
    pub fn new() -> Self {
        Self {
            ip: "127.0.0.1".to_string(),
            port: "6379".to_string(),
        }
    }

    pub async fn run(&self) {
        let listener = TcpListener::bind(format!("{}:{}", self.ip, self.port))
            .await
            .unwrap();

        loop {
            tokio::spawn(async move {
                let manager = clipboard_manager::initialize();
                manager.run();
            });

            let (socket, _) = listener.accept().await.unwrap();

            tokio::spawn(async move {
                let ctx: ClipboardContext = ClipboardProvider::new().unwrap();
                Self::process(socket, ctx).await;
            });
        }
    }

    async fn process(socket: TcpStream, mut ctx: ClipboardContext) {
        use mini_redis::Command::{self, Get, Set};

        let mut connection = Connection::new(socket);

        while let Some(frame) = connection.read_frame().await.unwrap() {
            let response = match Command::from_frame(frame).unwrap() {
                Set(cmd) => {
                    ctx.set_contents(cmd.key().to_string().to_owned()).unwrap();
                    Frame::Simple("OK".to_string())
                }
                Get(_) => {
                    let lock = DATA.lock().unwrap();
                    let history = lock.clone();
                    drop(lock);
                    Frame::Simple(history.to_comma_separated())
                }
                cmd => panic!("Unimplemented {:?}", cmd),
            };

            connection.write_frame(&response).await.unwrap();
        }
    }
}
