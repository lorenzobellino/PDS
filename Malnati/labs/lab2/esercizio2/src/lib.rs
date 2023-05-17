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
            // println!("d: {}", d);
            if *d == "" {
                continue;
            }
            let new_dir = Dir::new(d);
            cdir.children.push(Node::Dir(new_dir));
            // println!("ls {} - {:?}", cdir.name, cdir.ls());
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
        // let mut new_dir = Dir::new(dirname);
        // parent.children.push(Node::Dir(new_dir));
        // Some(parent)
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
        println!("parent: {:?}", parent);
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
                "contents" => {
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

#[cfg(test)]
mod tests {

    use super::*;
    #[test]
    fn create_empty_fs() {
        let fs = Filesystem::new();
        assert_eq!(fs.root.name, "/");
    }

    #[test]
    fn create_fs_from_dir() {
        let mut fs = Filesystem::from_dir("/a/b/c");
        assert!(fs.is_some());
        assert_eq!(fs.as_mut().unwrap().ls_dir("/").unwrap().len(), 1);
        assert_eq!(fs.as_mut().unwrap().ls_dir("/a").unwrap().len(), 1);
        assert_eq!(fs.as_mut().unwrap().ls_dir("/a/b").unwrap().len(), 1);
        assert_eq!(fs.as_mut().unwrap().ls_dir("/a/b/c").unwrap().len(), 0);
    }

    #[test]
    fn create_dir() {
        let mut fs = Filesystem::new();
        let dir = fs.mk_dir("/a");
        assert!(dir.is_some());
        assert_eq!(dir.as_ref().unwrap().name, "/");
    }

    #[test]
    fn delete_dir() {
        let mut fs = Filesystem::from_dir("/a/b").unwrap();
        let dir = fs.rm_dir("/a/b");
        assert!(dir.is_some());
    }

    #[test]
    fn create_multiple_dir() {
        let mut fs = Filesystem::new();
        let dir1 = fs.mk_dir("/a");
        assert!(dir1.is_some());
        let dir2 = fs.mk_dir("/b");
        assert!(dir2.is_some());
        assert_eq!(fs.root.children.len(), 2);
    }

    #[test]
    fn create_nested_dir() {
        let mut fs = Filesystem::new();
        let dir1 = fs.mk_dir("/a");
        assert!(dir1.is_some());
        assert_eq!(dir1.as_ref().unwrap().name, "/");
        let dir2 = fs.mk_dir("/a/b");
        assert!(dir2.is_some());
        assert_eq!(dir2.as_ref().unwrap().name, "a");
    }

    #[test]
    fn invalid_dir_path() {
        let mut fs = Filesystem::new();
        let d = fs.mk_dir("/a/b");
        assert!(d.is_none());
    }

    #[test]
    fn create_file() {
        let filename = "test.txt";
        let content = "test file content".into();
        let type_ = FileType::Text;
        let file = File::new(filename, content, type_);
        let mut fs = Filesystem::new();
        let dir = fs.mk_dir("/a");
        assert!(dir.is_some());
        let dir2 = fs.new_file("/a/test.txt", file);
        assert!(dir2.is_some());
    }

    #[test]
    fn create_file_invalid_path() {
        let filename = "test.txt";
        let content = "test file content".into();
        let type_ = FileType::Text;
        let file = File::new(filename, content, type_);
        let mut fs = Filesystem::from_dir("/a");
        let dir = fs.as_mut().unwrap().new_file("/a/test.txt", file);
        assert!(dir.is_some());
    }

    #[test]
    fn delete_file() {
        let filename = "test.txt";
        let content = "test file content".into();
        let type_ = FileType::Text;
        let file = File::new(filename, content, type_);
        let mut fs = Filesystem::from_dir("/a").unwrap();
        fs.new_file("/a/test.txt", file);
        let dir2 = fs.rm_file("/a/test.txt");
        assert!(dir2.is_some());
    }

    #[test]
    fn get_file() {
        let filename = "test.txt";
        let content = "test file content".into();
        let type_ = FileType::Text;
        let file = File::new(filename, content, type_);
        let mut fs = Filesystem::from_dir("/a").unwrap();
        fs.new_file("/a/test.txt", file);
        let d = fs.get_file("/a/test.txt");
        assert!(d.is_some());
    }

    #[test]
    fn get_invalid_file() {
        let filename = "test.txt";
        let content = "test file content".into();
        let type_ = FileType::Text;
        let file = File::new(filename, content, type_);
        let mut fs = Filesystem::from_dir("/a").unwrap();
        fs.new_file("/a/test.txt", file);
        let d = fs.get_file("/a/invalid.txt");
        assert!(d.is_none());
    }
}
