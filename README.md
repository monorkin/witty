# Witty

Unofficial [wit.ai](https://wit.ai/) client library for Rust.

## Disclamer

This is an unofficial client implementation! It comes with no guarantees and
[Wit.ai](https://wit.ai/) can't be held responsible for any bugs that may occure
from using this library in your project.

This is a port of [Wit.ai's](https://wit.ai/) official
[Ruby client](https://github.com/wit-ai/wit-ruby) to Rust

## Notice

Wit.ai decided to close down their chat bot / conversation API on
2018-01-01. This decision lead to the deprecation of the `run_actions` and
`converse` methods.

In consequence, this library now only handles communication with Wit.ai servers.

Reference the [change log](CHANGELOG.md) for detailed explanations as to what
has been done to facilitate the transition.

## Installation

This project is available on [Crates.io](https://crates.io/crates/witty), and
can be installed with `cargo`.

```Cargo
[dependencies]
witty = "1.0.0"
```

## Usage

To create a client you will need to get a token from [Wit.ai](https://wit.ai/).

```Rust
extern crate witty;
use witty::Value; # Serde JSON value struct

fn main() {
    let token = "AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA";
    let client = witty::client(token);
    let result = client.message("What's the weather like?");

    match result {
        Ok(response) => println!("The server returns: {:?}", response),
        Err(error) => println!("Something went wrong... {:?}", error)
    };
}
```

### Methods

The `Client` exposes a single method:

#### message

Used to query the API for the meaning of an input message.

```Rust
message(
  message: &str
) -> Result<Value, super::http::HttpError>
```

* `message` - The input message which [wit.ai](https://wit.ai/) will process.

The returned value is the context with which the API responded.

### Errors

All error structs contain a `message` and a `code` field. The `message` field is
human-readable and describes the reason why this error occured alongside with
any additional information gathered from the environment. The `code` field
indicates where the error spawned and the class of issue it represents.

The following is a list of errors with their possible codes and
explanations:

* _ExeccutionError_
  * `102` - The API responded with an `error` type response.
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
