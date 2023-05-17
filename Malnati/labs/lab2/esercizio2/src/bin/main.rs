// use esercizio2::run;
// use std::path::Path;
// use std::{env, fs};

use esercizio2::{File, FileType, Filesystem};
fn main() {
    let filename = "test.txt";
    let content = vec![1, 2, 3, 4];
    let type_ = FileType::Text;
    let file = File::new(filename, content, type_);
    let mut fs = Filesystem::from_dir("/a").unwrap();
    fs.new_file("/a/test.txt", file);
    let d = fs.get_file("/a/test.txt");
    println!("{:?}", d);
}
