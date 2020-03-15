#![allow(dead_code)]

pub fn abbreviate(phrase: &str) -> String {
    solution1(phrase)
}

fn solution1(phrase: &str) -> String {
    phrase.split(|c: char| c.is_whitespace() || c == '-')
        .flat_map(|word| word.chars()
            .filter(|c| c.is_alphabetic())
            .take(1)
            .chain(word.chars()
                .filter(|c| c.is_alphabetic())
                .skip_while(|c| c.is_uppercase())
                .filter(|c| c.is_uppercase())))
        .collect::<String>()
        .to_uppercase()
}

// can't handle camelcase
fn solution2(phrase: &str) -> String {
    let is_sp = |ch: char| ch.is_whitespace() || ch == '-';
    phrase.chars()
        .fold((true, &mut String::new()), |(flag, value), c| {
            let mut is_pushed = true;
            if flag {
                if c.is_alphabetic() {
                    value.push(c)
                } else {
                    is_pushed = false;
                }
            }
            let flag = if is_pushed { is_sp(c) } else { flag };
            (flag, value)
        }).1.clone().to_uppercase()
}

// TODO
fn solution3(phrase: &str) -> String {
    phrase.replace('_', "").chars().collect()
}

// TODO
fn solution4(phrase: &str) -> String {
    phrase.chars().collect::<Vec<char>>()
        .windows(2).flatten().collect()
}