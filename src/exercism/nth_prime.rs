pub fn nth(n: u32) -> u32 {
    (2..)
        .filter(|&x| is_prime(x))
        .nth(n as usize)
        .expect("not found.")
}

fn is_prime(num: u32) -> bool {
    let sqrt_num = (num as f64).sqrt() as u32;
    !(2..=sqrt_num).any(|x| num % x == 0)
}
