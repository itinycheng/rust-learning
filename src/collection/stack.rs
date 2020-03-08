#![allow(dead_code)]

#[derive(Debug)]
pub struct Stack<T> {
    top: Option<Box<StackNode<T>>>,
}

#[derive(Debug)]
struct StackNode<T> {
    value: T,
    next: Option<Box<StackNode<T>>>,
}

impl<T> StackNode<T> {
    fn new(val: T) -> StackNode<T> {
        StackNode {
            value: val,
            next: None,
        }
    }
}

impl<T> Stack<T> {
    pub fn new() -> Stack<T> {
        Stack { top: None }
    }

    pub fn push(&mut self, item: T) {
        let node = StackNode {
            value: item,
            next: self.top.take(),
        };
        self.top = Some(Box::new(node));
    }

    pub fn pop(&mut self) -> Option<T> {
        match self.top.take() {
            Some(x) => {
                self.top = x.next;
                Some(x.value)
            }
            None => None,
        }
    }
}
