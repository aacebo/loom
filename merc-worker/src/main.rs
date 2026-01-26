use merc_events::{Key, MemoryAction};

#[tokio::main]
async fn main() -> Result<(), merc_error::Error> {
    let rabbitmq_url = std::env::var("RABBITMQ_URL")
        .unwrap_or_else(|_| "amqp://admin:admin@localhost:5672".to_string());

    println!("connecting to rabbitmq at {}", rabbitmq_url);

    let mut consumer = merc_events::new(&rabbitmq_url)
        .with_app_id("merc[worker]")
        .with_queue(Key::memory(MemoryAction::Create))
        .connect()
        .await?
        .consume(Key::memory(MemoryAction::Create))
        .await?;

    println!("waiting for messages on memory.create...");

    while let Some(res) = consumer.dequeue::<String>().await {
        let _ = match res {
            Err(err) => return Err(err),
            Ok(v) => v,
        };
    }

    Ok(())
}
