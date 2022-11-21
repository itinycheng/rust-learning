use core::u8;

use super::{
    decode::{Decode, Input},
    encode::Encode,
};

#[derive(Debug)]
pub struct StoreArgs<Args> {
    args: Args,
}

impl StoreArgs<EmptyArgumentList> {
    pub fn new() -> StoreArgs<EmptyArgumentList> {
        StoreArgs {
            args: ArgumentList::new(),
        }
    }

    pub fn push_arg<T: Encode>(
        self,
        arg: T,
    ) -> StoreArgs<ArgumentList<Argument<T>, EmptyArgumentList>> {
        StoreArgs {
            args: self.args.push_arg(arg),
        }
    }
}

impl<H, R> StoreArgs<ArgumentList<Argument<H>, R>> {
    pub fn push_arg<T: Encode>(
        self,
        arg: T,
    ) -> StoreArgs<ArgumentList<Argument<T>, ArgumentList<Argument<H>, R>>> {
        StoreArgs {
            args: self.args.push_arg(arg),
        }
    }
}

pub struct Argument<T> {
    arg: T,
}

pub struct ArgumentEnd;

pub type EmptyArgumentList = ArgumentList<ArgumentEnd, ArgumentEnd>;

#[derive(Debug)]
pub struct ArgumentList<Head, Rest> {
    head: Head,
    rest: Rest,
}

impl ArgumentList<ArgumentEnd, ArgumentEnd> {
    fn new() -> ArgumentList<ArgumentEnd, ArgumentEnd> {
        ArgumentList {
            head: ArgumentEnd,
            rest: ArgumentEnd,
        }
    }

    pub fn push_arg<T: Encode>(self, arg: T) -> ArgumentList<Argument<T>, Self> {
        ArgumentList {
            head: Argument { arg },
            rest: self,
        }
    }
}

impl<Head, Rest> ArgumentList<Argument<Head>, Rest> {
    fn push_arg<T: Encode>(self, arg: T) -> ArgumentList<Argument<T>, Self> {
        ArgumentList {
            head: Argument { arg },
            rest: self,
        }
    }
}

// encode
impl<T> Encode for Argument<T>
where
    T: Encode,
{
    fn encode_to(&self, output: &mut Vec<u8>) {
        <T as Encode>::encode_to(&self.arg, output)
    }

    fn size_hint(&self) -> usize {
        <T as Encode>::size_hint(&self.arg)
    }
}

impl Encode for EmptyArgumentList {
    fn size_hint(&self) -> usize {
        0
    }

    fn encode_to(&self, _: &mut Vec<u8>) {}
}

impl Encode for ArgumentEnd {
    fn size_hint(&self) -> usize {
        0
    }

    fn encode_to(&self, _: &mut Vec<u8>) {}
}

impl<Head, Rest> Encode for ArgumentList<Argument<Head>, Rest>
where
    Head: Encode,
    Rest: Encode,
{
    fn size_hint(&self) -> usize {
        self.head.size_hint() + self.rest.size_hint()
    }

    fn encode_to(&self, output: &mut Vec<u8>) {
        self.rest.encode_to(output);
        self.head.encode_to(output);
    }
}

impl<T> Encode for StoreArgs<T>
where
    T: Encode,
{
    fn size_hint(&self) -> usize {
        self.args.size_hint()
    }

    fn encode_to(&self, output: &mut Vec<u8>) {
        self.args.encode_to(output);
    }
}

// decode

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

#[cfg(test)]
mod tests {
    use crate::trick::{decode::Decode, encode::Encode};
    use super::StoreArgs;

    #[test]
    fn test_works() {
        let store_args = StoreArgs::new()
        .push_arg(1)
        .push_arg(2)
        .push_arg("aa");
        
        let encode = store_args.encode();
        let args = <(i32, i32, String) as Decode>::decode(&mut encode.as_slice()).unwrap();
        assert_eq!(args, (1, 2, "aa".to_string()))
    }

    #[test]
    fn test_vec() {
        let mut vec = Vec::with_capacity(20);
        vec.push(0u8);
       
        let slice = vec.as_mut_slice();
        assert_eq!(1, slice.len());
    }
}
