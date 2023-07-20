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

    fn value<'a>(self: Pin<&'a Self>) -> &'a String {
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
    pub fn test_pin_to_stack() {
        let mut test1 = PinTest::new("pin_test_1");
        let mut test2 = PinTest::new("pin_test_2");
        let mut pin_test1 = unsafe { Pin::new_unchecked(&mut test1) };
        PinTest::init(pin_test1.as_mut());

        let mut pin_test2 = unsafe { Pin::new_unchecked(&mut test2) };
        PinTest::init(pin_test2.as_mut());

        println!(
            "pin_test_1: {:?}, pin_test_2: {:?}",
            pin_test1.as_ref(),
            pin_test2.as_ref()
        );

        println!(
            "pin_test_1 value: {}, reference: {}",
            PinTest::value(pin_test1.as_ref()),
            PinTest::reference(pin_test1.as_ref())
        );

        // compile error.
        // std::mem::swap(test1.get_mut(), test2.get_mut());
        // std::mem::swap(&mut *test1, &mut *test2);

        println!(
            "pin_test_2 value: {}, reference: {}",
            PinTest::value(pin_test2.as_ref()),
            PinTest::reference(pin_test2.as_ref())
        );
    }

    #[test]
    fn test_pin_to_heap() {
        let mut pin_test1 = Box::pin(PinTest::new("pin_test_1"));
        let mut pin_test2 = Box::pin(PinTest::new("pin_test_2"));
        PinTest::init(pin_test1.as_mut());
        PinTest::init(pin_test2.as_mut());

        println!(
            "pin_test_1: {:?}, pin_test_2: {:?}",
            pin_test1.as_ref(),
            pin_test2.as_ref()
        );
    }

    #[test]
    pub fn test_dyn() {
        // slice::from_raw_parts("".as_ptr(), 10);
        let mut vec: Vec<Box<dyn Debug>> = vec![];
        vec.push(Box::new("value"));
        vec.push(Box::new(1));
        vec.push(Box::new(Unsized { value: "tes" }));
        println!("{:?}", vec);
    }

    struct Unsized<T: ?Sized + Debug + 'static> {
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
