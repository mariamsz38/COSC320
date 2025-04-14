use std::{sync::{mpsc, Arc}, thread, time::Duration};

struct Queue {
    first_half: Vec<u32>,
    second_half: Vec<u32>,
}

impl Queue {
    fn new() -> Self {
        Self {
            first_half: vec![1, 2, 3, 4, 5],
            second_half: vec![6, 7, 8, 9, 10],
        }
    }
}

fn send_tx(q: Arc<Queue>, tx: mpsc::Sender<u32>) {
    // TODO: We want to send `tx` to both threads. But currently, it is moved
    // into the first thread. How could you solve this problem?
    let tx1 = tx.clone();
    let q1 = Arc::clone(&q);
    thread::spawn(move || {
        for val in &q1.first_half {
            println!("Sending {val:?}");
            tx1.send(*val).unwrap();
            thread::sleep(Duration::from_millis(250));
        }
    });

    thread::spawn(move || {
        for val in &q.second_half {
            println!("Sending {val:?}");
            tx.send(*val).unwrap();
            thread::sleep(Duration::from_millis(250));
        }
    });
}

fn main() {
    // You can optionally experiment here.
    let (tx, rx) = mpsc::channel();
    let queue = Arc::new(Queue::new());
    send_tx(queue, tx);
    let mut received = Vec::new();
    for value in rx {
        received.push(value);
    }
    received.sort();
    println!("Received: {:?}", received);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn threads3() {
        let (tx, rx) = mpsc::channel();
        let queue = Arc::new(Queue::new());

        send_tx(queue, tx);

        let mut received = Vec::with_capacity(10);
        for value in rx {
            received.push(value);
        }

        received.sort();
        assert_eq!(received, [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
    }
}