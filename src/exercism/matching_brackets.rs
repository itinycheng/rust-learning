#![allow(dead_code)]

use std::collections::HashMap;

pub fn brackets_are_balanced(string: &str) -> bool {
    let mut bucket: Vec<char> = vec![];
    for c in string.chars() {
        match c {
            '{' | '[' | '(' => bucket.push(c),
            '}' => {
                if bucket.pop() != Some('{') {
                    return false;
                }
            }
            ']' => {
                if bucket.pop() != Some('[') {
                    return false;
                }
            }
            ')' => {
                if bucket.pop() != Some('(') {
                    return false;
                }
            }
            _ => {}
        }
    }
    bucket.is_empty()
}

fn solution1(string: &str) -> bool {
    let mapping: HashMap<char, i32> = [
        ('{', 1),
        ('}', -1),
        ('[', 2),
        (']', -2),
        ('(', 3),
        (')', -3),
    ]
    .iter()
    .cloned()
    .collect();
    string
        .chars()
        .map(|c| *mapping.get(&c).unwrap_or(&0))
        .filter(|&value| value != 0)
        .fold(&mut vec![], |vec, num| {
            if num < 0 && vec.len() > 0 && *vec.get(vec.len() - 1).unwrap_or(&0) + num == 0 {
                vec.remove(vec.len() - 1);
            } else {
                vec.push(num);
            }
            vec
        })
        .is_empty()
}
