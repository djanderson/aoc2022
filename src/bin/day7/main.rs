/// Day 7: No Space Left On Device
use std::cell::RefCell;
use std::fs;
use std::rc::Rc;

pub fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let mut filesystem = FileSystem::new();
    let tokens = tokenize(&input);

    for token in tokens {
        match token {
            Token::Cmd(Cmd::Cd(dir)) => filesystem.cd(&dir),
            Token::Cmd(Cmd::Ls) => { /* noop */ }
            Token::File { name, size } => filesystem.add_file(name, size),
            Token::Directory { name } => filesystem.mkdir(name),
        }
    }

    let sum: usize = directory_sizes(filesystem)
        .iter()
        .filter(|&&s| s < 100_000)
        .sum();
    println!("{}", sum);
}

fn directory_sizes(fs: FileSystem) -> Vec<usize> {
    let mut sizes: Vec<usize> = vec![];
    fn dsize(dir: Rc<RefCell<Directory>>, sizes: &mut Vec<usize>) {
        sizes.push(dir.borrow().size());
        for subdir in dir.borrow().subdirectories.iter() {
            dsize(Rc::clone(subdir), sizes);
        }
    }
    dsize(fs.root, &mut sizes);
    sizes
}

fn tokenize(input: &str) -> Vec<Token> {
    input
        .lines()
        .map(|line| match line {
            l if l.starts_with('$') => match &l[2..4] {
                "cd" => Token::Cmd(Cmd::Cd(String::from(&l[5..]))),
                "ls" => Token::Cmd(Cmd::Ls),
                &_ => panic!("Invalid command token {}", &l[2..]),
            },
            l if l.starts_with("dir") => Token::Directory {
                name: String::from(&l[4..]),
            },
            file => {
                let (size, name) = file.split_once(' ').unwrap();
                Token::File {
                    name: name.to_string(),
                    size: size.parse().unwrap(),
                }
            }
        })
        .collect()
}

#[derive(Debug)]
enum Token {
    Cmd(Cmd),
    File { name: String, size: usize },
    Directory { name: String },
}

#[derive(Debug)]
enum Cmd {
    Cd(String),
    Ls,
}

struct FileSystem {
    root: Rc<RefCell<Directory>>,
    current: Rc<RefCell<Directory>>,
}

impl FileSystem {
    pub fn new() -> FileSystem {
        let root = Rc::new(RefCell::new(Directory::new()));
        let current = Rc::clone(&root);
        FileSystem { root, current }
    }

    pub fn mkdir(&mut self, name: String) {
        let mut current = self.current.borrow_mut();

        let dir = Rc::new(RefCell::new(Directory {
            name,
            parent: Some(Rc::clone(&self.current)),
            subdirectories: vec![],
            files: vec![],
        }));

        current.subdirectories.push(dir);
    }

    pub fn cd(&mut self, dir: &str) {
        match dir {
            "/" => {
                self.current = Rc::clone(&self.root);
            }
            ".." => {
                if self.current.borrow().name == "/" {
                    return; // already at root, do nothing
                }
                let current = Rc::clone(&self.current);
                self.current = Rc::clone(current.borrow().parent.as_ref().unwrap());
            }
            _ => {
                let newdir = Rc::clone(
                    self.current
                        .borrow()
                        .subdirectories
                        .iter()
                        .find(|d| d.borrow().name == dir)
                        .unwrap(),
                ); // TODO: panics if dir does not exist
                self.current = newdir;
            }
        }
    }

    pub fn add_file(&mut self, name: String, size: usize) {
        self.current.borrow_mut().files.push(File { name, size });
    }
}

#[derive(Debug, PartialEq)]
struct File {
    name: String,
    size: usize,
}

#[derive(Debug)]
struct Directory {
    name: String,
    parent: Option<Rc<RefCell<Directory>>>,
    subdirectories: Vec<Rc<RefCell<Directory>>>,
    files: Vec<File>,
}

impl Directory {
    /// Create a root directory
    pub fn new() -> Directory {
        Directory {
            name: String::from("/"),
            parent: None,
            subdirectories: vec![],
            files: vec![],
        }
    }

    pub fn size(&self) -> usize {
        let files_size: usize = self.files.iter().map(|f| f.size).sum();
        let dirs_size: usize = self.subdirectories.iter().map(|d| d.borrow().size()).sum();
        files_size + dirs_size
    }
}
