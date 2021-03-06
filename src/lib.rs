#![warn(rust_2018_idioms)]
#![allow(unused_imports)]

#[macro_use] extern crate serde_derive;
#[macro_use] extern crate prometheus;

use snafu::{Backtrace, OptionExt, ResultExt, Snafu};
#[derive(Debug, Snafu)]
pub enum Error {
    #[snafu(display("Foo has bad info: {}", info))]
    FooIsBad { info: String, backtrace: Backtrace },

    #[snafu(display("Failed to patch Foo: {}", source))]
    FooPatchFailed {
        source: kube::Error,
        backtrace: Backtrace,
    },

    SerializationFailed {
        source: serde_json::Error,
        backtrace: Backtrace,
    },
}
pub type Result<T, E = Error> = std::result::Result<T, E>;

/// State machinery for kube, as exposeable to actix
pub mod manager;
pub use manager::Manager;
