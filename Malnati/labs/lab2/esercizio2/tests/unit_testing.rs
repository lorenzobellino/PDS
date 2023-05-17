#[cfg(test)]
mod tests {

    use esercizio2::*;

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

    #[test]
    fn match_names() {
        let mut fs = Filesystem::from_dir("/a/b/c").unwrap();
        fs.mk_dir("/a/b/d");
        fs.mk_dir("/a/k");
        let file1 = File::new("test1.txt", "test file content 111".into(), FileType::Text);
        let file2 = File::new("test2.txt", "test file content 222".into(), FileType::Text);
        let file3 = File::new("test3.txt", "test file content 333".into(), FileType::Text);
        fs.new_file("/a/b/test1.txt", file1);
        fs.new_file("/a/test2.txt", file2);
        fs.new_file("/a/k/test3.txt", file3);

        let q1 = ["name:test"];
        let matches = fs.search(&q1).unwrap();
        assert_eq!(matches.matched_nodes.len(), 3);
        let mut names = vec![];
        for i in matches.matched_nodes {
            match i {
                Node::File(f) => names.push(f.name.as_str()),
                _ => (),
            }
        }
        names.sort();
        assert_eq!(names, vec!["test1.txt", "test2.txt", "test3.txt"]);
    }

    #[test]
    fn match_exact_name() {
        let mut fs = Filesystem::from_dir("/a/b/c").unwrap();
        fs.mk_dir("/a/b/d");
        fs.mk_dir("/a/k");
        let file1 = File::new("test1.txt", "test file content 111".into(), FileType::Text);
        let file2 = File::new("test2.txt", "test file content 222".into(), FileType::Text);
        let file3 = File::new("test3.txt", "test file content 333".into(), FileType::Text);
        fs.new_file("/a/b/test1.txt", file1);
        fs.new_file("/a/test2.txt", file2);
        fs.new_file("/a/k/test3.txt", file3);

        let q1 = ["name:test1.txt"];
        let matches = fs.search(&q1).unwrap();
        assert_eq!(matches.matched_nodes.len(), 1);
        let mut names = vec![];
        for i in matches.matched_nodes {
            match i {
                Node::File(f) => names.push(f.name.as_str()),
                _ => (),
            }
        }
        names.sort();
        assert_eq!(names, vec!["test1.txt"]);
    }

    #[test]
    fn match_invalid_name() {
        let mut fs = Filesystem::from_dir("/a/b/c").unwrap();
        fs.mk_dir("/a/b/d");
        fs.mk_dir("/a/k");
        let file1 = File::new("test1.txt", "test file content 111".into(), FileType::Text);
        let file2 = File::new("test2.txt", "test file content 222".into(), FileType::Text);
        let file3 = File::new("test3.txt", "test file content 333".into(), FileType::Text);
        fs.new_file("/a/b/test1.txt", file1);
        fs.new_file("/a/test2.txt", file2);
        fs.new_file("/a/k/test3.txt", file3);

        let q1 = ["name:invalid"];
        let matches = fs.search(&q1).unwrap();
        assert_eq!(matches.matched_nodes.len(), 0);
    }

    #[test]
    fn match_content() {
        let mut fs = Filesystem::from_dir("/a/b/c").unwrap();
        fs.mk_dir("/a/b/d");
        fs.mk_dir("/a/k");
        let file1 = File::new("test1.txt", "test file content 111".into(), FileType::Text);
        let file2 = File::new("test2.txt", "test file content 222".into(), FileType::Text);
        let file3 = File::new("test3.txt", "test file content 333".into(), FileType::Text);
        fs.new_file("/a/b/test1.txt", file1);
        fs.new_file("/a/test2.txt", file2);
        fs.new_file("/a/k/test3.txt", file3);

        let q1 = ["content:file"];
        let matches = fs.search(&q1).unwrap();
        assert_eq!(matches.matched_nodes.len(), 3);
        let mut names = vec![];
        for i in matches.matched_nodes {
            match i {
                Node::File(f) => names.push(f.name.as_str()),
                _ => (),
            }
        }
        names.sort();
        assert_eq!(names, vec!["test1.txt", "test2.txt", "test3.txt"]);
    }

    #[test]
    fn match_exact_content() {
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
        assert_eq!(matches.matched_nodes.len(), 1);
        let mut names = vec![];
        for i in matches.matched_nodes {
            match i {
                Node::File(f) => names.push(f.name.as_str()),
                _ => (),
            }
        }
        names.sort();
        assert_eq!(names, vec!["test1.txt"]);
    }

    #[test]
    fn match_invalid_content() {
        let mut fs = Filesystem::from_dir("/a/b/c").unwrap();
        fs.mk_dir("/a/b/d");
        fs.mk_dir("/a/k");
        let file1 = File::new("test1.txt", "test file content 111".into(), FileType::Text);
        let file2 = File::new("test2.txt", "test file content 222".into(), FileType::Text);
        let file3 = File::new("test3.txt", "test file content 333".into(), FileType::Text);
        fs.new_file("/a/b/test1.txt", file1);
        fs.new_file("/a/test2.txt", file2);
        fs.new_file("/a/k/test3.txt", file3);

        let q1 = ["content:invalid"];
        let matches = fs.search(&q1).unwrap();
        assert_eq!(matches.matched_nodes.len(), 0);
    }

    #[test]
    fn match_dimension_larger_20() {
        let mut fs = Filesystem::from_dir("/a/b/c").unwrap();
        fs.mk_dir("/a/b/d");
        fs.mk_dir("/a/k");
        let file1 = File::new("test1.txt", "test file content 111".into(), FileType::Text);
        let file2 = File::new("test2.txt", "test file content 222".into(), FileType::Text);
        let file3 = File::new("test3.txt", "test file content 333".into(), FileType::Text);
        fs.new_file("/a/b/test1.txt", file1);
        fs.new_file("/a/test2.txt", file2);
        fs.new_file("/a/k/test3.txt", file3);
        let q1 = ["larger:20"];
        let matches = fs.search(&q1).unwrap();
        assert_eq!(matches.matched_nodes.len(), 3);
        let mut names = vec![];
        for i in matches.matched_nodes {
            match i {
                Node::File(f) => names.push(f.name.as_str()),
                _ => (),
            }
        }
        names.sort();
        assert_eq!(names, vec!["test1.txt", "test2.txt", "test3.txt"]);
    }
    #[test]
    fn match_dimension_larger_1000() {
        let mut fs = Filesystem::from_dir("/a/b/c").unwrap();
        fs.mk_dir("/a/b/d");
        fs.mk_dir("/a/k");
        let file1 = File::new("test1.txt", "test file content 111".into(), FileType::Text);
        let file2 = File::new("test2.txt", "test file content 222".into(), FileType::Text);
        let file3 = File::new("test3.txt", "test file content 333".into(), FileType::Text);
        fs.new_file("/a/b/test1.txt", file1);
        fs.new_file("/a/test2.txt", file2);
        fs.new_file("/a/k/test3.txt", file3);
        let q1 = ["larger:1000"];
        let matches = fs.search(&q1).unwrap();
        assert_eq!(matches.matched_nodes.len(), 0);
    }

    #[test]
    fn match_dimension_larger_70() {
        let mut fs = Filesystem::from_dir("/a/b/c").unwrap();
        fs.mk_dir("/a/b/d");
        fs.mk_dir("/a/k");
        let file1 = File::new("test1.txt", "small".into(), FileType::Text);
        let file2 = File::new("test2.txt", "really really really really really really really really really really really really really long long long long long long long long file !!!!!".into(), FileType::Text);
        let file3 = File::new("test3.txt", "normal size file".into(), FileType::Text);
        fs.new_file("/a/b/test1.txt", file1);
        fs.new_file("/a/test2.txt", file2);
        fs.new_file("/a/k/test3.txt", file3);
        let q1 = ["larger:70"];
        let matches = fs.search(&q1).unwrap();
        assert_eq!(matches.matched_nodes.len(), 1);
        let mut names = vec![];
        for i in matches.matched_nodes {
            match i {
                Node::File(f) => names.push(f.name.as_str()),
                _ => (),
            }
        }
        names.sort();
        assert_eq!(names, vec!["test2.txt"]);
    }
}
