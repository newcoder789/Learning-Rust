// One increasingly popular approach to ensuring safe concurrency is message
// passing, where threads or actors communicate by sending each other
// messages containing data. Here's the idea in a slogan from the Go language
// documentation: "Do not communicate by sharing memory; instead, share
// memory by communicating."
// To accomplish message-sending concurrency, Rust's standard library provides
// an implementation of channels. A channel is a general programming concept by
// which data is sent from one thread to another.


// A channel has two halves: a transmitter and a receiver. The transmitter half is the upstream location where you put rubber
// ducks into the river, and the receiver half is where the rubber duck ends up downstream. One part of your code calls methods
// on the transmitter with the data you want to send, and another part checks the receiving end for arriving messages. A channel is
// said to be closed if either the transmitter or receiver half is
// dropped.


// mpsc -> multi producer single consumers

// use std::{
//     sync::mpsc,
//     thread::{self, spawn},
// };

// fn main() {
//     let (tx,rx) = mpsc::channel();

//     spawn(move ||{
//         tx.send(String::from("hi")).unwrap();
//     });

//     // use unwrap if u do not care about your code panicing cause of error  
//     let value = rx.recv().unwrap();
//     match value{
//         Ok(value) => println!("{}", value),
//         Err(err) => println!("ERRor while reading the data {err}")
//     }

// }

// ** Can you write the code that finds the sum from 1 - 10'"8? Use threads to make sure you use all cores of your machine Remember the name says •multiple producer single consumer' ** //
use std::sync::mpsc; // Import multi-producer, single-consumer channel
use std::thread;     // For spawning threads

fn main() {
    // Create a channel with sender (tx) and receiver (rx)
    let (tx, rx) = mpsc::channel();

    // Spawn 10 threads — each thread will calculate the sum of 10 million numbers
    for i in 0..10 {
        // Clone the sender so each thread can send its result
        let producer = tx.clone();

        // Spawn a thread that captures 'i' and 'producer' (move needed to transfer ownership)
        thread::spawn(move || {
            let mut sum: u64 = 0;

            // Each thread works on a different range of 10 million numbers
            for j in i * 10_000_000..(i + 1) * 10_000_000 {
                sum += j;
            }

            // Send the computed sum back to the main thread via the channel
            producer.send(sum).unwrap();
        });
    }

    // Drop the original sender to prevent rx from hanging
    //
    // 🧠 NOTE:
    // If we don't drop `tx`, the receiver loop (`for val in rx`) below won't know
    // when to stop — it will wait forever because it thinks more values might come.
    //
    // Dropping the original `tx` makes sure once all cloned `producer` senders finish,
    // the channel will be closed, and the loop will exit.
    drop(tx);

    // Now collect and sum all the values received from the threads
    let mut final_sum: u64 = 0;
    for val in rx {
        println!("recv value from thread");
        final_sum += val;
    }

    // Print the final total sum
    println!("{}", final_sum);
}
