use std::{
    thread::sleep,
    time::{Duration, Instant},
};

use esercizio1::RingBuf;

fn main() {
    let mut buf = RingBuf::new("buffer.bin", 10);
    let mut seq = 0;
    let start = Instant::now();
    loop {
        let elapsed = start.elapsed().as_millis() as i64;
        let drift = elapsed - 10000 * seq as i64;
        sleep(Duration::from_millis(
            (10000 - drift).try_into().unwrap_or(0),
        ));

        let mut count = 0;
        while let Some(data) = buf.read() {
            println!("consumer : sequence {}", data.seq);
            println!("consumer : timestamp {}", data.timestamp);
            // println!("consumer : values {}", data.values);
            for value in data.values.iter() {
                println!("consumer : value {}", value);
            }
            count += 1;
        }
        println!("consumer : read {} values", count);

        seq += 1;
    }
}
