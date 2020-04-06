use futures::executor::block_on;
use rust_learning::concurrent::async_and_wait;

#[test]
fn test1() {
    let hello = block_on(async_and_wait::hello_join("tiny"));
    println!("{}", hello);
}

#[test]
fn test2() {
    let hello = block_on(async_and_wait::hello_await("tiny"));
    println!("{}", hello);
}

#[test]
fn test3() {
    let hello = block_on(async_and_wait::hello_await("tiny"));
    println!("{}", hello);
}
