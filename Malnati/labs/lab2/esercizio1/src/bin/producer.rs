use std::{
    thread::sleep,
    time::{Duration, Instant},
};

use esercizio1::{my_time, RingBuf, SensorData};

fn main() {
    let mut seq: u32 = 0;
    let mut buf = RingBuf::new("biffer.bin", 10);

    let start = Instant::now();

    loop {
        let data = SensorData {
            seq: seq,
            values: [0.0; 10],
            timestamp: my_time(),
        };
        if let Some(_) = buf.write(data) {
            println!("producer: worte {}", seq);
        } else {
            println!("producer: buffer full");
        }

        let elapsed = start.elapsed().as_millis() as i64;
        let drift = elapsed - 1000 * seq as i64;
        sleep(Duration::from_millis(
            (1000 - drift).try_into().unwrap_or(0),
        ));
        seq += 1
    }
}
