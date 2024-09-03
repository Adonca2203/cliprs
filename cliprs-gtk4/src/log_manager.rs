use mini_redis::client;

pub struct LogManager {}

impl LogManager {
    pub fn get_logs() -> Vec<String> {
        let mut ret = Vec::new();

        use tokio::runtime::Runtime;

        let rt = Runtime::new().expect("Created Log Tokio Runtime");

        rt.block_on(async {
            let addr = "127.0.0.1:6379";
            if let Ok(mut conn) = client::connect(addr).await {
                ret = match conn.get("logs").await {
                    Ok(result) => match result {
                        Some(str) => {
                            let text = String::from_utf8(str.to_vec()).unwrap();
                            text.split(",").map(|x| x.to_string()).collect()
                        }
                        None => vec![],
                    },
                    Err(err) => {
                        println!("Error: {:?}", err);
                        vec![]
                    }
                }
            }
        });
        ret
    }
}
