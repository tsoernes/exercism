use std::mem;

pub struct CircularBuffer<T> {
    // The oldest element, if any, is at index 'oldest'. The age of the elements
    // increase to the right of 'oldest' (and wraps around if 'oldest' > 0).
    buffer: Vec<T>,
    oldest: usize,
    next: usize,
    is_empty: bool,
}

#[derive(Debug, PartialEq)]
pub enum Error {
    EmptyBuffer,
    FullBuffer,
}

impl<T> CircularBuffer<T>
where
    T: Default,
{
    pub fn new(capacity: usize) -> Self {
        let mut zeros = Vec::with_capacity(capacity);
        for _ in 0..capacity {
            zeros.push(Default::default());
        }
        CircularBuffer {
            buffer: zeros,
            oldest: 0,
            next: 0,
            is_empty: true,
        }
    }

    fn is_full(&self) -> bool {
        self.oldest == self.next && !self.is_empty
    }

    pub fn write(&mut self, element: T) -> Result<(), Error> {
        if self.is_full() {
            return Err(Error::FullBuffer);
        }
        self.is_empty = false;
        self.buffer[self.next] = element;
        self.next = (self.next + 1) % self.buffer.len();
        Ok(())
    }

    pub fn read(&mut self) -> Result<T, Error> {
        if self.is_empty {
            return Err(Error::EmptyBuffer);
        }
        let mut empty = Default::default();
        mem::swap(&mut empty, self.buffer.get_mut(self.oldest).unwrap());
        self.oldest = (self.oldest + 1) % self.buffer.len();
        if self.oldest == self.next {
            self.is_empty = true;
        }
        Ok(empty)
    }

    pub fn clear(&mut self) {
        for i in 0..self.buffer.capacity() {
            self.buffer[i] = Default::default();
        }
        self.oldest = 0;
        self.next = 0;
        self.is_empty = true
    }

    pub fn overwrite(&mut self, element: T) {
        self.is_empty = false;
        self.buffer[self.next] = element;
        if self.is_full() {
            self.oldest = (self.oldest + 1) % self.buffer.capacity();
        }
        self.next = (self.next + 1) % self.buffer.len();
    }
}
