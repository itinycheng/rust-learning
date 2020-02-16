pub fn proc_func() {
    print(("tiny", 18));
    let func: IncType = inc;
    println!("func {}", func(2));
    println!("call {}", call(3, inc));
    println!("call_1 {}", call_1(3, inc));
    println!("rtn_func {}", rtn_func(1)(2))
}

fn print((first, _): (&str, i32)) {
    println!("print, first elem {}, discard second", first);
}

type IncType = fn(i32) -> i32;

fn inc(i: i32) -> i32 {
    i + 1
}

fn dec(i: i32) -> i32 {
    i - 1
}

fn call(i: i32, func: IncType) -> i32 {
    func(i)
}

fn call_1<F>(i: i32, func: F) -> i32
    where F: Fn(i32) -> i32 {
    func(i)
}

fn rtn_func(i: i32) -> fn(i32) -> i32 {
    fn print() { println!("inner func"); }
    print();
    if i > 0 {
        inc
    } else {
        dec
    }
}