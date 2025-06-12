use std::sync::Arc;
use tokio::sync::Mutex;
use futures::future::join_all;

#[tokio::main]
async fn main() {
    let data = Arc::new(Mutex::new(0));

    let handles: Vec<_> = (0..10)
        .map(|_| {
            let data = data.clone();
            tokio::spawn(async move {
                *data.lock().await += 1;
            })
        })
        .collect();

    join_all(handles).await;

    println!("Result: {:?}", *data.lock().await);
}
