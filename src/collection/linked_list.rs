use std::{marker::PhantomData, ptr::NonNull};

#[derive(Debug)]
pub struct Node<T> {
    value: T,
    prev: Option<NonNull<Node<T>>>,
    next: Option<NonNull<Node<T>>>,
}

#[derive(Debug)]
pub struct LinkedList<T> {
    len: usize,
    head: Option<NonNull<Node<T>>>,
    tail: Option<NonNull<Node<T>>>,
    _maker: PhantomData<Box<Node<T>>>,
}

impl<T> Node<T> {
    pub fn new(t: T) -> Self {
        Node {
            value: t,
            prev: None,
            next: None,
        }
    }
}

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        Self {
            len: 0,
            head: None,
            tail: None,
            _maker: PhantomData::default(),
        }
    }

    pub fn push_head(&mut self, t: T) {
        let mut node = Box::new(Node::new(t));
        node.next = self.head;

        let new_head_ptr = Some(unsafe { NonNull::new_unchecked(Box::into_raw(node)) });
        match self.head {
            Some(head_ptr) => unsafe { (*head_ptr.as_ptr()).prev = new_head_ptr },
            None => self.tail = new_head_ptr,
        }

        self.len += 1;
        self.head = new_head_ptr;
    }

    pub fn push_tail(&mut self, t: T) {
        let mut node = Box::new(Node::new(t));
        node.prev = self.tail;

        let new_tail_ptr = Some(unsafe { NonNull::new_unchecked(Box::into_raw(node)) });
        match self.tail {
            Some(tail_ptr) => unsafe { (*tail_ptr.as_ptr()).next = new_tail_ptr },
            None => self.head = new_tail_ptr,
        }

        self.tail = new_tail_ptr;
        self.len += 1;
    }

    pub fn pop_head(&mut self) -> Option<T> {
        match self.head {
            Some(head_ptr) => unsafe {
                let head = Box::from_raw(head_ptr.as_ptr());
                let next = head.next;
                if let Some(mut next_ptr) = next {
                    next_ptr.as_mut().prev = None;
                }

                self.head = next;
                self.len -= 1;
                Some(head.value)
            },
            None => None,
        }
    }

    pub fn pop_tail(&mut self) -> Option<T> {
        match self.tail {
            Some(tail_ptr) => unsafe {
                let tail = Box::from_raw(tail_ptr.as_ptr());
                if let Some(mut prev_ptr) = tail.prev {
                    prev_ptr.as_mut().next = None;
                }

                self.tail = tail.prev;
                self.len -= 1;
                Some(tail.value)
            },
            None => None,
        }
    }
}

impl<T> Iterator for LinkedList<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.pop_head()
    }
}

impl<T> Drop for LinkedList<T> {
    fn drop(&mut self) {
        while self.pop_head().is_some() {}
    }
}

#[cfg(test)]
mod tests {
    use super::LinkedList;

    #[test]
    fn test_iter() {
        let mut list = LinkedList::new();
        list.push_tail(2);
        list.push_tail(3);
        list.push_head(1);

        for elem in list {
            println!("{}", elem);
        }
    }

    #[test]
    fn test_list() {
        let mut list = LinkedList::new();
        list.push_tail(2);
        list.push_tail(3);
        list.push_head(1);
        println!("{:?}", list.pop_tail());
        println!("{:?}", list.pop_head());
        println!("{:?}", list.pop_head());
    }

    #[test]
    fn test_copy() {
        let str = Some("test".to_string());
        match str {
            Some(_) => println!("some"),
            None => println!("none"),
        }

        println!("{:?}", str);
    }

    #[test]
    fn test_pointer() {
        let num = 5;
        println!("{:p}", &num);
        let box_num = Box::new(num);
        println!("{:p}", box_num);

        let str = String::from("value");
        println!("{:p}", &str);
        let box_str = Box::new(str);
        println!("{:p}", box_str);
    }

    #[test]
    #[should_panic]
    fn test_number_overflow() {
        let mut num = 255u8;
        num = add_one(num);
        println!("{}", num)
    }

    #[inline(never)]
    fn add_one(num: u8) -> u8 {
        num + 1
    }
}
