use clap::Parser;

/// simple program to slufigy a string passed to command line
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// string to slufigy
    #[clap(short, long)]
    string: String,
}

fn slufigy(s: &str) -> String {
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

fn main() {
    let args = Args::parse();
    let slug = slufigy(&args.string);
    println!("{}", slug);
}

#[cfg(test)]
mod tests_sliufigy {
    use super::*;

    #[test]
    fn test_hello() {
        let s1 = "Hello World!";
        assert!(slufigy(s1) == "hello-world");
    }

    #[test]
    fn test_single_char() {
        let s1 = "]";
        assert!(slufigy(s1) == "-");
    }

    #[test]
    fn test_empty() {
        let s1 = "";
        assert!(slufigy(s1) == "");
    }

    #[test]
    fn test_uppercase() {
        let s1 = "HELLO WORLD!";
        assert!(slufigy(s1) == "hello-world");
    }

    #[test]
    fn test_special_chars() {
        let s1 = "Hello,World!";
        assert!(slufigy(s1) == "hello-world");
    }

    #[test]
    fn test_multiple_words() {
        let s = "Hello World! This is a test with multiple words.";
        assert!(slufigy(s) == "hello-world-this-is-a-test-with-multiple-words");
    }

    #[test]
    fn test_multiple_words_with_special_chars() {
        let s = "testing multiple WORDS with special chars like ç, ã, õ, è, ù, ñ, ï";
        assert!(slufigy(s) == "testing-multiple-words-with-special-chars-like-c-a-o-e-u-n-i");
    }

    #[test]
    fn test_all_subs() {
        let s = "àáâäæãåāăąçćčđďèéêëēėęěğǵḧîïíīįìıİłḿñńǹňôöòóœøōõőṕŕřßśšşșťțûüùúūǘůűųẃẍÿýžźż";
        assert!(
            slufigy(s)
                == "aaaaaaaaaacccddeeeeeeeegghiiiiiiiilmnnnnoooooooooprrsssssttuuuuuuuuuwxyyzzz"
        );
    }

    #[test]
    fn test_white_string() {
        let s = "      ";
        assert!(slufigy(s) == "");
    }

    #[test]
    fn test_multiple_space_non_valid_char() {
        let s = "&    % ) a      $L";
        assert!(slufigy(s) == "-a-l");
    }

    #[test]
    fn test_only_invalid_char() {
        let s = "$%£()/=-.,*+";
        assert!(slufigy(s) == "");
    }
}
