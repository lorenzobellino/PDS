use slufigy::slufigy;

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
        slufigy(s) == "aaaaaaaaaacccddeeeeeeeegghiiiiiiiilmnnnnoooooooooprrsssssttuuuuuuuuuwxyyzzz"
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
