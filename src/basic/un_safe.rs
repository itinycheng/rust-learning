unsafe trait Trait {}

unsafe impl Trait for i32 {}

pub fn proc_unsafe() {
    // 1
    let x = 5;
    let raw = &x as *const i32;
    let point_at = unsafe { *raw };
    println!("point_at raw {}", point_at);
    // 2
    static mut N: i32 = 2;
    unsafe {
        N += 5;
        println!("N Added: {}", N);
    }
    // 3
    unsafe {
        unsafe_fn();
    }
}

unsafe fn unsafe_fn() {
    println!("unsafe");
}
