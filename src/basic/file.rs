use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use std::path::Path;

const INPUT_TEXT: &str = "input text";

const FILE_PATH: &str = "/Users/tiny/Applications/IdeaProjects/rust-learning/.gitignore";

pub fn proc_file() {
    read_file(FILE_PATH);
    write_to_file(FILE_PATH);
    open_option(FILE_PATH)
}

fn read_file(path: &str) {
    let path = Path::new(path);
    let display = path.display();
    let mut file = match File::open(path) {
        Err(why) => panic!("couldn't open {}: {}", display, why.to_string()),
        Ok(file) => file,
    };
    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read {}: {}", display, why.to_string()),
        Ok(_) => println!("{} content:\n {}", display, s),
    }
}

fn write_to_file(path: &str) {
    let path = Path::new(path);
    let display = path.display();
    let mut file = match File::create(path) {
        Err(why) => panic!("couldn't create {}: {}", display, why.to_string()),
        Ok(file) => file,
    };
    match file.write_all(INPUT_TEXT.as_bytes()) {
        Err(why) => panic!(
            "couldn't write {} to {}: {}",
            INPUT_TEXT,
            display,
            why.to_string()
        ),
        Ok(_) => println!("write to {} successfully", display),
    }
}

fn open_option(path: &str) {
    let mut buf = String::new();
    let size = OpenOptions::new()
        .read(true)
        .open(Path::new(path))
        .unwrap()
        .read_to_string(&mut buf)
        .unwrap_or(0);
    println!("open option {} {}", size, buf);
}
