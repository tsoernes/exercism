use std::io::{Read, Result, Write};

pub struct ReadStats<R> {
    source: R,
    n_bytes: usize,
    n_reads: usize,
}

impl<R: Read> ReadStats<R> {
    pub fn new(source: R) -> ReadStats<R> {
        ReadStats {
            source,
            n_bytes: 0,
            n_reads: 0,
        }
    }

    pub fn get_ref(&self) -> &R {
        &self.source
    }

    pub fn bytes_through(&self) -> usize {
        self.n_bytes
    }

    pub fn reads(&self) -> usize {
        self.n_reads
    }
}

impl<R: Read> Read for ReadStats<R> {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        let n_bytes = self.source.read(buf)?;
        self.n_bytes += n_bytes;
        self.n_reads += 1;
        Ok(n_bytes)
    }
}

pub struct WriteStats<W> {
    dest: W,
    n_bytes: usize,
    n_writes: usize,
}

impl<W: Write> WriteStats<W> {
    pub fn new(dest: W) -> WriteStats<W> {
        WriteStats {
            dest,
            n_bytes: 0,
            n_writes: 0,
        }
    }

    pub fn get_ref(&self) -> &W {
        &self.dest
    }

    pub fn bytes_through(&self) -> usize {
        self.n_bytes
    }

    pub fn writes(&self) -> usize {
        self.n_writes
    }
}

impl<W: Write> Write for WriteStats<W> {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        let n_bytes = self.dest.write(buf)?;
        self.n_bytes += n_bytes;
        self.n_writes += 1;
        Ok(n_bytes)
    }

    fn flush(&mut self) -> Result<()> {
        self.dest.flush()
    }
}
