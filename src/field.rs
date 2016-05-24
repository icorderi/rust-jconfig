// Based on the `Header` type present on `hyper`
// https://github.com/hyperium/hyper
// Original work by: Sean McArthur

use std::fmt::Display;
use std::borrow::Cow;
use std::any::Any;

use rustc_serialize::json::{self, Json};
use rustc_serialize::{Decodable, Encodable};

use result::Result;

pub type RawValue = Json;

/// Any object that will represent a config field.
pub trait Field: Display + Any + FieldClone {
    /// Returns the key of the config field.
    fn key() -> &'static str where Self: Sized;

    fn decode(raw: &RawValue) -> Result<Self> where Self: Sized;

    fn encode(&self) -> RawValue;
}

#[doc(hidden)]
pub trait FieldClone {
    fn clone_box(&self) -> Box<Field>;
}

impl<T: Field + Clone> FieldClone for T {
    #[inline]
    fn clone_box(&self) -> Box<Field> { Box::new(self.clone()) }
}

#[inline]
pub fn decode<T: Decodable>(raw: &RawValue) -> Result<T>
    where T: Sized
{
    let s = raw.to_string();
    let t = try!(json::decode(&s));
    Ok(t)
}

#[inline]
pub fn encode<T: Encodable>(x: &T) -> RawValue {
    let s = json::encode(x).unwrap_or("".to_string());
    return Json::from_str(&s).unwrap_or(Json::Null);
}

/// Trait for viewing the contents of a `Field` structure.
pub trait FieldRef {
    /// View a reference to a field if it exists.
    fn get<V>(&self) -> Option<&V> where V: Field;

    /// View a reference to the `RawValue` of the field if it exists.
    fn get_raw(&self, name: &str) -> Option<RawValue>;
}

/// Trait for manipulating the contents of a field structure.
pub trait FieldMut {
    /// Set a field to the given value.
    fn set<V: Field>(&mut self, value: V);

    /// Set a field to the given raw value.
    fn set_raw<K>(&mut self, name: K, value: RawValue) where K: Into<Cow<'static, str>>;
}

impl<'a, T: FieldRef + ?Sized> FieldRef for &'a T {
    fn get<V: Field>(&self) -> Option<&V> { FieldRef::get::<V>(*self) }
    fn get_raw(&self, name: &str) -> Option<RawValue> { FieldRef::get_raw(*self, name) }
}

impl<'a, T: FieldRef + ?Sized> FieldRef for &'a mut T {
    fn get<V: Field>(&self) -> Option<&V> { FieldRef::get::<V>(*self) }
    fn get_raw(&self, name: &str) -> Option<RawValue> { FieldRef::get_raw(*self, name) }
}

impl<'a, T: FieldMut + ?Sized> FieldMut for &'a mut T {
    fn set<V: Field>(&mut self, value: V) { FieldMut::set(*self, value) }

    fn set_raw<K>(&mut self, name: K, value: RawValue)
        where K: Into<Cow<'static, str>>
    {
        FieldMut::set_raw(*self, name, value)
    }
}
