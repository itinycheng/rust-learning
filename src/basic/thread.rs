use std::cell::{Cell, RefCell};
use std::collections::HashMap;
use std::rc::Rc;
use std::sync::mpsc::channel;
use std::sync::{Arc, Mutex, RwLock};
use std::thread;

#[derive(PartialEq, Eq, Debug)]
struct Owner<'a> {
    name: &'a str,
}

#[derive(Debug)]
struct Gadget<'a> {
    id: i32,
    owner: Rc<Owner<'a>>,
}

pub fn proc_thread() {
    rc_and_arc();
    ref_obj_twice_times();
    arc_mutex();
    rw_lock();
    cell_refcell();
}

fn cell_refcell() {
    let cell = Cell::new(5);
    println!("cell {}", cell.get());
    cell.set(6);
    println!("cell {}", cell.get());
    // 只能
    let ref_map = Rc::new(RefCell::new(HashMap::with_capacity(2)));
    ref_map.borrow_mut().insert("tiny", 18);
    ref_map.borrow_mut().insert("look", 18);
    println!("ref_map: {:?}", ref_map);
}

fn rw_lock() {
    let lock = RwLock::new(3);
    {
        let r1 = lock.read().unwrap();
        let r2 = lock.read().unwrap();
        println!("r1 {:?}, r2 {:?}", r1, r2);
    }
    // release lock
    {
        let mut w = lock.write().unwrap();
        *w += 1;
        println!("w {:?}", w);
    }
    println!("lock {:?}", lock);
}

fn arc_mutex() {
    let data = Arc::new(Mutex::new(0));
    let (tx, rx) = channel();
    for _ in 1..=10 {
        let (data, tx) = (data.clone(), tx.clone());
        thread::spawn(move || {
            let mut data = data.lock().unwrap();
            *data += 1;
            if *data == 10 {
                tx.send("super daddy").unwrap();
            }
        });
    }
    rx.recv().map(|x| println!("{:?}. dog.", x)).unwrap();
}

fn ref_obj_twice_times() {
    let owner = Rc::new(Owner { name: "tiny" });
    let gadget1 = Gadget {
        id: 1,
        owner: owner.clone(),
    };
    let gadget2 = Gadget {
        id: 1,
        owner: owner.clone(),
    };
    println!("gadget1: {:?}, gadget2: {:?}", gadget1, gadget2);

    // 只是drop reference，对象还能访问到
    drop(owner);

    println!("gadget1:{:?}， gadget2： {:?}", gadget1, gadget2);
}

fn rc_and_arc() {
    // 可以直接使用，无需拆包裹
    let five = Rc::new(5);
    let cloned = five.clone();
    println!("origin {}, cloned {}", five, cloned);
    let weak_five = Rc::downgrade(&five);
    let strong_five = weak_five.upgrade();
    println!(
        "weak_five: {:?}, strong_five: {:?}",
        weak_five,
        strong_five.unwrap()
    );

    // 一个对象只能move一次
    let numbers: Vec<i32> = (1..=10).collect();
    let shared_numbers = Arc::new(numbers);
    for _ in 0..=10 {
        let child_numbers = shared_numbers.clone();
        thread::spawn(move || {
            let local_numbers = &child_numbers[3..];
            println!("{:?}", local_numbers);
        });
    }
    let zero = Arc::new(0);
    let weak_zero = Arc::downgrade(&zero);
    let strong_zero = weak_zero.upgrade().unwrap();
    println!("weak_zero: {:?}, strong_zero: {:?}", weak_zero, strong_zero);
}
