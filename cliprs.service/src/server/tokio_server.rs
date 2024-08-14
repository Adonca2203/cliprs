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

            tokio::spawn(async move {
                let ctx: ClipboardContext = ClipboardProvider::new().unwrap();
                Self::process(socket, ctx).await;
            });
        }
    }

    async fn process(socket: TcpStream, mut ctx: ClipboardContext) {
        use mini_redis::Command::{self, Set};

        let mut connection = Connection::new(socket);

        while let Some(frame) = connection.read_frame().await.unwrap() {
            let response = match Command::from_frame(frame).unwrap() {
                Set(cmd) => {
                    ctx.set_contents(cmd.key().to_string().to_owned()).unwrap();
                    Frame::Simple("OK".to_string())
                }
                cmd => panic!("Unimplemented {:?}", cmd),
            };

            connection.write_frame(&response).await.unwrap();
        }
    }
}
