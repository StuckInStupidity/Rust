use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use tokio::sync::mpsc;
use tokio::time::sleep;

#[tokio::main]
async fn main() {
    // --- Sync counter with threads ---
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..50 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
    println!("Result: {}", *counter.lock().unwrap());

    // --- Sync channel ---
    let (tx, rx) = std::sync::mpsc::channel();
    thread::spawn(move || {
        let vals = vec!["hi", "from", "the", "thread"];
        for val in vals {
            tx.send(val.to_string()).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("Got: {received}");
    }

    // --- Async tasks ---
    let fut1 = async {
        for i in 1..10 {
            println!("hi number {i} from the first task!");
            sleep(Duration::from_millis(500)).await;
        }
    };
    let fut2 = async {
        for i in 1..5 {
            println!("hi number {i} from the second task!");
            sleep(Duration::from_millis(500)).await;
        }
    };
    tokio::join!(fut1, fut2);

    // --- Async channel ---
    let (tx, mut rx) = mpsc::channel(32);
    let tx1 = tx.clone();
    let tx1_fut = async move {
        let vals = vec!["hi", "from", "the", "future"];
        for val in vals {
            tx1.send(val.to_string()).await.unwrap();
            sleep(Duration::from_millis(500)).await;
        }
    };
    let tx_fut = async move {
        let vals = vec!["more", "messages", "for", "you"];
        for val in vals {
            tx.send(val.to_string()).await.unwrap();
            sleep(Duration::from_millis(1500)).await;
        }
    };
    let rx_fut = async {
        while let Some(value) = rx.recv().await {
            println!("received '{value}'");
        }
    };
    tokio::join!(tx1_fut, tx_fut, rx_fut);
}