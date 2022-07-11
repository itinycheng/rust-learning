use lazy_static::lazy_static;
use rand::Rng;
use std::collections::HashSet;
use std::sync::RwLock;

lazy_static! {
    static ref NAMES_LOCK: RwLock<HashSet<String>> = RwLock::new(HashSet::new());
}

#[derive(Debug)]
pub struct Robot {
    name: String,
}

impl Drop for Robot {
    fn drop(&mut self) {
        let mut names = NAMES_LOCK.write().unwrap();
        names.remove(&self.name);
    }
}

impl Robot {
    pub fn new() -> Self {
        Robot {
            name: Self::new_name(),
        }
    }

    pub fn name(&self) -> &str {
        self.name.as_str()
    }

    pub fn reset_name(&mut self) {
        self.name = Self::new_name();
    }

    fn new_name() -> String {
        let mut rng = rand::thread_rng();
        let number: u16 = rng.gen_range(100..=1000);
        let name: String = (0..)
            .map(|_| rng.gen_range(b'A'..=b'Z' + 1) as char)
            .take(2)
            .collect::<String>();
        let name = format!("{}{}", name, number);
        let exist;
        {
            let read_lock = NAMES_LOCK.read().unwrap();
            exist = read_lock.contains(&name);
        }
        if exist {
            Self::new_name()
        } else {
            let mut write_lock = NAMES_LOCK.write().unwrap();
            write_lock.insert(name.to_owned());
            name
        }
    }
}
