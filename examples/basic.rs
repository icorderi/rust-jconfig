#[macro_use]
extern crate jconfig;

use jconfig::Config;

fn main() {
    let raw = r#"{"port":12345,"name":"test"}"#;

    // Create from raw JSON, you can also do Config::load("my-config.json")
    let config = Config::from_str(raw).unwrap();

    // Try to get the field of type `Port`
    println!("{}", config.get::<Port>().unwrap());

    // Try to get the raw JSON for field "name"
    println!("{}", config.get_raw("name").unwrap());
}

config_field!{
    /// Indicates some port of sorts...
    ///
    /// This will show up in your documentation.
    struct Port: u16,
    key: "port"
}
