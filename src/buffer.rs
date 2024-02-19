use std::fmt::Debug;
use std::ops::{Deref, DerefMut};
use std::sync::Mutex;

pub struct Buffer<T> {
    r: Mutex<T>,
    w: Mutex<T>,
}

impl<T> Default for Buffer<T>
    where T: Default,
{
    fn default() -> Self {
        return Self {
            r: T::default().into(),
            w: T::default().into(),
        };
    }
}

impl<T> Buffer<T> {
    pub fn with_constructor(f: impl Fn() -> T) -> Self {
        return Self {
            r: f().into(),
            w: f().into(),
        };
    }

    pub fn update(&self, update: impl FnOnce(&mut T)) {
        let mut w = self.w.lock().expect("Lock");
        update(&mut *w);

        let mut r = self.r.lock().expect("Lock");
        std::mem::swap(r.deref_mut(), w.deref_mut());
    }

    pub fn read<'s>(&'s self) -> impl Deref<Target=T> + 's {
        return self.r.lock().expect("Lock");
    }
}
