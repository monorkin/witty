# Witty

Unofficial wit.ai client library for Rust.

## Disclamer

This is an unofficial client implementation! It comes with no guarantees and
[Wit.ai](https://wit.ai/) can't be held responsible for any bugs that may occure
from using this library in your project.

This is a port of [Wit.ai's](https://wit.ai/) official
[Ruby client](https://github.com/wit-ai/wit-ruby) to Rust

## Installation

This project is _not yet available_ on [Crates.io](https://crates.io/), but
it can be installed via git.

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

### Errors

The following is a list of errors with their possible status codes and
explanations.

* _ExeccutionError_
  * `101` - Max number of steps reached while iterating through actions with the `run_actions` method.
  * `102` - The API responded with an `error` type response.
  * `103` - The API didn't specify which action to run.
  * `104` - The API wanted to run a method that doesn't exist locally.
  * `105` - The API responded with an unknown type.
  * `106` - There was a problem connecting to the API.

* _HttpError_
  * `100` - Unable to connect to server
