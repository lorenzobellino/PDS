use clap::Parser;

/// simple program to slufigy a string passed to command line
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// string to slufigy
    #[clap(short, long)]
    string: String,
}

pub fn slufigy(s: &str) -> String {
    let mut slug: String = s.chars().map(conv).collect();
    if slug.len() > 1 {
        slug = slug.trim_end_matches('-').to_string();
        while slug.contains("--") {
            slug = slug.replace("--", "-");
        }
    }
    slug
}

fn conv(c: char) -> char {
    const SUBS_I: &str =
        "àáâäæãåāăąçćčđďèéêëēėęěğǵḧîïíīįìıİłḿñńǹňôöòóœøōõőṕŕřßśšşșťțûüùúūǘůűųẃẍÿýžźż";
    const SUBS_O: &str =
        "aaaaaaaaaacccddeeeeeeeegghiiiiiiiilmnnnnoooooooooprrsssssttuuuuuuuuuwxyyzzz";

    let mut converted: char = '-';
    if c.is_ascii_alphanumeric() {
        converted = c.to_ascii_lowercase();
    } else if SUBS_I.contains(c) {
        for (i, c_i) in SUBS_I.chars().enumerate() {
            if c == c_i {
                converted = SUBS_O.chars().nth(i).unwrap();
                break;
            }
        }
    }
    converted
}

// fn main() {
//     let args = Args::parse();
//     let slug = slufigy(&args.string);
//     println!("{}", slug);
// }
