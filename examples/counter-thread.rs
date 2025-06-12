use std::{
    sync::{Arc, Mutex},
    thread,
};

fn main() {
    let data = Arc::new(Mutex::new(0));

    let handles: Vec<_> = (0..10)
        .map(|_| {
            let data = data.clone();
            thread::spawn(move || {
                *data.lock().unwrap() += 1;
            })
        })
        .collect();

    handles.into_iter().for_each(|handle| {
        handle.join().unwrap();
    });

    println!("Result: {:?}", *data.lock().unwrap());
}
