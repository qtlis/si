#![feature(proc_macro)]

#![warn(unused,
        missing_debug_implementations, missing_copy_implementations,
        missing_docs)]
#![deny(bad_style, future_incompatible,
        unsafe_code,
        trivial_casts, trivial_numeric_casts)]

pub extern crate si_macros as macros;

pub use macros::unit;

pub mod prelude;
pub mod units;

#[cfg(test)]
mod tests {

}
