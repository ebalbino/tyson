use core::convert::AsRef;
use core::fmt::Debug;
use core::ops::{Deref, DerefMut};

#[derive(Clone, Copy)]
pub struct Array<T> {
    buffer: *mut T,
    capacity: usize,
    len: usize,
}

impl<T> Array<T> {
    pub fn new(buffer: &mut [T]) -> Self {
        Array {
            buffer: buffer.as_mut_ptr(),
            capacity: buffer.len(),
            len: 0,
        }
    }

    pub fn buffer(&self) -> *mut T {
        self.buffer
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn capacity(&self) -> usize {
        self.capacity
    }

    pub fn push(&mut self, value: &T) -> Option<&T>
    where
        T: Copy,
    {
        let current = self.len();
        if current < self.capacity() {
            self[current] = *value;
            self.len += 1;
            Some(&self[current])
        } else {
            None
        }
    }

    pub fn pop(&mut self) -> Option<&T> {
        if self.len > 0 {
            self.len -= 1;
            Some(&self[self.len])
        } else {
            None
        }
    }

    pub fn concat(&mut self, other: &[T]) -> Option<&[T]>
    where
        T: Copy,
    {
        let current = self.len();
        if current + other.len() <= self.capacity {
            self[current..current + other.len()].copy_from_slice(other);
            self.len += other.len();
            Some(&self[..self.len])
        } else {
            None
        }
    }

    pub fn insert(&mut self, index: usize, value: &T) -> Option<&T>
    where
        T: Copy,
    {
        let len = self.len();
        if index <= len {
            for i in (index..len).rev() {
                self[i + 1] = self[i];
            }
            self[index] = *value;
            self.len += 1;
            Some(&self[index])
        } else {
            None
        }
    }

    pub fn remove(&mut self, index: usize) -> Option<T>
    where
        T: Copy,
    {
        let len = self.len();
        if index < len {
            let value = *&self[index];
            for i in index..len - 1 {
                self[i] = self[i + 1];
            }
            self.len -= 1;
            Some(value)
        } else {
            None
        }
    }

    pub fn clear(&mut self) {
        self.len = 0;
    }

    pub fn contains(&self, value: &T) -> bool
    where
        T: PartialEq,
    {
        for i in 0..self.len {
            if &self[i] == value {
                return true;
            }
        }
        false
    }

    pub fn iter(&self) -> core::slice::Iter<T> {
        let len = self.len;
        self[..len].iter()
    }

    pub fn iter_mut(&mut self) -> core::slice::IterMut<T> {
        let len = self.len;
        self[..len].iter_mut()
    }
}

impl<T> Deref for Array<T> {
    type Target = [T];

    fn deref(&self) -> &Self::Target {
        unsafe { core::slice::from_raw_parts(self.buffer, self.capacity) }
    }
}

impl<T> DerefMut for Array<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { core::slice::from_raw_parts_mut(self.buffer, self.capacity) }
    }
}

impl<T> AsRef<[T]> for Array<T> {
    fn as_ref(&self) -> &[T] {
        self.deref()
    }
}

impl<T> Debug for Array<T>
where
    T: Debug,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_list().entries(self.iter()).finish()
    }
}
