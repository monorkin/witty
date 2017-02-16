#![feature(proc_macro)]

extern crate hyper;
extern crate serde;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;
extern crate serde_urlencoded;

mod client;
mod http;

use serde_json::{Value, Map};
use std::collections::BTreeMap;

///
/// Builds and returns a Witty client struct. Conveniance method.
/// Reference `witty::Client` for more information.
///
pub fn client(
    token: &str,
    actions: BTreeMap<String, fn(Value) -> Value>,
    send_method: fn(Value, Value)
) -> client::Client
{
    client::Client::new(token, actions, send_method)
}
