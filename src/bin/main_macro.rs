#[macro_use]
extern crate my_proc_macro;

use rust_learning::HelloMacro;

#[derive(hello_macro)]
struct TestS<T>(T);

create_fn!(proc);

fn main() {
    let test = TestS("TEST");
    test.hello();

    // fn
    proc(String::from("proc"));

    say_hi("tes".to_string());
}

#[time_cost]
fn say_hi(name: String) {
    println!("hi {}", name);
}