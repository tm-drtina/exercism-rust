use std::io::{Read, Result, Write};

pub struct ReadStats<R> {
    reader: R,
    reads: usize,
    bytes: usize,
}

impl<R: Read> ReadStats<R> {
    pub fn new(wrapped: R) -> Self {
        Self {
            reader: wrapped,
            reads: 0,
            bytes: 0,
        }
    }

    pub fn get_ref(&self) -> &R {
        &self.reader
    }

    pub fn bytes_through(&self) -> usize {
        self.bytes
    }

    pub fn reads(&self) -> usize {
        self.reads
    }
}

impl<R: Read> Read for ReadStats<R> {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        let read = self.reader.read(buf)?;
        self.reads += 1;
        self.bytes += read;
        Ok(read)
    }
}

pub struct WriteStats<W>{
    writer: W,
    writes: usize,
    bytes: usize,
}

impl<W: Write> WriteStats<W> {
    pub fn new(wrapped: W) -> Self {
        Self { writer: wrapped, writes: 0, bytes: 0 }
    }

    pub fn get_ref(&self) -> &W {
        &self.writer
    }

    pub fn bytes_through(&self) -> usize {
        self.bytes
    }

    pub fn writes(&self) -> usize {
        self.writes
    }
}

impl<W: Write> Write for WriteStats<W> {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        let written = self.writer.write(buf)?;
        self.writes += 1;
        self.bytes += written;

        Ok(written)
    }

    fn flush(&mut self) -> Result<()> {
        self.writer.flush()
    }
}
