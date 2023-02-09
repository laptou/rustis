use rustis::{client::Client, Result, commands::{StringCommands, GenericCommands}};

#[tokio::main]
async fn main() -> Result<()> {
    let mut client = Client::connect("127.0.0.1:6379").await?;

    for _i in 0..1000 {
        let key = "test_key";
        client.set(key, 42.423456).await?;
        let _: f64 = client.get(key).await?;
        client.del(key).await?;
    }    

    Ok(())
}