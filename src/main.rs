use colored::Colorize;
pub use events::*;
use rand::random;
use tokio::{
    sync::mpsc,
    time::{Duration, sleep},
};

mod events;

// Producer async function: sends events to the channel
async fn producer(id: usize, tx: mpsc::Sender<Box<dyn Event>>, max_events: u64) {
    for _ in 1..=max_events {
        // Randomly select event type
        let event: Box<dyn Event> = match random::<u8>() % 5 {
            0 => Box::new(CreateTokenEvent::new()),
            1 => Box::new(MintEvent::new()),
            2 => Box::new(BurnEvent::new()),
            3 => Box::new(TransferEvent::new()),
            _ => Box::new(FreezeEvent::new()),
        };

        let description = event.describe();

        if tx.send(event).await.is_err() {
            break;
        }

        println!(
            "{}",
            format!("Producer {id} pushed: {description}").yellow()
        );

        sleep(Duration::from_secs(1)).await;
    }
}

#[tokio::main]
async fn main() {
    let (tx, mut rx) = mpsc::channel(10);

    let max_events_per_producer = 5;
    let max_producer = 3;

    // Thread1: Keeps reading new events from above function. Push it to a
    // queue. We can have many threads pushing to this: Many producers
    let producer = tokio::spawn({
        let tx = tx.clone();
        async move {
            for id in 1..=max_producer {
                let thread_tx = tx.clone();
                producer(id, thread_tx, max_events_per_producer).await;
            }
        }
    });

    drop(tx);

    // Thread2: Keeps reading this queue and logs the number of events till
    // now: Single consumer
    let consumer = tokio::spawn(async move {
        let mut processed_events = 0;
        while let Some(event) = rx.recv().await {
            processed_events += 1;
            println!(
                "{}",
                format!(
                    "Consumer processed event #{}: {}",
                    processed_events,
                    event.describe()
                )
                .blue()
            );
        }

        println!(
            "{}",
            "All producers have stopped. Consumer exiting.".green()
        );
    });

    producer.await.unwrap();
    consumer.await.unwrap();
}
