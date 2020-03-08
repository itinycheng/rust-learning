use crate::collection::binary_tree::{BTree, BinaryTree};
use crate::collection::queue::Queue;
use crate::collection::stack::Stack;

pub fn proc_stack() {
    let mut stack = Stack::new();
    stack.push(1);
    stack.push(2);
    println!("stack {:?}", stack.pop());
    println!("stack {:?}", stack.pop());
}

pub fn proc_queue() {
    let mut queue = Queue::new();
    queue.push(1);
    queue.push(2);
    println!("queue {:?}", queue.pop());
    println!("queue {:?}", queue.pop());
}

pub fn proc_binary_tree() {
    let mut tree = BTree::new(2, "2");
    tree.insert(1, "1");
    tree.insert(3, "3");
    let consume: fn(&i32, &&str) = |&key, &value| println!("key = {}, value= {}", key, value);
    tree.pre_order(consume);
}
