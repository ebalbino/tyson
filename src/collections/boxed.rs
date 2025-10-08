use core::{
    fmt::Debug,
    ops::{Deref, DerefMut},
};

#[derive(Copy, Clone)]
pub struct Box<T> {
    inner: *mut T,
}

impl<T> Box<T> {
    pub fn new(data: &mut T) -> Self {
        Self {
            inner: data as *mut _,
        }
    }
}

impl<T> DerefMut for Box<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *self.inner }
    }
}

impl<T> Deref for Box<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { &*self.inner }
    }
}

impl<T> Debug for Box<T>
where
    T: Debug,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        Debug::fmt(&**self, f)
    }
}

impl<T> PartialEq for Box<T>
where
    T: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.deref() == other.deref()
    }
}
