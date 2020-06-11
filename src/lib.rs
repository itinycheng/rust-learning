pub mod basic;
pub mod collection;
pub mod concurrent;
pub mod exercism;

#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate my_proc_macro;

pub trait HelloMacro {
    fn hello(&self) -> ();
}