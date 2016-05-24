#[macro_use]
extern crate log;
#[macro_use]
extern crate quick_error;
extern crate rustc_serialize;
extern crate unicase;
extern crate traitobject;

pub mod error;
pub mod result;
pub mod field;
pub mod config;
mod item;
mod cowstr;

pub use config::Config;
pub use field::Field;

#[macro_export]
macro_rules! config_field {
    ($(#[$attr:meta])* struct $id:ident: $t:ty, key: $key:expr) => {
        __config_field!($(#[$attr])* field $id, $key, $t);
    };
    ($(#[$attr:meta])* struct $id:ident: $t:ty, key: $key:expr, default: $def:expr) => {
        __config_field!($(#[$attr])* field $id, $key, $t);
        __config_field_default!($id, $def);
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __config_field {
    ($(#[$attr:meta])* field $id:ident, $key:expr, $t:ty) => {

        $(#[$attr])*
        #[derive(Debug, Clone)]
        pub struct $id(pub $t);

        impl ::jconfig::field::Field for $id {
            fn key() -> &'static str { $key }

            fn decode(raw: &::jconfig::field::RawValue) -> ::jconfig::result::Result<Self> { ::jconfig::field::decode(raw).map(|x| $id(x)) }

            fn encode(&self) -> ::jconfig::field::RawValue { ::jconfig::field::encode(&self.0) }
        }

        impl ::std::fmt::Display for $id {
            fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result { write!(f, "{}", self.0) }
        }
    }
}

#[doc(hidden)]
#[macro_export]
macro_rules! __config_field_default {
    ($id:ident, $def:expr) => {
        impl Default for $id {
            fn default() -> Self {
                 $id($def)
            }
        }
    }
}
