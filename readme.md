# Witty

Unofficial wit.ai client library for Rust

## Installation

This project is ~not yet available~ on [Crates.io](https://crates.io/), but it can
be installed via git.

```Cargo
[dependencies]
witty = { git = "git@github.com:Stankec/witty.git" }
```

## Usage

To create a client you will need to get a token from [Wit.ai](https://wit.ai/).
A map of actions and a send fuction need to be defined.

```Rust
extern crate witty;
use std::collections::BTreeMap;

struct Actions {}

impl Actions {
  pub fn send(request: Value, response: Value) {
    // TODO: Implement logic for sending the message to the client
    println!("Sending message: '{}'", response.get("text").unwrap_or(""));
  }

  pub fn weather(request: Value) -> Option<Value> {
    // TODO: Implement logic to get the weather

    // Return the new context
    request.get("context").unwrap_or(json!({}))
  }
}

fn main() {
  let token = "the_token_you_got_from_your_wit_ai_repo";
  let actions = BTreeMap::new();
  actions.insert("weather".to_string(), Actions::weather);

  let client = witty::client(token, actions, Actions::send);

  // ...
}
```
