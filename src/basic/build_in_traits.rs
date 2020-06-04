use std::borrow::{BorrowMut, Cow};

#[derive(Debug)]
struct Toy {
    name: String,
}

impl Toy {
    fn new<T: Into<String>>(name: T) -> Toy {
        Toy { name: name.into() }
    }
}

// U -> &T
impl AsRef<String> for Toy {
    fn as_ref(&self) -> &String {
        &self.name
    }
}

// U -> &mut T
impl AsMut<String> for Toy {
    fn as_mut(&mut self) -> &mut String {
        &mut self.name
    }
}

impl From<&str> for Toy {
    fn from(name: &str) -> Self {
        Toy {
            name: name.to_string(),
        }
    }
}

impl Into<String> for Toy {
    fn into(self) -> String {
        self.name
    }
}

pub fn proc_build_in_traits() {
    let car = Toy::from("car");
    println!("toy from str: {:?}", car);
    let car2: Toy = "car2".into();
    println!("string into Toy: {:?}", car2);
    let name: String = car.into();
    println!("toy into name: {}", name);
    let mut airplane = Toy::new("airplane");
    let name = airplane.as_ref();
    println!("as_ref toy name: {}", name);
    let name = airplane.as_mut();
    name.push('~');
    println!("as_mut toy name: {}", name);
    let airplane1 = airplane.borrow_mut();
    println!("{:?}", airplane1);
    cow();
    // abs
    abs(&mut Cow::Owned(vec![-1, -2, -3]));
}

// copy-on-write
fn cow() {
    let mut cow: Cow<[_]> = Cow::Owned(vec![1, 2, 3]);
    let cow_v1 = cow.to_mut();
    cow_v1.push(4);
    println!("cow v1 {:?}", cow_v1);
    let cow_v2 = cow.into_owned();
    println!("cow v2 {:?}", cow_v2);
    let no_space = rm_space("ti ny");
    println!("no_space {:?}", no_space);
}

fn abs(cow: &mut Cow<[i32]>) {
    for i in 0..cow.len() {
        let v = cow[i];
        if v < 0 {
            cow.to_mut()[i] = -v;
        }
    }
    println!("cow {:?}", cow);
}

fn rm_space(input: &str) -> Cow<str> {
    if input.contains(' ') {
        let mut buf = String::with_capacity(input.len());
        for c in input.chars() {
            if c != ' ' {
                buf.push(c);
            }
        }
        return Cow::Owned(buf);
    }
    return Cow::Borrowed(input);
}
