#![doc = include_str!("../README.md")]
mod read;
mod testcase;

pub type Error = Box<dyn std::error::Error>;

pub mod prelude {
    pub use crate::read::{Parse, ParseBlock, ParseLine};
    pub use crate::testcase::{BlockTestCase, InlineTestCase, TestCase};
    pub use crate::Error;
}

#[cfg(test)]
pub mod tests;
