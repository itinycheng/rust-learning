use std::cell::Cell;

use derive_more::{Display, From};

#[derive(From, Display)]
#[display(fmt = "({}, {})", prefix, age)]
pub struct Name {
    prefix: String,
    age: u8,
}

fn main() -> () {
    let c_num = '0' as u64;
    println!("{}", c_num);

    let mut arr = [1, 2, 3];
    let slice = &mut arr[..];
    println!("{:?}", slice);

    let name = Name {
        prefix: "da".to_string(),
        age: 123,
    };

    let box_name = Box::new(name);
    test_name(*box_name);

    let origin = Cell::new("adfa".to_string());
    origin.set(origin.take().replace("a", "d"));
    println!("{:?} ", origin.take());

    let mut vec = Vec::new();
    vec.extend([1, 2, 3]);
    vec.push(4);
}

fn test_name(name: Name) {
    println!("{}", name);
}
