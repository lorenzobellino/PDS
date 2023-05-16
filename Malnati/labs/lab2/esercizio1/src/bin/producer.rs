use std::{
    thread::sleep,
    time::{Duration, Instant},
};

use esercizio1::{my_time, RingBuf, SensorData};

fn main() {
    let mut seq = 0;
    let mut buf = RingBuf::new("buffer.bin", 10);

    let start = Instant::now();

    let mut v = 0.01;

    println!("producer: starting");

    loop {
        let data = SensorData {
            seq: seq,
            values: [v; 10],
            timestamp: my_time(),
        };
        v += 0.01;
        if let Some(_) = buf.write(data) {
            println!("producer: wrote {}", seq);
        } else {
            println!("producer: buffer full");
        }

        // println!("producer: sleeping");

        let elapsed = start.elapsed().as_millis() as i64;
        let drift = elapsed - 1000 * seq as i64;
        sleep(Duration::from_millis(
            (1000 - drift).try_into().unwrap_or(0),
        ));
        seq += 1;
    }
}
