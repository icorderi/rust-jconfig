// Based on the `CowStr type present on `hyper`
// https://github.com/hyperium/hyper
// Original work by: Sean McArthur

use std::borrow::Cow;
use std::fmt;
use std::ops::{Deref, DerefMut};

#[derive(Clone, Hash, Eq, PartialEq, PartialOrd, Ord)]
pub struct CowStr(Cow<'static, str>);

impl CowStr {
    pub fn new<S: Into<Cow<'static, str>>>(s: S) -> Self { CowStr(s.into()) }
}

impl Deref for CowStr {
    type Target = Cow<'static, str>;

    fn deref(&self) -> &Cow<'static, str> { &self.0 }
}

impl fmt::Debug for CowStr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result { fmt::Debug::fmt(&self.0, f) }
}

impl fmt::Display for CowStr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result { fmt::Display::fmt(&self.0, f) }
}

impl DerefMut for CowStr {
    fn deref_mut(&mut self) -> &mut Cow<'static, str> { &mut self.0 }
}

impl AsRef<str> for CowStr {
    fn as_ref(&self) -> &str { self }
}
