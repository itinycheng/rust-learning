use core::mem::size_of;

// Encode
pub trait Encode {
    fn size_hint(&self) -> usize;

    fn encode_to(&self, output: &mut Vec<u8>);

    fn encode(&self) -> Vec<u8> {
        let mut vec = Vec::with_capacity(self.size_hint());
        self.encode_to(&mut vec);
        vec
    }
}

macro_rules! numeric_encode_impl {
    ($($type:ty)*) => {
        $(
            impl Encode for $type {
                fn size_hint(&self) -> usize {
                    size_of::<Self>()
                }

                fn encode_to(&self, output: &mut Vec<u8>) {
                    output.extend(self.to_be_bytes())
                }
            }
        )*
    };
}
numeric_encode_impl!(u8 u16 u32 u64 u128 i8 i16 i32 i64 i128 f32 f64);

impl Encode for &str {
    fn size_hint(&self) -> usize {
        self.as_bytes().size_hint()
    }

    fn encode_to(&self, output: &mut Vec<u8>) {
        self.as_bytes().encode_to(output)
    }
}

impl<T> Encode for &[T]
where
    T: Encode,
{
    fn size_hint(&self) -> usize {
        size_of::<u32>() + size_of::<T>() * self.len()
    }

    fn encode_to(&self, output: &mut Vec<u8>) {
        (self.len() as u32).encode_to(output);
        self.iter().for_each(|t| t.encode_to(output));
    }
}
