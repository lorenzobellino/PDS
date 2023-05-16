use std::io::{Read, Seek, Write};

fn main() {
    let mut f = std::fs::OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open("test.txt")
        .unwrap();

    let mut rbuf = vec![0; 4];
    f.seek(std::io::SeekFrom::Start(0)).unwrap();
    f.read_exact(&mut rbuf).unwrap();
    println!("read: {:?}", rbuf);

    rbuf[3] += 1;
    f.seek(std::io::SeekFrom::Start(0)).unwrap();
    f.write(&rbuf).unwrap();

    rbuf[3] += 1;
    f.seek(std::io::SeekFrom::Start(0)).unwrap();
    f.write(&rbuf).unwrap();

    f.seek(std::io::SeekFrom::Start(0)).unwrap();
    f.read_exact(&mut rbuf).unwrap();
    println!("read: {:?}", rbuf);
}
