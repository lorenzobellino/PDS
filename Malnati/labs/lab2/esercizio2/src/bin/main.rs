// use esercizio2::run;
// use std::path::Path;
// use std::{env, fs};

// use core::slice::SlicePattern;
use esercizio2::{File, FileType, Filesystem};
use std::thread::sleep;
use std::time::{SystemTime, UNIX_EPOCH};
fn main() {
    // let mut fs = Filesystem::from_dir("/a/b/c").unwrap();
    // fs.mk_dir("/a/b/d");
    // fs.mk_dir("/a/k");
    // let file1 = File::new("test1.txt", "small".into(), FileType::Text);
    // let file2 = File::new("test2.txt", "really really really really really really really really really really really really really long long long long long long long long file !!!!!".into(), FileType::Text);
    // let file3 = File::new("test3.txt", "normal size file".into(), FileType::Text);

    // // let files = fs.get_file("/a/b/test1.txt").unwrap();
    // // println!("time: {:?}", files.creation_time);

    // let sleeptime = std::time::Duration::from_secs(3);

    // fs.new_file("/a/b/test1.txt", file1);
    // fs.new_file("/a/test2.txt", file2);
    // fs.new_file("/a/k/test3.txt", file3);

    // sleep(sleeptime);

    // let t1 = SystemTime::now()
    //     .duration_since(UNIX_EPOCH)
    //     .unwrap()
    //     .as_secs() as i64;

    // // println!("time : {}", t1);

    // // // let q = [format!("newer:{}", t1).as_str()];
    // // // let q: Vec<String> = vec![format!("older:{}", t1)];
    // // let q = vec![format!("newer:{}", t1)];
    // // let q_str: Vec<&str> = q.iter().map(|s| s.as_str()).collect();

    // // let matches = fs.search(&q_str).unwrap();

    // // println!("matches: {:?}", matches.matched_nodes);

    // // println!("files: {:?}", files);
    // let f = fs.get_file("/a/b/test1.txt");
    // println!("{:?}", f);
    // println!("{:?}", t1);

    let sleeptime = std::time::Duration::from_secs(3);

    let _t = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();

    println!("t : {:?}", _t);

    sleep(sleeptime);

    let mut fs = Filesystem::from_dir("/a/b/c").unwrap();
    fs.mk_dir("/a/b/d");
    fs.mk_dir("/a/k");
    let file1 = File::new("test1.txt", "small".into(), FileType::Text);
    let file2 = File::new("test2.txt", "really really really really really really really really really really really really really long long long long long long long long file !!!!!".into(), FileType::Text);
    let file3 = File::new("test3.txt", "normal size file".into(), FileType::Text);
    fs.new_file("/a/b/test1.txt", file1);
    fs.new_file("/a/test2.txt", file2);
    fs.new_file("/a/k/test3.txt", file3);

    let q = vec![format!("newer:{}", _t)];
    let q_str: Vec<&str> = q.iter().map(|s| s.as_str()).collect();

    let matches = fs.search(&q_str).unwrap();

    assert_eq!(matches.matched_nodes.len(), 0);
}
