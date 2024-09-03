use mini_redis::{client, Result};

#[tokio::main]
async fn main() -> Result<()> {
    // Open a connection to the mini-redis address.
    let mut client = client::connect("127.0.0.1:6379").await?;

    // Set the key "hello" with value "world"
    client.set("world", "world".into()).await?;

    let result = match client.get("world").await {
        Ok(result) => result,
        Err(err) => {
            panic!("Error: {:?}", err);
        }
    };

    println!("got value from server; result={:?}", result);

    Ok(())
}

