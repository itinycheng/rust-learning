mod basic;

use crate::basic::traits::{Square, get_area_girth, sum_area};
use crate::basic::file::{read_file, write_to_file};
use crate::basic::expr::proc_expr;
use crate::basic::datatype::proc_data;
use crate::basic::operator::operate;
use crate::basic::lifetime::life_time;
use crate::basic::function::func;

const FILE_PATH: &'static str = "/Users/tiny/Applications/IdeaProjects/rust-learning/.gitignore";

#[allow(dead_code)]
fn main() {
    // data type
    println!("-------------data type----------");
    proc_data();
    // trait
    println!("-------------trait----------");
    let square = &Square { x: 1, y: 2, side: 5f64 };
    let (area, girth) = get_area_girth(square);
    let sum_area = sum_area(square, square);
    println!("square area={}, girth={}, sum={}", area, girth, sum_area);
    // file
    println!("-------------file----------");
    read_file(FILE_PATH);
    write_to_file(FILE_PATH);
    // expr
    println!("-------------expr----------");
    proc_expr();
    println!("-------------operator----------");
    operate();
    println!("-------------lifetime----------");
    life_time();
    println!("-------------function----------");
    func();
    println!("-------------other----------");
    // 0
    match_int(6);
    match_tuple((1, 1));

    // function
    let plus_num = |x: i32| x + 2;
    println!("plus_num, {}", plus_num(5));

    let mut origin_num = 5;
    {
        let mut plus_num_n = move |i: i32| origin_num += i;
        plus_num_n(3);
        println!("inner plus_num_n, {}", origin_num);
    }
    assert_eq!(5, origin_num);

    let multi_opr = |x: i32, y: i32| x * y;
    println!("multi_opr, {}", apply(multi_opr, 2));

    let box_fn = factory(2);
    println!("box_fn, {}", box_fn(2));

}


fn factory(x: i32) -> Box<dyn Fn(i32) -> i32> {
    Box::new(move |y| x + y)
}

fn apply<F>(f: F, y: i32) -> i32
    where F: Fn(i32, i32) -> i32
{
    f(y, y)
}

fn match_int(i: i32) {
    match i {
        e @ 1..=5 => println!("match_int: weekday = {}", e),
        6 | 7 => println!("match_int: weekend = {}", i),
        n if n > 0 => println!("match_int: positive = {}", n),
        _ => println!("match_int: invalid")
    }
}

fn match_tuple(tuple: (i32, i32)) {
    match tuple {
        (0, y) => println!("match_tuple: tuple x=0, y={}", y),
        (x, 0) => println!("match_tuple: tuple x={}, y=0", x),
        _ => println!("match_tuple: tuple x={}, y={}", tuple.0, tuple.1)
    }
}
