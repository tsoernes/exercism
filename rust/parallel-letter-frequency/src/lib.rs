use std::collections::HashMap;
use std::sync::mpsc;
use std::thread;

/// Given a String and a transmit channel, spawn a thread and
/// transmit back the frequency map of the String.
fn async_string_frequency(string: String, tx: mpsc::Sender<HashMap<char, usize>>) {
    let _handle = thread::spawn(move || {
        let mut frequencies = HashMap::new();
        for ch in string.to_lowercase().chars() {
            if ch.is_alphabetic() {
                let counter = frequencies.entry(ch).or_insert(0);
                *counter += 1;
            }
        }
        tx.send(frequencies).unwrap();
    });
}

/// Balances the input strings so that each worker is fed the same number of
/// bytes to work on. This prevents starvation if one of the input strings is much longer
/// than the rest of the strings combined.
pub fn frequency(input: &[&str], worker_count: usize) -> HashMap<char, usize> {
    if input.is_empty() {
        return HashMap::new();
    }

    // One transmitter for each worker and a single receiver
    let (tx, rx) = mpsc::channel();
    let mut txs = Vec::with_capacity(worker_count);
    for _ in 1..worker_count {
        txs.push(mpsc::Sender::clone(&tx));
    }
    txs.push(tx);

    // Given total string length of K and N workers, give strings of length (in bytes)
    // (N / K + 1) to the first (N % K) workers and (N / K) to the rest (floor div).
    let tot_len: usize = input.iter().map(|string| string.len()).sum();
    let mut worker_str_lens = vec![tot_len / worker_count; worker_count];
    for i in 0..tot_len % worker_count {
        worker_str_lens[i] += 1;
    }
    let mut str_ix = 0; // Index of first input string with bytes remaining to be processed
    let mut str_bytes: usize = 0; // Number of bytes already processed of input[str_ix]

    for (worker_str_len, worker_tx) in worker_str_lens.into_iter().zip(txs.into_iter()) {
        // In addition, allocate 3 extra bytes to each worker to avoid splitting a
        // potentially 4 byte character in the middle.
        let mut worker_str: String = String::with_capacity(worker_str_len + 3);
        while worker_str.len() < worker_str_len && str_ix < input.len() {
            let input_str = input[str_ix];
            let rem = worker_str_len - worker_str.len();
            if rem >= input_str.len() - str_bytes {
                // Consume remainder of input string
                worker_str += &input_str[str_bytes..];
                str_ix += 1;
                str_bytes = 0;
            } else {
                // Partially consume input string
                let mut end = str_bytes + rem;
                while !&input_str.is_char_boundary(end) {
                    // Avoid splitting multi-byte char
                    end += 1;
                }
                worker_str += &input_str[str_bytes..end];
                str_bytes += end;
                if str_bytes >= input_str.len() {
                    str_ix += 1;
                    str_bytes = 0;
                }
            }
        }

        async_string_frequency(worker_str, worker_tx);
    }

    // When a worker finishes, add its result to the main frequency hashmap
    let mut frequencies = HashMap::new();
    for mut received in rx {
        for (ch, counter_thrd) in received.drain() {
            let counter = frequencies.entry(ch).or_insert(0);
            *counter += counter_thrd;
        }
    }

    frequencies
}

// The workers are fed complete strings from the input list of strings as they complete
// their work. This approach causes starvation if one of the input strings is much longer
// than the rest of the strings combined.
pub fn frequency_unbalanced(input: &[&str], worker_count: usize) -> HashMap<char, usize> {
    if input.is_empty() {
        return HashMap::new();
    }

    let (tx, rx) = mpsc::channel();
    let mut txs = Vec::with_capacity(input.len());
    for _ in 0..input.len() {
        txs.push(mpsc::Sender::clone(&tx));
    }
    txs.push(tx);

    let mut n_spawned = 0;
    for str_ in input.iter().take(worker_count) {
        let worker_tx = txs.pop().unwrap();
        let string = str_.to_string();
        async_string_frequency(string, worker_tx);
        n_spawned += 1;
    }

    // When a worker finishes, add its result to the main frequency hashmap
    // and spawn a new worker if there's more work left to be done.
    let mut frequencies = HashMap::new();
    for mut received in rx {
        for (ch, counter_thrd) in received.drain() {
            let counter = frequencies.entry(ch).or_insert(0);
            *counter += counter_thrd;
        }
        if n_spawned < input.len() {
            let worker_tx = txs.pop().unwrap();
            let string = input[n_spawned].to_string();
            async_string_frequency(string, worker_tx);
            n_spawned += 1;
        }
    }

    frequencies
}

// use std::ops::Add;
// Given an iterator yielding e1, e2, e3, ..
// return [d+e1, d+e1+e2, d+e1+e2+e3, ..]
// where d is the default element of the element type
// fn acc_sums<I, E>(i: I) -> Vec<E>
// where
//     I: IntoIterator<Item = E>,
//     E: Add<Output = E>,
//     E: Default,
//     E: Copy,
// {
//     i.into_iter()
//         .scan(Default::default(), |sum, el| {
//             *sum = *sum + el;
//             Some(*sum)
//         })
//         .collect()
// }
