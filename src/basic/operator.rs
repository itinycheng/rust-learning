use std::ops::Deref;

struct Person<'a> {
    name: &'a str,
    age: i32,
}

impl<'a> Deref for Person<'a> {
    type Target = &'a str;

    fn deref(&self) -> &Self::Target {
        &self.name
    }
}

pub fn proc_operate() {
    let person = Person { name: "tiny", age: 18 };
    println!("person name:{}, age:{}", person.name, person.age);
    let p_str = *person;
    println!("deref person {}", p_str);
}