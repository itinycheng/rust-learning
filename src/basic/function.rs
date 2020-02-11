pub fn func() {
    print(("tiny", 18));
}

fn print((first, _): (&str, i32)) {
    println!("print, first elem {}, discard second", first);
}