// use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};
// use std::{env, fs};

#[derive(Debug)]
pub enum FileType {
    Text,
    Binary,
}

#[derive(Debug)]
pub struct File {
    pub name: String,
    pub content: Vec<u8>,
    pub creation_time: u64,
    pub type_: FileType,
}

impl File {
    pub fn new(name: &str, content: Vec<u8>, type_: FileType) -> Self {
        let t = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        File {
            name: String::from(name),
            content,
            creation_time: t,
            type_,
        }
    }
}

#[derive(Debug)]
pub struct Dir {
    pub name: String,
    pub creation_time: u64,
    pub children: Vec<Node>,
}

impl Dir {
    pub fn new(name: &str) -> Self {
        let t = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        Dir {
            name: String::from(name),
            creation_time: t,
            children: vec![],
        }
    }

    pub fn ls(&self) -> Vec<&Node> {
        self.children.iter().collect()
    }
}

#[derive(Debug)]
pub enum Node {
    File(File),
    Dir(Dir),
}

pub struct MatchResult<'a> {
    pub qs: Vec<&'a str>,
    pub matched_nodes: Vec<&'a mut Node>,
}

pub struct Filesystem {
    pub root: Dir,
}

impl Filesystem {
    pub fn new() -> Filesystem {
        Filesystem {
            root: Dir::new("/"),
        }
    }

    pub fn ls_dir(&mut self, path: &str) -> Option<Vec<&Node>> {
        let dirs = path.split("/").collect::<Vec<&str>>();
        let mut cdir = &mut self.root;
        if dirs.len() == 1 {
            return Some(cdir.ls());
        }
        for d in dirs.iter() {
            if *d == "" {
                continue;
            }
            let n = cdir.children.iter_mut().find(|x| match x {
                Node::Dir(x) => x.name == *d,
                _ => false,
            });
            match n {
                Some(Node::Dir(ref mut x)) => {
                    cdir = x;
                }
                _ => return None,
            }
        }
        Some(cdir.ls())
    }

    pub fn from_dir(path: &str) -> Option<Filesystem> {
        let mut fs = Filesystem::new();
        let mut cdir = &mut fs.root;
        let dirs = path.split("/").collect::<Vec<&str>>();
        for d in dirs.iter() {
            if *d == "" {
                continue;
            }
            let new_dir = Dir::new(d);
            cdir.children.push(Node::Dir(new_dir));
            let n = cdir.children.iter_mut().find(|x| match x {
                Node::Dir(x) => x.name == *d,
                _ => false,
            });

            match n {
                Some(Node::Dir(ref mut x)) => {
                    cdir = x;
                }
                _ => return None,
            }
        }
        Some(fs)
    }

    fn find_dir(&mut self, path: &str) -> Option<&mut Dir> {
        let dirs = path.split("/").collect::<Vec<&str>>();
        let mut cdir = &mut self.root;
        if dirs.len() == 1 {
            return Some(cdir);
        }
        for d in dirs.iter() {
            if *d == "" {
                continue;
            }
            let n = cdir.children.iter_mut().find(|x| match x {
                Node::Dir(x) => x.name == *d,
                _ => false,
            });
            match n {
                Some(Node::Dir(ref mut x)) => {
                    cdir = x;
                }
                _ => return None,
            }
        }
        Some(cdir)
    }

    pub fn mk_dir(&mut self, path: &str) -> Option<&mut Dir> {
        let dirname = path.split("/").last().unwrap();
        let basepath = path
            .split("/")
            .take_while(|x| *x != dirname)
            .collect::<Vec<&str>>()
            .join("/");
        let parent = self.find_dir(&basepath);
        match parent {
            Some(p) => {
                let new_dir = Dir::new(dirname);
                p.children.push(Node::Dir(new_dir));
                Some(p)
            }
            None => None,
        }
    }

    pub fn rm_dir(&mut self, path: &str) -> Option<&mut Dir> {
        let dirname = path.split("/").last().unwrap();
        let basepath = path
            .split("/")
            .take_while(|x| *x != dirname)
            .collect::<Vec<&str>>()
            .join("/");
        let parent = self.find_dir(&basepath);
        match parent {
            Some(x) => {
                x.children.retain(|x| match x {
                    Node::Dir(x) => x.name != *dirname,
                    _ => true,
                });
                Some(x)
            }
            None => None,
        }
    }

    pub fn new_file(&mut self, path: &str, file: File) -> Option<&mut Dir> {
        let filename = path.split("/").last().unwrap();
        let basepath = path
            .split("/")
            .take_while(|x| *x != filename)
            .collect::<Vec<&str>>()
            .join("/");
        let parent = self.find_dir(&basepath);
        // println!("parent: {:?}", parent);
        match parent {
            Some(x) => {
                x.children.push(Node::File(file));
                Some(x)
            }
            None => None,
        }
    }

    pub fn rm_file(&mut self, path: &str) -> Option<&mut Dir> {
        let filename = path.split("/").last().unwrap();
        let basepath = path
            .split("/")
            .take_while(|x| *x != filename)
            .collect::<Vec<&str>>()
            .join("/");
        let parent = self.find_dir(&basepath);
        match parent {
            Some(x) => {
                x.children.retain(|x| match x {
                    Node::File(x) => x.name != *filename,
                    _ => true,
                });
                Some(x)
            }
            None => None,
        }
    }
    pub fn get_file(&mut self, path: &str) -> Option<&mut File> {
        let filename = path.split("/").last().unwrap();
        let basepath = path
            .split("/")
            .take_while(|x| *x != filename)
            .collect::<Vec<&str>>()
            .join("/");
        let parent = self.find_dir(&basepath);
        match parent {
            Some(x) => {
                let f = x.children.iter_mut().find(|x| match x {
                    Node::File(x) => x.name == *filename,
                    _ => false,
                });
                match f {
                    Some(Node::File(ref mut x)) => Some(x),
                    _ => None,
                }
            }
            None => None,
        }
    }

    fn do_match<'a>(f: &File, query: &'a [&'a str]) -> Option<Vec<&'a str>> {
        let mut matched = vec![];
        for q in query {
            let toks = q.split(":").collect::<Vec<&str>>();
            let qtype = toks[0];
            let qval = toks[1];
            match qtype {
                "name" => {
                    if f.name.contains(qval) {
                        matched.push(*q);
                    }
                }
                "content" => {
                    if String::from_utf8_lossy(&f.content).contains(qval) {
                        matched.push(*q);
                    }
                }
                "larger" => {
                    let qval = qval.parse::<usize>().unwrap();
                    if f.content.len() > qval {
                        matched.push(*q);
                    }
                }
                "smaller" => {
                    let qval = qval.parse::<usize>().unwrap();
                    if f.content.len() < qval {
                        matched.push(*q);
                    }
                }
                "newer" => {
                    let qval = qval.parse::<u64>().unwrap();
                    if f.creation_time > qval {
                        matched.push(*q);
                    }
                }
                "older" => {
                    let qval = qval.parse::<u64>().unwrap();
                    if f.creation_time < qval {
                        matched.push(*q);
                    }
                }
                _ => println!("invalid query"),
            }
        }
        if matched.len() == 0 {
            return None;
        }
        Some(matched)
    }

    pub fn search<'a>(&'a mut self, query: &'a [&'a str]) -> Option<MatchResult> {
        let mut mr = MatchResult {
            qs: vec![],
            matched_nodes: vec![],
        };
        let mut visits = vec![&mut self.root];
        while let Some(d) = visits.pop() {
            for cc in d.children.iter_mut() {
                match cc {
                    Node::Dir(ref mut x) => {
                        visits.push(x);
                    }
                    Node::File(ref mut x) => {
                        if let Some(matches) = Filesystem::do_match(x, query) {
                            for m in matches {
                                if !mr.qs.contains(&m) {
                                    mr.qs.push(m);
                                }
                            }
                            mr.matched_nodes.push(cc);
                        }
                    }
                }
            }
        }
        Some(mr)
    }
}
