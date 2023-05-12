use std::marker::PhantomPinned;
use std::pin::Pin;

#[derive(Debug)]
struct PinTest {
    value: String,
    reference: *const String,
    _marker: PhantomPinned,
}

impl PinTest {
    fn new(txt: &str) -> Self {
        PinTest {
            value: String::from(txt),
            reference: std::ptr::null(),
            _marker: PhantomPinned,
        }
    }

    fn init<'a>(self: Pin<&'a mut Self>) {
        let self_ptr: *const String = &self.value;
        let this = unsafe { self.get_unchecked_mut() };
        this.reference = self_ptr;
    }

    fn value<'a>(self: Pin<&'a Self>) -> &'a str {
        &self.get_ref().value
    }

    fn reference<'a>(self: Pin<&'a Self>) -> &'a String {
        unsafe { &*(self.reference) }
    }
}

#[cfg(test)]
mod tests {
    use std::{fmt::Debug, pin::Pin};

    use crate::basic::pin_mem::PinTest;

    #[test]
    pub fn test_pin() {
        let mut pin_test1 = PinTest::new("pin_test_1");
        let mut pin_test2 = PinTest::new("pin_test_1");
        let mut test1 = unsafe { Pin::new_unchecked(&mut pin_test1) };
        PinTest::init(test1.as_mut());

        let mut test2 = unsafe { Pin::new_unchecked(&mut pin_test2) };
        PinTest::init(test2.as_mut());

        println!(
            "test_1 value: {}, reference: {}",
            PinTest::value(test1.as_ref()),
            PinTest::reference(test1.as_ref())
        );

        // compile error.
        // std::mem::swap(test1.get_mut(), test2.get_mut());
        // std::mem::swap(&mut *test1, &mut *test2);

        println!(
            "test_2 value: {}, reference: {}",
            PinTest::value(test2.as_ref()),
            PinTest::reference(test2.as_ref())
        );
    }

    #[test]
    pub fn test_dyn() {
        let mut vec: Vec<Box<dyn Debug>> = vec![];
        vec.push(Box::new("value"));
        vec.push(Box::new(1));
        vec.push(Box::new(Unsized { value: "tes" }));
        println!("{:?}", vec);
    }

    struct Unsized<T: ?Sized + Debug> {
        value: T,
    }

    impl<T: Debug> Debug for Unsized<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.debug_struct("Unsized")
                .field("value", &self.value)
                .finish()
        }
    }
}
