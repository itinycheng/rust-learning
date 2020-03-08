#![allow(dead_code)]

struct Foo<T>(T);

#[derive(Debug)]
struct Bar<T: ?Sized>(T);

// name
// struct FooUse(Foo<[i32]>);

#[derive(Debug)]
struct BarUse(Bar<[i32]>);

pub fn proc_feature() {
    let mut vec1 = vec![];
    vec1.push(1);
    println!("vec1 {:?}", &*vec1);
}
