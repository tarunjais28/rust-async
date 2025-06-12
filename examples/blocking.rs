use colored::Colorize;
use event_handler::*;
use rand::random;
use std::{sync::mpsc, thread, time::Duration};

// Producer async function: sends events to the channel
fn producer(id: usize, tx: mpsc::Sender<Box<dyn Event>>, max_events: u64) {
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

        if tx.send(event).is_err() {
            break;
        }

        println!(
            "{}",
            format!("Producer {id} pushed: {description}").yellow()
        );

        thread::sleep(Duration::from_secs(5));
    }
}

fn main() {
    let (tx, rx) = mpsc::channel::<Box<dyn Event>>();

    let max_events_per_producer = 5;
    let max_producer = 3;
    let mut handles = Vec::new();

    // Thread1: Keeps reading new events from above function. Push it to a
    // queue. We can have many threads pushing to this: Many producers
    for id in 1..=max_producer {
        let thread_tx = tx.clone();
        let handle = thread::spawn(move || producer(id, thread_tx, max_events_per_producer));
        handles.push(handle);
    }

    drop(tx);

    // Thread2: Keeps reading this queue and logs the number of events till
    // now: Single consumer
    let consumer = thread::spawn(move || {
        let mut processed_events = 0;
        for event in rx {
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
    });

    handles.push(consumer);

    handles
        .into_iter()
        .for_each(|handle| handle.join().unwrap());

    println!(
        "{}",
        "All producers have stopped. Consumer exiting.".green()
    );
}
