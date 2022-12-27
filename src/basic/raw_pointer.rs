use core::slice;

pub fn incr_twice(num: &i32) {
    let raw_pointer1 = num as *const i32 as *mut i32;
    let raw_pointer2 = num as *const i32 as *mut i32;
    unsafe {
        *raw_pointer1 += 1;
        *raw_pointer2 += 1;
    }
}

pub fn incr_boxed_num(num: Box<i32>) -> Box<i32> {
    // let ref_num = num.as_ref();
    let raw_pointer = Box::into_raw(num);
    unsafe {
        *raw_pointer += 1;
        Box::from_raw(raw_pointer)
    }
}

pub fn split_mut_slice(vec: &mut [String]) -> (&mut [String], &mut [String]) {
    let len = vec.len();
    assert!(len > 1);

    let mid = vec.len() / 2;
    let raw_vec_ptr = vec.as_mut_ptr();
    unsafe {
        (
            slice::from_raw_parts_mut(raw_vec_ptr, mid),
            slice::from_raw_parts_mut(raw_vec_ptr.add(mid), len - mid),
        )
    }
}

#[cfg(test)]
mod tests {
    use core::time::Duration;
    use std::{sync::Arc, thread};

    use super::{incr_boxed_num, incr_twice, split_mut_slice};

    #[test]
    fn test_split_slice() {
        let mut strings = vec!["a".to_owned(), "b".to_owned()];
        let (first, second) = split_mut_slice(strings.as_mut_slice());

        assert_eq!(first, &mut ["a".to_owned()]);
        assert_eq!(second, &mut ["b".to_owned()]);
    }

    #[test]
    fn test_incr_twice() {
        let num = 1;
        incr_twice(&num);
        println!("{}", num)
    }

    #[test]
    fn test_async_incr_twice() {
        // incr_twice() thread safe, these code may cause error.
        let num = Arc::new(1);
        let ref_num1 = num.clone();
        let _ = thread::spawn(move || {
            incr_twice(ref_num1.as_ref());
            println!("{}", ref_num1)
        });

        let ref_num2 = num.clone();
        let _ = thread::spawn(move || {
            incr_twice(ref_num2.as_ref());
            println!("{}", ref_num2)
        });

        thread::sleep(Duration::from_secs(1))
    }

    #[test]
    fn test_incr_box_num() {
        let mut result = incr_boxed_num(Box::new(1));

        let ref_num = &mut *result;
        *ref_num += 1;

        println!("{}", ref_num);
    }
}
