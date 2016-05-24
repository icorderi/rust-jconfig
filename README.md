# rust-jconfig

Configuration / settings backed by JSON files

## Dashboard

| Linux CI | Test Coverage | Crate | Documentation |
|:--------:|:-------------:|:-----:|:-------------:|
| [![Build Status](https://travis-ci.org/icorderi/rust-jconfig.svg?branch=master)](https://travis-ci.org/icorderi/rust-jconfig) | [![Coverage Status](https://coveralls.io/repos/icorderi/rust-jconfig/badge.svg?branch=master)](https://coveralls.io/r/icorderi/rust-jconfig?branch=master) | [![Crate](http://meritbadge.herokuapp.com/jconfig)](https://crates.io/crates/jconfig) | [![Docs](https://img.shields.io/badge/docs-up--to--date-blue.svg)](http://icorderi.github.io/rust-jconfig/index.html)

## Basic usage

```rust
use jconfig::Config;

fn main() {
    // Load the config
    let config = Config::load("my-config.json").unwrap();

    // Try to get the field of type `Port`
    println!("{}", config.get::<Port>().unwrap());

    // Try to get the raw JSON for field "name"
    println!("{}", config.get_raw("name").unwrap());
}
```

```json
// your my-config.json
{
    "port": 12345,
    "name": "foo"
}
```

Don't forget to check out the [examples](./examples)

## The `config_field!()` macro

The crate defines a macro `config_field!` that can simplify the consutrction of configuration field types.

```rust
#[macro_use]
extern crate jconfig;

config_field!{
    /// Indicates some port of sorts...
    ///
    /// This will show up in your documentation.
    struct Port: u16,
    key: "port",
    default: 80 // this field is optional
}
```

## License

Licensed under:

- Apache License, Version 2.0 - [LICENSE-APACHE](LICENSE-APACHE) ([source](http://www.apache.org/licenses/LICENSE-2.0))
- MIT license - ([LICENSE-MIT](LICENSE-MIT) ([source](http://opensource.org/licenses/MIT))

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.
