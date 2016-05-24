// Based on the `Headers` map type present on `hyper`
// https://github.com/hyperium/hyper
// Original work by: Sean McArthur

use std::borrow::Cow;
use std::collections::HashMap;
use std::collections::hash_map::{Entry, Iter};
use std::iter::FromIterator;
use std::fmt;
use std::mem;
use std::path::Path;
use std::io::Read;
use std::fs::File;

use unicase::UniCase;
use rustc_serialize::json::Json;

use field::{Field, FieldRef, FieldMut, RawValue};
use item::ConfigItem;
use result::Result;
use error::Error;
use cowstr::CowStr;

type Key = UniCase<CowStr>;

#[inline]
fn key<T: Field>() -> &'static str { <T as Field>::key() }

/// A map of the configuration entries
pub struct Config {
    data: HashMap<Key, ConfigItem>,
}

impl Default for Config {
    fn default() -> Self { Config::new() }
}

impl Config {
    /// Creates a new, empty configuration
    pub fn new() -> Self { Config { data: HashMap::new() } }

    #[doc(hidden)]
    pub fn from_raw_pairs(raw: &[(&str, RawValue)]) -> Result<Self> {
        let mut config = Config::new();
        for kv in raw {
            trace!("raw value: {}={}", kv.0, kv.1);
            let key = UniCase(CowStr::new(Cow::Owned(kv.0.to_owned())));
            match config.data.entry(key) {
                Entry::Vacant(entry) => entry.insert(ConfigItem::new_raw(kv.1.clone())),
                Entry::Occupied(_) => return Err(Error::DuplicateKeys), // duplicate config entries
            };
        }
        Ok(config)
    }

    /// Load a `Config` from a _JSON_ string
    pub fn from_str<'a>(s: &'a str) -> Result<Self> {
        match try!(Json::from_str(s)) {
            Json::Object(ref obj) => {
                let mut config = Config::new();
                for kv in obj {
                    trace!("raw value: {}={}", kv.0, kv.1);
                    let key = UniCase(CowStr::new(Cow::Owned(kv.0.to_owned())));
                    match config.data.entry(key) {
                        Entry::Vacant(entry) => entry.insert(ConfigItem::new_raw(kv.1.clone())),
                        Entry::Occupied(_) => return Err(Error::DuplicateKeys), // duplicate config entries
                    };
                }
                Ok(config)
            }
            _ => Err(Error::ExpectedJsonObject),
        }
    }

    /// Load a `Config` from a file
    pub fn load<P: AsRef<Path>>(path: P) -> Result<Self> {
        let mut f = try!(File::open(path));
        let mut j = String::new();
        f.read_to_string(&mut j).unwrap();
        let config = try!(Config::from_str(j.as_ref()));
        Ok(config)
    }

    /// Set a config field to the corresponding value.
    ///
    /// The field is determined by the type of the value being set.
    pub fn set<V: Field>(&mut self, value: V) {
        trace!("Config.set({}, {})", key::<V>(), &value);
        self.data.insert(UniCase(CowStr::new(Cow::Borrowed(key::<V>()))), ConfigItem::new_typed(value));
    }

    /// Access the raw value of a field.
    ///
    /// Prefer to use the typed getters instead.
    pub fn get_raw(&self, name: &str) -> Option<RawValue> {
        self.data
            .get(&UniCase(CowStr::new(Cow::Borrowed(unsafe { mem::transmute::<&str, &str>(name) }))))
            .map(ConfigItem::raw)
    }

    /// Set the raw value of a field, bypassing any typed fields.
    pub fn set_raw<K: Into<Cow<'static, str>>>(&mut self, name: K, value: RawValue) {
        let name = name.into();
        trace!("Config.set_raw({}, {:?})", name, value);
        self.data.insert(UniCase(CowStr::new(name)), ConfigItem::new_raw(value));
    }

    /// Remove a field by name
    pub fn remove_raw(&mut self, name: &str) {
        trace!("Config.remove_raw({})", name);
        self.data.remove(&UniCase(CowStr::new(Cow::Borrowed(unsafe { mem::transmute::<&str, &str>(name) }))));
    }

    /// Get a reference to a field's value, if it exists.
    pub fn get<V: Field>(&self) -> Option<&V> {
        self.data
            .get(&UniCase(CowStr::new(Cow::Borrowed(key::<V>()))))
            .and_then(ConfigItem::typed::<V>)
    }

    /// Get a mutable reference to a field's value, if it exists.
    pub fn get_mut<V: Field>(&mut self) -> Option<&mut V> {
        self.data
            .get_mut(&UniCase(CowStr::new(Cow::Borrowed(key::<V>()))))
            .and_then(ConfigItem::typed_mut::<V>)
    }

    /// Returns a boolean of whether a certain field is in the config.
    pub fn has<V: Field>(&self) -> bool {
        self.data.contains_key(&UniCase(CowStr::new(Cow::Borrowed(key::<V>()))))
    }

    /// Removes a field from the config, if one existed.
    /// Returns true if field has been removed.
    pub fn remove<V: Field>(&mut self) -> bool {
        trace!("Config.remove({})", key::<V>());
        self.data.remove(&UniCase(CowStr::new(Cow::Borrowed(key::<V>())))).is_some()
    }

    /// Returns an iterator over the fields.
    pub fn iter(&self) -> ConfigItems { ConfigItems { inner: self.data.iter() } }

    /// Returns the number of fields in the config.
    pub fn len(&self) -> usize { self.data.len() }

    /// Remove all fields from the config.
    pub fn clear(&mut self) { self.data.clear() }
}

impl FieldRef for Config {
    fn get<V: Field>(&self) -> Option<&V> { Config::get::<V>(self) }
    fn get_raw(&self, name: &str) -> Option<RawValue> { Config::get_raw(self, name) }
}


impl FieldMut for Config {
    fn set<V: Field>(&mut self, value: V) { Config::set(self, value) }

    fn set_raw<K>(&mut self, name: K, value: RawValue)
        where K: Into<Cow<'static, str>>
    {
        Config::set_raw(self, name, value)
    }
}

/// An `Iterator` over the fields in a `Config`.
pub struct ConfigItems<'a> {
    inner: Iter<'a, Key, ConfigItem>,
}

impl<'a> Iterator for ConfigItems<'a> {
    type Item = FieldView<'a>;

    fn next(&mut self) -> Option<FieldView<'a>> { self.inner.next().map(|(k, v)| FieldView(k, v)) }
}

/// Returned with the `ConfigItems` iterator.
pub struct FieldView<'a>(&'a Key, &'a ConfigItem);

impl<'a> FieldView<'a> {
    /// Check if a `FieldView `is a certain field.
    #[inline]
    pub fn is<V: Field>(&self) -> bool { UniCase(CowStr::new(Cow::Borrowed(key::<V>()))) == *self.0 }

    /// Get the field name as a slice.
    #[inline]
    pub fn key(&self) -> &'a str { self.0.as_ref() }

    /// Cast the value to a certain field type.
    #[inline]
    pub fn value<V: Field>(&self) -> Option<&'a V> { self.1.typed::<V>() }

    /// Get just the value as a `String`.
    #[inline]
    pub fn value_string(&self) -> String { (*self.1).to_string() }
}

impl<'a> fmt::Display for FieldView<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result { write!(f, "{}: {}", self.0, *self.1) }
}

impl<'a> fmt::Debug for FieldView<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result { fmt::Display::fmt(self, f) }
}

impl<'a> Extend<FieldView<'a>> for Config {
    fn extend<I: IntoIterator<Item = FieldView<'a>>>(&mut self, iter: I) {
        for kv in iter {
            self.data.insert((*kv.0).clone(), (*kv.1).clone());
        }
    }
}

impl<'a> FromIterator<FieldView<'a>> for Config {
    fn from_iter<I: IntoIterator<Item = FieldView<'a>>>(iter: I) -> Config {
        let mut config = Config::new();
        config.extend(iter);
        config
    }
}
