use my_proc_macro::create_fn;
use my_proc_macro::hello_macro;
use my_proc_macro::print_attr;
use my_proc_macro::time_cost;
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

    say_no("tiny".to_string())
}

#[time_cost]
fn say_hi(name: String) {
    println!("hi {}", name);
}

#[print_attr(uri = "path", format = "default")]
fn say_no(name: String) {
    println!("no {}", name);
}
