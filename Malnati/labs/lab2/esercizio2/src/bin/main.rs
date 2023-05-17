// use esercizio2::run;
// use std::path::Path;
// use std::{env, fs};

use esercizio2::{File, FileType, Filesystem, Node};
fn main() {
    let mut fs = Filesystem::from_dir("/a/b/c").unwrap();
    fs.mk_dir("/a/b/d");
    fs.mk_dir("/a/k");
    let file1 = File::new("test1.txt", "test file content 111".into(), FileType::Text);
    let file2 = File::new("test2.txt", "test file content 222".into(), FileType::Text);
    let file3 = File::new("test3.txt", "test file content 333".into(), FileType::Text);
    fs.new_file("/a/b/test1.txt", file1);
    fs.new_file("/a/test2.txt", file2);
    fs.new_file("/a/k/test3.txt", file3);

    let q1 = ["content:test file content 111"];
    let matches = fs.search(&q1).unwrap();
    // assert_eq!(matches.matched_nodes.len(), 3);
    let mut names = vec![];
    for i in matches.matched_nodes {
        match i {
            Node::File(f) => names.push(f.name.as_str()),
            _ => (),
        }
    }
    names.sort();
    println!("{:?}", names)
}
