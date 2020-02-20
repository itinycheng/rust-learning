use std::fmt::{Debug, Error, Formatter};

trait PrintSomething {
    fn gene(&self) -> &str;
    fn print(&self) {
        println!("gene is {}", self.gene());
    }
}

trait HasArea: PrintSomething {
    fn area(&self) -> f64;
}

trait HasGirth {
    fn girth(&self) -> f64;
}

struct Square {
    pub x: i32,
    pub y: i32,
    pub side: f64,
}

impl Debug for Square {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(
            f,
            "Square {{ x:{}, y:{}, side:{} }}",
            self.x, self.y, self.side
        )
    }
}

impl HasGirth for f64 {
    fn girth(&self) -> f64 {
        *self as f64
    }
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

fn get_area_girth<T: HasArea + HasGirth>(shape: &T) -> (f64, f64) {
    (shape.area(), shape.girth())
}

fn sum_girth(shape_a: &impl HasGirth, shape_b: &(impl HasArea + HasGirth)) -> f64 {
    println!("traits area of shape_b is {}", shape_b.area());
    shape_a.girth() + shape_b.girth()
}

fn sum_area<T, K>(shape_a: &T, shape_b: &K) -> f64
where
    T: HasArea,
    K: HasArea + HasGirth,
{
    println!("traits: girth of shape_b is {}", shape_b.girth());
    shape_a.area() + shape_b.area()
}

fn combine(shape_a: &Square, shape_b: &Square) -> impl HasArea {
    Square {
        x: shape_a.x + shape_b.x,
        y: shape_a.y + shape_b.y,
        side: shape_a.side + shape_b.side,
    }
}

fn rtn_closure() -> impl Fn(&Square) -> f64 {
    |square: &Square| square.area()
}

fn rtn_func() -> fn(&Square) -> f64 {
    |square: &Square| square.area()
}

// `impl Trait`包含两个指针，一个指向实例对象，一个指向虚函数表的函数实现
pub fn proc_trait() {
    println!("f64 girth {}", 5f64.girth());
    let square = &Square {
        x: 1,
        y: 2,
        side: 5f64,
    };
    println!("square debug {:?}", square);
    let (area, girth) = get_area_girth(square);
    let sum_area = sum_area(square, square);
    println!("square area={}, girth={}, sum={}", area, girth, sum_area);
    let sum_girth = sum_girth(square, square);
    println!("sum_girth {}", sum_girth);
    let comb = combine(square, square);
    println!("combine square as HasArea {}", comb.area());
    let rtn_closure = rtn_closure()(square);
    let rtn_func = rtn_func()(square);
    println!("rtn_closure={}, rtn_func={}", rtn_closure, rtn_func);
}
