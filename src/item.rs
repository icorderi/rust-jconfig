// Based on an internal type `Item` present on `hyper`
// https://github.com/hyperium/hyper
// Original work by: Sean McArthur

use std::any::Any;
use std::any::TypeId;
use std::fmt;
use std::cell::UnsafeCell;
use std::mem;

use traitobject;

use field::{Field, RawValue};
use result::Result;

pub struct ConfigItem {
    inner: UnsafeCell<Inner>,
}

enum Inner {
    Raw(RawValue),
    Typed(TypeId, Box<Field>),
}

impl Clone for ConfigItem {
    #[inline]
    fn clone(&self) -> Self { ConfigItem { inner: UnsafeCell::new(unsafe { (*self.inner.get()).clone() }) } }
}

impl Clone for Inner {
    #[inline]
    fn clone(&self) -> Self {
        match *self {
            Inner::Raw(ref raw) => Inner::Raw(raw.clone()),
            Inner::Typed(tid, ref bv) => Inner::Typed(tid, bv.clone_box()),
        }
    }
}

impl Field {
    #[inline]
    unsafe fn downcast_ref_unchecked<T: 'static>(&self) -> &T { mem::transmute(traitobject::data(self)) }

    #[inline]
    unsafe fn downcast_mut_unchecked<T: 'static>(&mut self) -> &mut T {
        mem::transmute(traitobject::data_mut(self))
    }
}

impl ConfigItem {
    #[inline]
    pub fn new_raw(raw: RawValue) -> Self { ConfigItem { inner: UnsafeCell::new(Inner::Raw(raw)) } }

    #[inline]
    pub fn new_typed<V: Field + Any + 'static>(v: V) -> Self {
        let tid = TypeId::of::<V>();
        ConfigItem { inner: UnsafeCell::new(Inner::Typed(tid, Box::new(v))) }
    }

    pub fn raw(&self) -> RawValue {
        let inner = unsafe { &*self.inner.get() };
        match *inner {
            Inner::Raw(ref raw) => raw.clone(),
            Inner::Typed(_, ref bv) => bv.encode(),
        }
    }

    pub fn typed<V: Field + Any>(&self) -> Option<&V> {
        let tid = TypeId::of::<V>();

        let mut inner = unsafe { &mut *self.inner.get() };

        // parse if missing
        let bv = match *inner {
            Inner::Raw(ref raw) => parse::<V>(raw).ok(),
            _ => None,
        };

        if let Some(bv) = bv {
            // mutate inner
            mem::replace(inner, Inner::Typed(tid, bv));
        }

        match *inner {
            Inner::Typed(t, ref bv) if t == tid => Some(unsafe { bv.downcast_ref_unchecked() }),
            _ => None,
        }
    }

    pub fn typed_mut<V: Field + Any>(&mut self) -> Option<&mut V> {
        let tid = TypeId::of::<V>();

        let mut inner = unsafe { &mut *self.inner.get() };

        // parse if missing
        let bv = match *inner {
            Inner::Raw(ref raw) => parse::<V>(raw).ok(),
            _ => None,
        };

        if let Some(bv) = bv {
            // mutate inner
            mem::replace(inner, Inner::Typed(tid, bv));
        }

        match *inner {
            Inner::Typed(t, ref mut bv) if t == tid => Some(unsafe { bv.downcast_mut_unchecked() }),
            _ => None,
        }
    }
}

#[inline]
fn parse<V: Field + 'static>(raw: &RawValue) -> Result<Box<Field>> {
    Field::decode(&raw).map(|v: V| {
        // FIXME: Use Type ascription
        let h: Box<Field> = Box::new(v);
        h
    })
}

impl fmt::Display for ConfigItem {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let inner = unsafe { &*self.inner.get() };
        match *inner {
            Inner::Raw(ref raw) => write!(f, "{}", raw),
            Inner::Typed(_, ref bv) => write!(f, "{}", bv),
        }
    }
}
