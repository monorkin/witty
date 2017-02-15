#![feature(proc_macro)]

extern crate hyper;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

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
    actions: BTreeMap<String, fn(Map<String, Value>, Map<String, Value>)>
) -> Result<client::Client, client::InvalidActionsError>
{
    client::Client::new(token, actions)
}
