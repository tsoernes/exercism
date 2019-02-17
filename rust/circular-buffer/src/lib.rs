pub struct CircularBuffer<T> {
    // The oldest element, if any, is at index 'oldest'. The age of the elements
    // increase to the right of 'oldest' (and wraps around if 'oldest' > 0).
    buffer: Vec<T>,
    oldest: usize,
}

#[derive(Debug, PartialEq)]
pub enum Error {
    EmptyBuffer,
    FullBuffer,
}

impl<T> CircularBuffer<T> {
    pub fn new(capacity: usize) -> Self {
        CircularBuffer {
            buffer: Vec::with_capacity(capacity),
            oldest: 0,
        }
    }

    fn is_full(&self) -> bool {
        self.buffer.len() == self.buffer.capacity()
    }

    pub fn write(&mut self, element: T) -> Result<(), Error> {
        if self.is_full() {
            return Err(Error::FullBuffer);
        }
        self.buffer.push(element);
        Ok(())
    }

    pub fn read(&mut self) -> Result<T, Error> {
        if self.buffer.is_empty() {
            return Err(Error::EmptyBuffer);
        }
        let element = self.buffer.remove(self.oldest);
        // Vec::remove automatically shifts elements to the left,
        // thus 'oldest' does not need to be changed unless we just
        // popped the last element of the vector.
        if self.oldest == self.buffer.len() {
            self.oldest = 0;
        }
        Ok(element)
    }

    pub fn clear(&mut self) {
        self.oldest = 0;
        self.buffer.clear();
    }

    pub fn overwrite(&mut self, element: T) {
        if self.is_full() {
            self.buffer[self.oldest] = element;
            self.oldest = (self.oldest + 1) % self.buffer.capacity();
        } else {
            self.buffer.push(element);
        }
    }
}
