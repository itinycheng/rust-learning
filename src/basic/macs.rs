use crate::HelloMacro;
use my_proc_macro::hello_macro;

#[macro_export]
macro_rules! print_sth {
    ($x:ident) => {
        println!("print_sth {}", $x);
    };
}

macro_rules! vector {
    ($($x:expr),*) => {
        {
            let mut tmp = Vec::new();
            $(tmp.push($x);)*
            tmp
        }
    };
}

// find_min!(5, 2, 3) 展开过程
// std::cmp::min(5u32, find_min!(2u32 * 3, 4u32))
// std::cmp::min(5u32, std::cmp::min(2u32 * 3, find_min!(4u32)))
// std::cmp::min(5u32, std::cmp::min(2u32 * 3, 4u32))
macro_rules! find_min {
    ($x: expr) => {$x};
    ($x: expr,$($y:expr),+) => {
        std::cmp::min($x, find_min!($($y),+))
    }
}

#[derive(hello_macro)]
struct Test<T>(T);

pub fn proc_macro() {
    let a = "fd";
    print_sth!(a);
    let vec = vector!(1, 2, 3);
    println!("vec {:?}", vec);
    let min = find_min!(5, 2, 3);
    println!("min {}", min);

    let test = Test("TEST");
    test.hello()
}
