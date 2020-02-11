const NO_1: i32 = 1;

pub fn proc_expr() {
    expr_if(NO_1);
    expr_loop();
}

fn expr_if(x: i32) {
    // 1
    if x > 0 {
        println!("x:{} is bigger than 0", x)
    } else if x == 0 {
        println!("x is 0")
    } else {
        println!("x:{} is littler than 0", x)
    }

    // 2
    let elem: i32 = if x == 5 { 10 } else { 3 };
    println!("if statement elem = {}", elem);

    // 3
    let some = Some(x);
    if let Some(i) = some {
        println!("some matched {}", i)
    } else {
        println!("some unmatched")
    }

    // 4
    let elem = if let Some(i) = some {
        i
    } else { 0 };
    println!("let if let Some elem = {}", elem);
}

fn expr_loop() {
    // 1
    for i in 1..5 {
        println!("for statement i = {}", i);
    }
    for i in 1..=5 {
        println!("for statement second i = {}", i);
    }
    for (i, j) in (1..5).enumerate() {
        println!("for enumerate, i={}, j={}", i, j)
    }
    // 2
    let mut number = NO_1;
    while number < 3 {
        println!("while expr, number = {}", number);
        number += 1;
    }

    let mut some = Some(NO_1);
    while let Some(i) = some {
        if i > 5 {
            println!("bigger than 5.");
            break;
        } else {
            println!("{} is not bigger than 5", i);
            some = Some(i + 1)
        }
    }

    'outer: loop {
        println!("outer loop");
        'inner: loop {
            println!("inner loop");
            break 'inner;
        }
        break;
    }
}



