use std::ops::{Add, Deref, DerefMut, Mul};

#[derive(Debug)]
struct Person {
    name: String,
    age: i32,
}

#[derive(Debug)]
struct Square<T> {
    x: T,
    y: T,
    r: T,
}

trait Area<T> {
    fn area(&self) -> T;
}

impl<T> Area<T> for Square<T>
where
    T: Mul<Output = T> + Copy,
{
    fn area(&self) -> T {
        self.r * self.r
    }
}

impl Deref for Person {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.name
    }
}

impl DerefMut for Person {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.name
    }
}

impl Add for Person {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let mut new_name = String::from(self.name);
        new_name.push_str(&rhs.name);
        Person {
            name: new_name,
            age: self.age + rhs.age,
        }
    }
}

pub fn proc_operate() {
    let mut person = Person {
        name: "tiny".to_string(),
        age: 18,
    };
    println!("original person {:?}", person);
    *person = "deref_mut".to_string();
    println!("deref_mut person {:?}", person);
    let p1 = Person {
        name: "tiny".to_string(),
        age: 18,
    };
    let p2 = Person {
        name: "tiny".to_string(),
        age: 18,
    };
    let p3 = p1 + p2;
    println!("person added name: {}, age: {}", p3.name, p3.age);
    let square = Square {
        x: 0_f32,
        y: 0f32,
        r: 3.2f32,
    };
    println!("area of {:?} is {}", square, square.area());
}
