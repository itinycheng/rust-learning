pub fn square_of_sum(n: u32) -> u32 {
    (1..=n).fold(0, |acc, x| acc + x).pow(2)
}

/// pow
pub fn sum_of_squares(n: u32) -> u32 {
    (1..=n).map(|x| x.pow(2)).fold(0, |acc, x| acc + x)
}

pub fn difference(n: u32) -> u32 {
    square_of_sum(n) - sum_of_squares(n)
}
