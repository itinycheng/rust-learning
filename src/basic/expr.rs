const NO_1: i32 = 1;

pub fn proc_expr() {
    expr_if(NO_1);
    expr_loop();
    expr_match();
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

fn expr_match() {
    match_int(6);
    match_tuple((1, 1));
    match_string(String::from("tiny"));
    match_deep(Some(Teacher { name: "tiny", age: 18, teach: Some("programming"), detail: Some("nothing") }));
}

fn match_deep(t: Option<Teacher>) {
    match t {
        Some(Teacher { age: 18, teach: Some(tch), detail: ref d @ Some(_), .. }) => println!("match_deep, {} {}", tch, d.unwrap()),
        Some(Teacher { name, age: 19, .. }) => println!("match_deep name={}", name),
        Some(_) | None => ()
    }
}

fn match_int(i: i32) {
    match i {
        e @ 1..=5 => println!("match_int: weekday = {}", e),
        6 | 7 => println!("match_int: weekend = {}", i),
        n @ 8..=10 if n > 0 => println!("match_int: positive = {}", n),
        _ => println!("match_int: invalid")
    }
}

fn match_string(mut s: String) {
    match s {
        ref mut e => println!("match_string, {}", e)
    }
    println!("match_string, println {}", s);
}

fn match_tuple(tuple: (i32, i32)) {
    match tuple {
        (0, y) => println!("match_tuple: tuple x=0, y={}", y),
        (x, ..) => println!("match_tuple: tuple x={}, y=..", x),
    }
}

struct Teacher<'a> {
    name: &'a str,
    age: i8,
    teach: Option<&'a str>,
    detail: Option<&'a str>,
}

