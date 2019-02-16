use std::collections::HashMap;
use std::sync::mpsc;
use std::thread;

pub fn frequency(input: &[&str], worker_count: usize) -> HashMap<char, usize> {
    if input.is_empty() {
        return HashMap::new();
    }
    let spawn = |string: &str, tx_thrd: mpsc::Sender<HashMap<char, usize>>| {
        let string_cpy = string.to_string();
        let _handle = thread::spawn(move || {
            let mut frequencies_thrd = HashMap::new();
            for ch in string_cpy.to_lowercase().chars() {
                if ch.is_alphabetic() {
                    let counter = frequencies_thrd.entry(ch).or_insert(0);
                    *counter += 1;
                }
            }
            tx_thrd.send(frequencies_thrd).unwrap();
        });
    };

    let (tx, rx) = mpsc::channel();
    let mut txs = Vec::with_capacity(input.len());
    for _ in 1..input.len() {
        txs.push(mpsc::Sender::clone(&tx));
    }
    txs.push(tx);

    let mut n_spawned = 0;
    for string in input.iter().take(worker_count) {
        let tx_thrd = txs.pop().unwrap();
        spawn(string, tx_thrd);
        n_spawned += 1;
    }

    let mut frequencies = HashMap::new();
    // When a worker finishes, add its result to the main frequency hashmap
    // and spawn a new worker if there's more work left to be done.
    for mut received in rx {
        for (ch, counter_thrd) in received.drain() {
            let counter = frequencies.entry(ch).or_insert(0);
            *counter += counter_thrd;
        }
        if n_spawned < input.len() {
            let tx_thrd = txs.pop().unwrap();
            spawn(input[n_spawned], tx_thrd);
            n_spawned += 1;
        }
    }

    frequencies
}
