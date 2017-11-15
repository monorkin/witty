# Change Log

All user visible changes to this project will be documented in this file.
This project adheres to [Semantic Versioning](http://semver.org/), as described
for Rust libraries in
[RFC #1105](https://github.com/rust-lang/rfcs/blob/master/text/1105-api-evolution.md)

## [ 1.0.0 ] - 2017-11-14

### Changed

* `witty::Value` is now publicly exposed (no need to import serde_json)
* `witty::client` now only accepts a single argument - the token
* Updated `serde` and `serde_json` to version `1.x.x`
* Updated `hyper` to version `0.11.x`
* Dropped `hyper-native-tls` in favor of `hyper-rustls`

### Removed

* `run_actions` method from `Client` because it will be deprecated by
  2018-01-01
* `converse` method from `Client` because it will be deprecated by
  2018-01-01
* `ExeccutionError` codes `101`, `103`, `104` are no longer possible
