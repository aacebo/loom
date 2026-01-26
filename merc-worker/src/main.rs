mod config;

use merc_events::{Key, MemoryAction};

use config::Config;

#[tokio::main]
async fn main() -> Result<(), merc_error::Error> {
    let config = Config::from_env();
    let socket = merc_events::new(&config.rabbitmq_url)
        .with_app_id("merc[worker]")
        .with_queue(Key::memory(MemoryAction::Create))
        .connect()
        .await?;

    let mut consumer = socket.consume(Key::memory(MemoryAction::Create)).await?;

    println!("waiting for messages on memory.create...");

    while let Some(res) = consumer.dequeue::<String>().await {
        let _ = match res {
            Err(err) => return Err(err),
            Ok(v) => v,
        };
    }

    Ok(())
}
