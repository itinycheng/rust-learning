use rand::random;

/// run the two futures in series
pub async fn hello_await<T: Into<String>>(name: T) -> String {
    let say_hello = say_hello().await;
    let greeting = greeting().await;
    format!("{}{}{}", say_hello, name.into(), greeting)
}

/// run the two futures concurrently
pub async fn hello_join<T: Into<String>>(name: T) -> String {
    let say_hello = say_hello();
    let greeting = greeting();
    let (hello, greet) = futures::join!(say_hello, greeting);
    format!("{}{}{}", hello, name.into(), greet)
}

async fn say_hello() -> String {
    if random::<u8>() % 2 == 0 {
        "Hello ".to_string()
    } else {
        "Hi ".to_string()
    }
}

async fn greeting() -> String {
    if random::<u8>() % 2 == 0 {
        ", what a nice day!".to_string()
    } else {
        ", nice day isn't it!".to_string()
    }
}
