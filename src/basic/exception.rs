use std::fs::File;
use std::path::Path;
use std::io::Read;

pub fn proc_exception() {
    let file_name = "file.rs";
    match find(file_name, '.') {
        Some(x) => println!("find {} at position {}", ".", x),
        None => println!("no found")
    }

    let result = find(file_name, '.')
        .ok_or("not found".to_owned())
        // .map_err() handle err
        .and_then(|x| Ok((x * 10 + x).to_string()))
        .unwrap();
    println!("convert Oprtion to Result {}", result);

    let parsed = "2".parse::<i32>().unwrap();
    println!("parse number result {}", parsed);

    match file_double("a.rd") {
        Ok(x) => println!("get result succeed {}", x),
        Err(_) => println!("err found")
    }
}

// TODO ? vs try!
//fn file_double_2(path: &str) -> Result<i32, String> {
//    let mut file = File::open(Path::new(path))?;
//    let mut buf = String::new();
//    let byte_size = file.read_to_string(&mut buf)?;
//    Ok((2 * byte_size) as i32)
//}

fn file_double(path: &str) -> Result<i32, String> {
    File::open(Path::new(path))
        .map_err(|err| err.to_string())
        .and_then(|mut file| {
            let mut contents = String::new();
            file.read_to_string(&mut contents)
                .map_err(|err| err.to_string())
                .map(|_| contents)
        }).and_then(|contents| {
        contents.trim().parse::<i32>()
            .map_err(|err| err.to_string())
    }).map(|n| 2 * n)
}

fn find(src: &str, needle: char) -> Option<usize> {
    src.char_indices()
        .filter(|&(_, ch)| ch == needle)
        .map(|(offset, _)| offset)
        .nth(0)
}