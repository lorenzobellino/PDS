use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// name of the person to greet
    #[arg(short, long)]
    name: String,
    /// count of times to greet
    #[arg(short, long, default_value_t = 1)]
    count: u8,
}

fn main() {
    let args = Args::parse();

    for _ in 0..args.count {
        println!("Hello {}!", args.name)
    }

    // let mut i = 0;
    // let test = loop {
    //     i += 1;
    //     if i == 2 {
    //         break true;
    //     }
    // };
    // if test {
    //     println!("test: {}", test);
    // }
    // added from wsl branch
}
