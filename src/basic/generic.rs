use std::ops::Add;

#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}

impl<T: Add<Output = T>> Add for Point<T> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

pub fn proc_generic() {
    let p_1 = Point { x: 1, y: 1 };
    let p_2 = Point { x: 2, y: 2 };
    let added = p_1 + p_2;
    println!(" added: {:?}", added);
}
