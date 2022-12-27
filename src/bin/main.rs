use rust_learning::basic::build_in_traits::proc_build_in_traits;
use rust_learning::basic::closure::proc_closure;
use rust_learning::basic::collection::{proc_collection, proc_iterator};
use rust_learning::basic::datatype::proc_data;
use rust_learning::basic::dyn_impl_trait::proc_dyn_impl_trait;
use rust_learning::basic::errors::proc_errors;
use rust_learning::basic::exception::proc_exception;
use rust_learning::basic::expr::proc_expr;
use rust_learning::basic::feature::proc_feature;
use rust_learning::basic::file::proc_file;
use rust_learning::basic::function::proc_func;
use rust_learning::basic::generic::proc_generic;
use rust_learning::basic::lifetime::proc_lifetime;
use rust_learning::basic::macs::proc_macro;
use rust_learning::basic::operator::proc_operate;
use rust_learning::basic::reflect::proc_reflect;
use rust_learning::basic::thread::proc_thread;
use rust_learning::basic::traits::proc_trait;
use rust_learning::basic::un_safe::proc_unsafe;
use rust_learning::collection::collection_test::{proc_binary_tree, proc_queue, proc_stack};
use rust_learning::print_sth;

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
    println!("-------------feature----------");
    proc_feature();
    println!("-------------collection----------");
    proc_stack();
    proc_queue();
    proc_binary_tree();
    println!("-------------dyn/impl trait----------");
    proc_dyn_impl_trait();
    println!("-------------errors----------");
    proc_errors();

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
