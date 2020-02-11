pub trait PrintSomething {
    fn gene(&self) -> &str;
    fn print(&self) {
        println!("gene is {}", self.gene());
    }
}

pub trait HasArea: PrintSomething {
    fn area(&self) -> f64;
}

pub trait HasGirth {
    fn girth(&self) -> f64;
}

pub struct Square {
    pub x: i32,
    pub y: i32,
    pub side: f64,
}

impl PrintSomething for Square {
    fn gene(&self) -> &str {
        "Square"
    }
}

impl HasArea for Square {
    fn area(&self) -> f64 {
        self.side * self.side
    }
}

impl HasGirth for Square {
    fn girth(&self) -> f64 {
        4f64 * self.side
    }
}


pub fn get_area_girth<T: HasArea + HasGirth>(shape: &T) -> (f64, f64) {
    (shape.area(), shape.girth())
}

pub fn sum_area<T, K>(shape_a: &T, shape_b: &K) -> f64
    where T: HasArea,
          K: HasArea + HasGirth {
    println!("traits: girth of shape_b is {}", shape_b.girth());
    return shape_a.area() + shape_b.area();
}