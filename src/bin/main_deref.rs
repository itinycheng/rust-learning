use std::ops::Deref;

#[derive(Debug, Default, PartialEq, Eq)]
struct Person {
    name: String,
    age: u16,
}

#[derive(Debug, Default, PartialEq, Eq)]
struct Cooker {
    person: Person,
    favor: String,
}

trait Cook {
    fn cook(&self, food: String);
}

trait Action {
    fn action(&self) -> bool;
}

impl Deref for Cooker {
    type Target = Person;

    fn deref(&self) -> &Self::Target {
        &self.person
    }
}

impl<T: Deref<Target = Person>> Action for T {
    fn action(&self) -> bool {
        println!("{:?} action.", &self.name);
        true
    }
}

fn do_action(action: &dyn Action) -> bool {
    action.action()
}

fn main() {
    let cooker = Cooker {
        person: Person {
            name: "A".to_string(),
            age: 20,
        },
        ..Default::default()
    };
    let done = do_action(&cooker);
    println!("{:?}", done)
} 
