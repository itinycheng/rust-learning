pub use anyhow::Error as AnyError;
use anyhow::{anyhow, Ok};

pub trait Input {
    fn read(&mut self, into: &mut [u8]) -> Result<(), AnyError>;

    fn read_byte(&mut self) -> Result<u8, AnyError> {
        let mut into = [0_u8];
        self.read(&mut into)?;
        Ok(into[0])
    }
}

impl Input for &[u8] {
    fn read(&mut self, into: &mut [u8]) -> Result<(), AnyError> {
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

impl Decode for u8 {
    fn decode<I: Input>(input: &mut I) -> Result<Self, AnyError> {
        Ok(input.read_byte()?)
    }
}

impl Decode for i32 {
    fn decode<I: Input>(input: &mut I) -> Result<Self, AnyError> {
        let mut buf = [0; 4];
        input.read(buf.as_mut_slice())?;
        Ok(Self::from_be_bytes(buf))
    }
}

impl Decode for u32 {
    fn decode<I: Input>(input: &mut I) -> Result<Self, AnyError> {
        let mut buf = [0; 4];
        input.read(buf.as_mut_slice())?;
        Ok(Self::from_be_bytes(buf))
    }
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
