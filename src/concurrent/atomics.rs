use std::sync::atomic::Ordering::SeqCst;
use std::sync::atomic::{AtomicBool, AtomicI32};
use std::thread;

static BOOL_X: AtomicBool = AtomicBool::new(false);
static BOOL_Y: AtomicBool = AtomicBool::new(false);
static NUM_Z: AtomicI32 = AtomicI32::new(0);

pub fn proc_atomics() {
    let t_x = thread::spawn(move || write_x());
    let t_y = thread::spawn(move || write_y());
    let t_x_y = thread::spawn(move || read_x_then_y());
    let t_y_x = thread::spawn(move || read_y_then_x());

    t_x.join();
    t_y.join();
    t_x_y.join();
    t_y_x.join();
    println!("x: {:?}, y: {:?}, z: {:?}", BOOL_X, BOOL_Y, NUM_Z)
}

fn write_x() {
    BOOL_X.store(true, SeqCst)
}

fn write_y() {
    BOOL_Y.store(true, SeqCst)
}

fn read_x_then_y() {
    while !BOOL_X.load(SeqCst) {}
    if BOOL_Y.load(SeqCst) {
        NUM_Z.fetch_add(1, SeqCst);
    }
}

fn read_y_then_x() {
    while !BOOL_Y.load(SeqCst) {}
    if BOOL_X.load(SeqCst) {
        NUM_Z.fetch_add(1, SeqCst);
    }
}
