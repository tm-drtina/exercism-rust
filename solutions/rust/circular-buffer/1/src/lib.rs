use std::mem::MaybeUninit;

pub struct CircularBuffer<T> {
    buf: Box<[MaybeUninit<T>]>,
    start: usize,
    end: usize,
    len: usize,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    EmptyBuffer,
    FullBuffer,
}

impl<T> CircularBuffer<T> {
    fn next_index(&self, cur: usize) -> usize {
        if cur + 1 == self.buf.len() {
            0
        } else {
            cur + 1
        }
    }

    pub fn new(capacity: usize) -> Self {
        Self {
            buf: Box::new_uninit_slice(capacity),
            start: 0,
            end: 0,
            len: 0,
        }
    }

    pub fn write(&mut self, element: T) -> Result<(), Error> {
        if self.len == self.buf.len() {
            Err(Error::FullBuffer)
        } else {
            self.buf[self.end].write(element);
            self.len += 1;
            self.end = self.next_index(self.end);
            Ok(())
        }
    }

    pub fn read(&mut self) -> Result<T, Error> {
        if self.len == 0 {
            Err(Error::EmptyBuffer)
        } else {
            let index = self.start;
            self.len -= 1;
            self.start = self.next_index(index);
            let val = std::mem::replace(&mut self.buf[index], MaybeUninit::uninit());
            let val = unsafe { val.assume_init() };
            Ok(val)
        }
    }

    pub fn clear(&mut self) {
        while let Ok(_) = self.read() { }
    }

    pub fn overwrite(&mut self, element: T) {
        if self.len == self.buf.len() {
            self.read().unwrap();
        }
        self.write(element).unwrap();
    }
}

impl<T> Drop for CircularBuffer<T> {
    fn drop(&mut self) {
        self.clear();
    }
}
