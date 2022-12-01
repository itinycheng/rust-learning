pub fn proc_closure() {
    func_1();
    func_2();
    func_3();
    func_4();
    func_5();
    func_6();
}

fn func_6() {
    fn get_closure() -> impl Fn(i32) -> i32 {
        |x| x + 5
    }
    let result = get_closure()(3);
    println!("get_closure {}", result);
}

fn func_5() {
    fn call(closure: &dyn Fn(i32) -> i32, input: i32) -> i32 {
        closure(input)
    }
    fn add_one(x: i32) -> i32 {
        x + 1
    }
    let result = call(&add_one, 2);
    println!("func_5 {}", result);
}

fn func_4() {
    fn call(closure: &dyn Fn(i32) -> i32, input: i32) -> i32 {
        closure(input)
    }
    let result = call(&|x: i32| x * x, 2);
    println!("func_4 {}", result);
}

fn func_3() {
    let mut num = 2;
    {
        // move will cause number copy
        let mut add_num = |x: i32| num += x;
        add_num(2);
        println!("func_3 inner {}", num);
    }
    println!("func_3 outer {}", num);
}

fn func_2() {
    let add = |x: i32, y: i32| -> i32 { x + y };
    println!("1 + 2 = {}", add(1, 2));
}

fn func_1() {
    let mut num = 0;
    let mut add_num = |x: i32| num += x;
    add_num(2);
    println!("num = {}", num);
}
