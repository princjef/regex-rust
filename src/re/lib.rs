#![crate_id = "rustre#0.1.1"]
#![feature(globs)]
#![feature(macro_rules)]

#![allow(dead_code)]

#![license = "MIT"]
#![comment = "Regular Expression Engine in Rust"]


pub use regexp::Regexp;

mod test;
mod exec;
pub mod error;
pub mod parse;
mod state;
pub mod compile;
pub mod charclass;
mod unicode;

pub mod result;
pub mod regexp;

fn main() {
}
