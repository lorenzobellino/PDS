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

        // println!("ls fs - {:?}", fs.ls_dir("/"));

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

    pub fn rm_dir(&mut self, path: &str) -> Option<Dir> {
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
            }
            None => return None,
        }
        None
    }

    // pub fn new_file(path: &str, file: File) -> bool {
    //     todo!("Implement this function");
    // }

    // pub fn rm_file(path: &str) -> bool {
    //     todo!("Implement this function");
    // }

    // pub fn get_file(path: &str) -> Option<File> {
    //     todo!("Implement this function");
    // }

    // pub fn search(&self, query: &str) -> Vec<&Node> {
    //     todo!("Implement this function");
    // }
}

#[cfg(test)]
mod tests {

    // use std::fs;

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
        // if let Node::Dir(ref d) = fs.root.children[0] {
        //     assert_eq!(d.name, "a");
        // } else {
        //     assert!(false, "expected dir");
        // }
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

    // #[test]
    // fn create_fs_from_dir() {
    //     let fs = Filesystem::from_dir("./test_dirr");
    // }
}
