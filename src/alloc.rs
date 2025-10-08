use core::cell::Cell;
use core::ffi::c_void;
use core::fmt::Write;
use core::marker::PhantomData;
use core::ops::{Deref, DerefMut};
use core::ptr::NonNull;
use libc::{MAP_ANONYMOUS, MAP_PRIVATE, PROT_READ, PROT_WRITE, mmap, munmap};

type Byte = u8;

#[derive(Debug)]
pub struct MemoryBlock {
    buffer: *mut Byte,
    capacity: usize,
    len: Cell<usize>,
}

#[derive(Debug)]
pub struct Arena<'a> {
    buffer: *mut Byte,
    capacity: usize,
    len: Cell<usize>,
    phantom: PhantomData<&'a MemoryBlock>,
}

#[derive(Debug)]
pub struct StrSink<'a> {
    buffer: &'a mut [Byte],
    used: Cell<usize>,
}

impl MemoryBlock {
    pub fn with_capacity(capacity: usize) -> Self {
        let buffer = unsafe {
            mmap(
                core::ptr::null_mut(),
                capacity,
                PROT_READ | PROT_WRITE,
                MAP_ANONYMOUS | MAP_PRIVATE,
                0,
                0,
            ) as *mut Byte
        };

        MemoryBlock {
            buffer,
            capacity,
            len: Cell::new(0),
        }
    }

    pub fn buffer(&self) -> *mut Byte {
        self.buffer
    }

    pub fn len(&self) -> usize {
        self.len.get()
    }

    pub fn capacity(&self) -> usize {
        self.capacity
    }

    pub fn arena<'a>(&self, size: usize) -> Option<Arena<'a>> {
        let current = self.len();
        let end = current + size;

        if end > self.capacity {
            return None;
        }

        self.len.set(end);
        let slice = &self[current..end];
        Some(unsafe { Arena::from_raw_parts_mut(slice.as_ptr() as *mut Byte, slice.len()) })
    }

    pub fn split(&self) -> (&[Byte], &[Byte]) {
        self[..].split_at(self.len())
    }

    pub fn allocated(&self) -> &[Byte] {
        self.split().0
    }

    pub unsafe fn remaining(&self) -> &mut [Byte] {
        let remaining = self.split().1;

        unsafe { core::slice::from_raw_parts_mut(remaining.as_ptr() as *mut Byte, remaining.len()) }
    }
}

impl Arena<'_> {
    pub unsafe fn from_raw_parts_mut(ptr: *mut Byte, capacity: usize) -> Self {
        unsafe {
            core::ptr::write_bytes(ptr, 0, capacity);
        }

        Arena {
            buffer: ptr,
            capacity,
            len: Cell::new(0),
            phantom: PhantomData::default(),
        }
    }

    pub fn len(&self) -> usize {
        self.len.get()
    }

    pub fn capacity(&self) -> usize {
        self.capacity
    }

    pub fn seek(&self, offset: usize) -> bool {
        if offset <= self.capacity() {
            self.len.set(offset);
            return true;
        }
        false
    }

    pub fn allocate<T>(&self, len: usize) -> Option<NonNull<T>> {
        let size = core::mem::size_of::<T>();
        let align = core::mem::align_of::<T>();
        let current = self.len();
        let offset = (current + align - 1) & !(align - 1);
        let new_offset = offset + (size * len);

        if new_offset <= self.capacity() {
            self.seek(new_offset);
            return NonNull::new(&self[offset] as *const u8 as *mut T);
        }

        None
    }

    pub fn clear(&self) -> usize {
        self.len.replace(0)
    }

    pub fn split(&self) -> (&[Byte], &[Byte]) {
        self[..].split_at(self.len())
    }

    pub fn allocated(&self) -> &[Byte] {
        self.split().0
    }

    pub unsafe fn remaining(&self) -> &mut [Byte] {
        let remaining = self.split().1;

        unsafe { core::slice::from_raw_parts_mut(remaining.as_ptr() as *mut Byte, remaining.len()) }
    }
}

impl Drop for MemoryBlock {
    fn drop(&mut self) {
        unsafe {
            munmap(self.buffer as *mut c_void, self.capacity);
        }
    }
}

impl Deref for MemoryBlock {
    type Target = [Byte];

    fn deref(&self) -> &Self::Target {
        unsafe { core::slice::from_raw_parts(self.buffer, self.capacity) }
    }
}

impl DerefMut for MemoryBlock {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { core::slice::from_raw_parts_mut(self.buffer, self.capacity) }
    }
}

impl<'a> Drop for Arena<'_> {
    fn drop(&mut self) {
        let len = self.len.get();
        unsafe {
            core::ptr::write_bytes(self.buffer, 0, len);
        }
    }
}

impl<'a> Deref for Arena<'_> {
    type Target = [Byte];

    fn deref(&self) -> &Self::Target {
        unsafe { core::slice::from_raw_parts(self.buffer, self.capacity) }
    }
}

impl<'a> DerefMut for Arena<'_> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { core::slice::from_raw_parts_mut(self.buffer, self.capacity) }
    }
}

impl<'a> StrSink<'a> {
    pub fn new(buffer: &'a mut [Byte]) -> Self {
        StrSink {
            buffer,
            used: Cell::new(0),
        }
    }

    pub fn used(&self) -> usize {
        self.used.get()
    }

    pub fn as_str(self) -> Option<&'a str> {
        let used = self.used.get();
        if used != 0 && used + 1 < self.buffer.len() {
            self.buffer[used + 1] = b'\0';
            Some(unsafe { core::str::from_utf8_unchecked(&self.buffer[..used]) })
        } else {
            None
        }
    }
}

impl Write for StrSink<'_> {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        let used = self.used.get();
        if used + s.len() + 1 >= self.buffer.len() {
            return Err(core::fmt::Error);
        }

        let remaining_buf = &mut self.buffer[used..];
        let raw_s = s.as_bytes();
        let write_num = core::cmp::min(raw_s.len(), remaining_buf.len());
        remaining_buf[..write_num].copy_from_slice(&raw_s[..write_num]);
        self.used.set(used + raw_s.len());

        if write_num < raw_s.len() {
            Err(core::fmt::Error)
        } else {
            Ok(())
        }
    }
}
