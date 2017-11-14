# Change Log

All user visible changes to this project will be documented in this file.
This project adheres to [Semantic Versioning](http://semver.org/), as described
for Rust libraries in
[RFC #1105](https://github.com/rust-lang/rfcs/blob/master/text/1105-api-evolution.md)

## [ 1.0.0 ] - 2017-11-14

### Changed

* `witty::client` now only accepts a single argument - the token

### Removed

* `run_actions` method from `Client` because it will be deprecated by
  2018-01-01
* `converse` method from `Client` because it will be deprecated by
  2018-01-01
