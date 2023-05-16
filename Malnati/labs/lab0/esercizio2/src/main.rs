use clap::Parser;
use std::fs::File;
use std::io::BufReader;
use std::io::Read;

#[derive(Parser, Debug)]
struct Args {
    /// The file to read
    #[clap(short, long)]
    file: String,
}

#[derive(Copy, Clone, Debug)]
struct Cdata {
    type_s: u8,
    data: [u8; 64],
}

impl Cdata {
    fn from_file(&mut self, reader: &mut BufReader<File>) {
        let bytes_read = read_n(reader, 64);
        let mut d = [0; 64];
        d.copy_from_slice(&bytes_read);
        self.data = d;
        self.type_s = bytes_read[0];
    }

    fn new() -> Cdata {
        Cdata {
            type_s: u8::MAX,
            data: [0; 64],
        }
    }
}
fn read_n<R>(reader: R, bytes_to_read: u64) -> Vec<u8>
where
    R: Read,
{
    let mut buf = vec![];
    let mut chunk = reader.take(bytes_to_read);
    let n = chunk.read_to_end(&mut buf).expect("Didn't read enough");
    assert_eq!(bytes_to_read as usize, n);
    buf
}

fn main() {
    let args = Args::parse();
    const N: usize = 100;
    println!("file: {}", args.file);
    let mut reader = BufReader::new(File::open(args.file).unwrap());
    let mut arr_cdata = [Cdata::new(); N];

    for i in 0..N {
        arr_cdata[i].from_file(&mut reader);
        print!("arr_cdata[{}] = ", i);
        println!("{:?}", arr_cdata[i]);
    }
}
