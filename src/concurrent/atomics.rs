use std::sync::atomic::Ordering::SeqCst;
use std::sync::atomic::{AtomicBool, AtomicI32, AtomicPtr};
use std::thread;

static BOOL_X: AtomicBool = AtomicBool::new(false);
static BOOL_Y: AtomicBool = AtomicBool::new(false);
static NUM_Z: AtomicI32 = AtomicI32::new(0);

static mut NUM_PRT: AtomicPtr<i32> = AtomicPtr::new(0 as *mut i32);

pub fn proc_atomics() {
    unsafe {
        println!(":#? {:#?}", NUM_PRT);
        NUM_PRT = AtomicPtr::new(&mut 2);
        println!(":? {:?}, {:?}", NUM_PRT, *NUM_PRT.get_mut());
        *NUM_PRT.get_mut() = 5 as *mut i32;
        println!("#?: {:#?}", NUM_PRT);
        println!("AtomicPtr deref: {}", *NUM_PRT.get_mut() as i32);
    }

    let t_x = thread::spawn(write_x);
    let t_y = thread::spawn(write_y);
    let t_x_y = thread::spawn(read_x_then_y);
    let t_y_x = thread::spawn(read_y_then_x);

    let _ = t_x.join();
    let _ = t_y.join();
    let _ = t_x_y.join();
    let _ = t_y_x.join();
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
