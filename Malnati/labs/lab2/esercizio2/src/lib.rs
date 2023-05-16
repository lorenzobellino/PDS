use std::time::{SystemTime, UNIX_EPOCH};

pub enum FileType {
    Text,
    Binary,
}

pub struct File {
    name: String,
    content: Vec<u8>,
    creation_time: u64,
    type_: FileType,
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

pub struct Dir {
    name: String,
    creation_time: u64,
    children: Vec<Node>,
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
}

pub enum Node {
    File(File),
    Node(Dir),
}

pub struct MatchResult<'a> {
    qs: Vec<&'a str>,
    matched_nodes: Vec<&'a mut Node>,
}

pub struct Filesystem {
    root: Dir,
}

impl Filesystem {
    fn new() -> Filesystem {
        Filesystem {
            root: Dir::new("root"),
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn exploration() {
        assert_eq!(2 + 2, 4);
    }
}
