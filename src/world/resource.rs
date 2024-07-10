use std::{
    any::{Any, TypeId},
    ops::{Deref, DerefMut},
};

use atomic_refcell::{AtomicRef, AtomicRefCell, AtomicRefMut};
use rustc_hash::FxHashMap;

pub trait Resource: Any + Send + Sync + 'static {}
impl<T: Any + Send + Sync> Resource for T {}
impl dyn Resource {
    #[inline]
    pub fn is<R: Resource>(&self) -> bool {
        TypeId::of::<R>() == Any::type_id(self)
    }
    #[inline]
    pub fn downcast<R: Resource>(self: Box<Self>) -> Result<Box<R>, Box<Self>> {
        if self.is::<R>() {
            Ok(unsafe { Box::from_raw(Box::into_raw(self) as *mut R) })
        } else {
            Err(self)
        }
    }
    #[inline]
    pub fn downcast_ref<R: Resource>(&self) -> Option<&R> {
        if self.is::<R>() {
            Some(unsafe { &*(self as *const dyn Resource as *const R) })
        } else {
            None
        }
    }
    #[inline]
    pub fn downcast_mut<R: Resource>(&mut self) -> Option<&mut R> {
        if self.is::<R>() {
            Some(unsafe { &mut *(self as *mut dyn Resource as *mut R) })
        } else {
            None
        }
    }
}

pub struct Fetch<'a, R> {
    inner: AtomicRef<'a, R>,
}
impl<'a, R> Deref for Fetch<'a, R> {
    type Target = R;

    fn deref(&self) -> &Self::Target {
        self.inner.deref()
    }
}

pub struct FetchMut<'a, R> {
    inner: AtomicRefMut<'a, R>,
}
impl<'a, R> Deref for FetchMut<'a, R> {
    type Target = R;

    fn deref(&self) -> &Self::Target {
        self.inner.deref()
    }
}
impl<'a, R> DerefMut for FetchMut<'a, R> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.inner.deref_mut()
    }
}

#[derive(Default)]
pub struct ResourceMap {
    res: FxHashMap<TypeId, AtomicRefCell<Box<dyn Resource>>>,
}
impl ResourceMap {
    pub fn insert<R: Resource>(&mut self, resource: R) {
        self.res
            .insert(TypeId::of::<R>(), AtomicRefCell::new(Box::new(resource)));
    }

    pub fn remove<R: Resource>(&mut self) -> Option<R> {
        self.res
            .remove(&TypeId::of::<R>())
            .map(AtomicRefCell::into_inner)
            .map(|v: Box<dyn Resource>| v.downcast::<R>().unwrap_or_else(|_| unreachable!()))
            .map(|v: Box<R>| *v) // DerefMove
    }

    pub fn has_value<R: Resource>(&self) -> bool {
        self.res.contains_key(&TypeId::of::<R>())
    }

    pub fn fetch<R: Resource>(&self) -> Option<Fetch<R>> {
        self.res.get(&TypeId::of::<R>()).map(|v| Fetch {
            inner: AtomicRef::map(v.borrow(), |r| {
                r.downcast_ref::<R>().unwrap_or_else(|| unreachable!())
            }),
        })
    }

    pub fn fetch_mut<R: Resource>(&self) -> Option<FetchMut<R>> {
        self.res.get(&TypeId::of::<R>()).map(|v| FetchMut {
            inner: AtomicRefMut::map(v.borrow_mut(), |r| {
                r.downcast_mut::<R>().unwrap_or_else(|| unreachable!())
            }),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn resources() {
        let mut res_map = ResourceMap::default();
        res_map.insert(255_u8);
        {
            assert!(res_map.has_value::<u8>());
            let mut u = res_map.fetch_mut::<u8>().unwrap();
            *u -= 100;
            assert_eq!(*u, 255 - 100);
        }
        assert_eq!(Some(255 - 100), res_map.remove::<u8>());
    }
}
