use crate::Array;

pub struct RingBuffer<T> {
    buffer: Array<T>,
    read_index: usize,
    write_index: usize,
    is_full: bool,
}

impl<T> RingBuffer<T> {
    pub fn new(buffer: &mut [T]) -> Self {
        RingBuffer {
            buffer: Array::new(buffer),
            read_index: 0,
            write_index: 0,
            is_full: false,
        }
    }

    pub fn read_index(&self) -> usize {
        self.read_index
    }

    pub fn write_index(&self) -> usize {
        self.write_index
    }

    pub fn push(&mut self, value: T)
    where
        T: Copy,
    {
        self.buffer[self.write_index] = value;
        if self.is_full {
            self.read_index = (self.read_index + 1) % self.capacity();
        }
        self.write_index = (self.write_index + 1) % self.capacity();
        self.is_full = self.write_index == self.read_index;
    }

    pub fn pop(&mut self) -> Option<T>
    where
        T: Copy,
    {
        if self.is_empty() {
            return None;
        }
        let value = self.buffer[self.read_index];
        self.read_index = (self.read_index + 1) % self.capacity();
        self.is_full = false;
        Some(value)
    }

    pub fn is_empty(&self) -> bool {
        !self.is_full && self.read_index == self.write_index
    }

    pub fn is_full(&self) -> bool {
        self.is_full
    }

    pub fn len(&self) -> usize {
        if self.is_full {
            self.capacity()
        } else if self.write_index >= self.read_index {
            self.write_index - self.read_index
        } else {
            self.capacity() - (self.read_index - self.write_index)
        }
    }

    pub fn capacity(&self) -> usize {
        self.buffer.capacity()
    }
}
