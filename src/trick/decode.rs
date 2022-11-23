use core::mem::size_of;

pub use anyhow::Error as AnyError;
use anyhow::{anyhow, Ok};

pub trait Input {
    fn read<const N: usize>(&mut self, into: &mut [u8; N]) -> Result<(), AnyError>;

    fn read_byte(&mut self) -> Result<u8, AnyError> {
        let mut into = [0_u8];
        self.read(&mut into)?;
        Ok(into[0])
    }
}

impl Input for &[u8] {
    fn read<const N: usize>(&mut self, into: &mut [u8; N]) -> Result<(), AnyError> {
        if self.len() < into.len() {
            return Err(anyhow!("Not have enough data for into buf"));
        }

        let len = into.len();
        into.copy_from_slice(&self[..len]);
        *self = &self[len..];
        Ok(())
    }
}

pub trait Decode: Sized {
    fn decode<I: Input>(input: &mut I) -> Result<Self, AnyError>;
}

impl<T: Decode> Decode for Vec<T> {
    fn decode<I: Input>(input: &mut I) -> Result<Self, AnyError> {
        let len = u32::decode(input)?;
        let mut vec = Vec::with_capacity(len as usize);
        for _ in 0..len {
            vec.push(T::decode(input)?);
        }
        Ok(vec)
    }
}

impl Decode for String {
    fn decode<I: Input>(input: &mut I) -> Result<Self, AnyError> {
        let vec = Vec::decode(input)?;
        Ok(Self::from_utf8(vec)?)
    }
}

// decode numeric.
macro_rules! numeric_decode_impl {
    ($($type:ty)*) => ($(
        impl Decode for $type {
            fn decode<I: Input>(input: &mut I) -> Result<Self, AnyError> {
                let mut buf = [0; size_of::<Self>()];
                input.read(&mut buf)?;
                Ok(Self::from_be_bytes(buf))
            }
        }
    )*)
}
numeric_decode_impl!(u8 u16 u32 u64 u128 i8 i16 i32 i64 i128 f32 f64);

// decode tuple.
macro_rules! tuple_decode_impl {
    ($( $ident:ident, )+) => {
        impl<$($ident: Decode,)+> Decode for ($($ident, )+) {
            fn decode<I: Input>(input: &mut I) -> Result<Self, anyhow::Error> {
                Ok((
                    $($ident::decode(input)?,)+
                ))
            }
        }
    }
}

tuple_decode_impl!(T1,);
tuple_decode_impl!(T1, T2,);
tuple_decode_impl!(T1, T2, T3,);
tuple_decode_impl!(T1, T2, T3, T4,);
tuple_decode_impl!(T1, T2, T3, T4, T5,);
tuple_decode_impl!(T1, T2, T3, T4, T5, T6,);
