mod basic;

use crate::basic::build_in_traits::proc_build_in_traits;
use crate::basic::closure::proc_closure;
use crate::basic::collection::{proc_collection, proc_iterator};
use crate::basic::datatype::proc_data;
use crate::basic::exception::proc_exception;
use crate::basic::expr::proc_expr;
use crate::basic::file::proc_file;
use crate::basic::function::proc_func;
use crate::basic::generic::proc_generic;
use crate::basic::lifetime::proc_lifetime;
use crate::basic::macs::proc_macro;
use crate::basic::operator::proc_operate;
use crate::basic::reflect::proc_reflect;
use crate::basic::thread::proc_thread;
use crate::basic::traits::proc_trait;
use crate::basic::un_safe::proc_unsafe;

fn main() {
    // data type
    println!("-------------data type----------");
    proc_data();
    // trait
    println!("-------------trait----------");
    proc_trait();
    // file
    println!("-------------file----------");
    proc_file();
    // expr
    println!("-------------expr----------");
    proc_expr();
    println!("-------------operator----------");
    proc_operate();
    println!("-------------lifetime----------");
    proc_lifetime();
    println!("-------------function----------");
    proc_func();
    println!("-------------generic----------");
    proc_generic();
    println!("-------------closure----------");
    proc_closure();
    println!("-------------collection----------");
    proc_collection();
    proc_iterator();
    println!("-------------handle exception----------");
    proc_exception();
    println!("-------------macro----------");
    proc_macro();
    let name = "tiny";
    print_sth!(name);
    println!("-------------thread----------");
    proc_thread();
    println!("-------------build in traits----------");
    proc_build_in_traits();
    println!("-------------reflect----------");
    proc_reflect();
    println!("-------------unsafe----------");
    proc_unsafe();
    println!("-------------other----------");

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
where
    F: Fn(i32, i32) -> i32,
{
    f(y, y)
}
