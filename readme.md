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
Additionally a map of available actions and a `send` action need to be
specified.

```Rust
extern crate witty;
use std::collections::BTreeMap;
use serde_json::Value;

struct Actions {}

impl Actions {
    pub fn send(request: Value, response: Value) {
        println!(
            "Sending message: '{}'",
            response.get("text").unwrap().as_str().unwrap_or("")
        )
    }

    pub fn weather(request: Value) -> Value {
        let mut context = match request.get("context") {
            Some(json) => json.to_owned(),
            _ => json!({})
        };

        context["temperature"] = json!("24°");
        context["cloudiness"] = json!("scattered clouds");
        context["windiness"] = json!("light brease");

        context
    }
}

fn main() {
    let token = "AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA";
    let mut actions: BTreeMap<String, fn(Value) -> Value> = BTreeMap::new();
    actions.insert("weather".to_string(), Actions::weather);

    let client = witty::client(token, actions, Actions::send);

    let result = client.run_actions(
        "test_session-201702171132-0",
        "What's the weather like?",
        json!({}),
        None
    );

    match result {
        Ok(context) => println!("Yay!"),
        Err(error) => println!("Something went wrong... {:?}", error)
    };
}
```

### Methods

The `Client` exposes three methods:

#### run_actions

This method, given a message, communicates with [Wit.ai](https://wit.ai/) API,
runs actions and sends out messages until the API instructs it to stop.

```Rust
run_actions(
    session_id: &str,
    message: &str,
    context: Value,
    max_steps: Option<i8>
) -> Result<Value, ExecutionError>
```

* `session_id` - Used to track sessions on the client and server.
* `message` - The input message which [wit.ai](https://wit.ai/) will process.
* `context` - The data being sent to the server, reference the official API reference to see all options
* `max_steps` - This is a 'safety net' feature. It specifies how many actions can be run on the client. If you have complex interactions you should raise this number to the maximum number of actions you have. E.g. `Some(12)`

The returned value is the final context sent by the server.

#### converse

This method gets the API's instructions. Successively calling this method would
get all instructions from the API.

```Rust
converse(
    session_id: &str,
    message: &str,
    context: Value,
    reset: bool
) -> Result<Value, super::http::HttpError>
```

* `session_id` - Same as in the `run_actions` method.
* `message` - Same as in the `run_actions` method.
* `context` - Same as in the `run_actions` method.
* `reset` - Resets the session

The returned value is the instruction with which the API responded.

#### message

Used to query the API for the meaning of an input message.

```Rust
message(
  message: &str
) -> Result<Value, super::http::HttpError>
```

* `message` - Same as in the `run_actions` method.

The returned value is the context with which the API responded.

### Errors

All error structs contain a `message` and a `code` field. The `message` field is
human-readable and describes the reason why this error occured alongside with
any additional information gathered from the environment. The `code` field
indicates where the error spawned and the class of issue it represents.

The following is a list of errors with their possible codes and
explanations:

* _ExeccutionError_
  * `101` - Max number of steps reached while iterating through actions with the `run_actions` method.
  * `102` - The API responded with an `error` type response.
  * `103` - The API didn't specify which action to run.
  * `104` - The API wanted to run a method that doesn't exist locally.
  * `105` - The API responded with an unknown type.
  * `106` - There was a problem connecting to the API.

* _HttpError_
  * `100` - Unable to connect to server
  * `101` - Server responded with message

__Note:__ The `HttpError` struct also has an additional `status` field which
contains the response's HTTP status code. This code will also be displayed in
the message alongside with the canonical explanation.

# License

This project is licensed under the [MIT license](LICENSE.txt). It's in no way
affiliated with [Wit.ai](https://wit.ai/).
