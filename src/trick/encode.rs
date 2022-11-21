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

impl Encode for u8 {
    fn size_hint(&self) -> usize {
        size_of::<u8>()
    }

    fn encode_to(&self, output: &mut Vec<u8>) {
        output.extend(self.to_be_bytes())
    }
}

impl Encode for i32 {
    fn size_hint(&self) -> usize {
        size_of::<i32>()
    }

    fn encode_to(&self, output: &mut Vec<u8>) {
        output.extend(self.to_be_bytes())
    }
}

impl Encode for u32 {
    fn size_hint(&self) -> usize {
        size_of::<u32>()
    }

    fn encode_to(&self, output: &mut Vec<u8>) {
        output.extend(self.to_be_bytes())
    }
}

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
