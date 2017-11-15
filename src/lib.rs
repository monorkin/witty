#![feature(proc_macro)]

extern crate futures;
extern crate hyper;
extern crate hyper_rustls;
extern crate tokio_core;
extern crate serde;
extern crate serde_derive;
#[macro_use]
extern crate serde_json;
extern crate serde_urlencoded;

pub mod client;
pub mod http;

pub use serde_json::Value;

///
/// Builds and returns a Witty client struct. Conveniance method.
/// Reference `witty::Client` for more information.
///
pub fn client(token: &str) -> client::Client {
    client::Client::new(token)
}
