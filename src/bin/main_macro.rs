use my_proc_macro::create_fn;
use my_proc_macro::hello_macro;
use my_proc_macro::print_attr;
use my_proc_macro::time_cost;
use my_proc_macro::CustomDebug;
use rust_learning::HelloMacro;

#[derive(hello_macro)]
struct TestS<T>(T);

#[derive(CustomDebug)]
struct NodeS {
    #[debug(rename = "n_id", format = "{:b}")]
    id: i32,

    #[debug(rename = "n_name", format = "{:<5}")]
    name: String,
}

create_fn!(proc);

fn main() {
    let test = TestS("TEST");
    test.hello();

    // fn
    proc(String::from("proc"));

    say_hi("tes".to_string());

    say_no("tiny".to_string());

    println!(
        "{:?}",
        NodeS {
            id: 1,
            name: "tiny".to_string()
        }
    );
}

#[time_cost]
fn say_hi(name: String) {
    println!("hi {}", name);
}

// only the first macro works
#[print_attr(uri = "path", format = "default")]
#[time_cost]
fn say_no(name: String) {
    println!("no {}", name);
}
