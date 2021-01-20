use std::io::{Read, Result, Write};

pub struct ReadStats<R: std::io::Read>(R,usize,usize);

impl<R: Read> ReadStats<R> {
    // _wrapped is ignored because R is not bounded on Debug or Display and therefore
    // can't be passed through format!(). For actual implementation you will likely
    // wish to remove the leading underscore so the variable is not ignored.
    pub fn new(wrapped: R) -> ReadStats<R> {
        ReadStats (wrapped,0,0)
    }

    pub fn get_ref(&self) -> &R {
        &self.0
    }

    pub fn bytes_through(&self) -> usize {
        self.2
    }

    pub fn reads(&self) -> usize {
        self.1
    }
}

impl<R: Read> Read for ReadStats<R> {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        let r = self.0.read(buf);
        match r {
            Ok(r)   =>  { self.1 += 1; self.2 += r; return Ok(r);},
            _       => r,
        }
    }
}

pub struct WriteStats<W: std::io::Write>(W,usize,usize);

impl<W: Write> WriteStats<W> {
    // _wrapped is ignored because W is not bounded on Debug or Display and therefore
    // can't be passed through format!(). For actual implementation you will likely
    // wish to remove the leading underscore so the variable is not ignored.
    pub fn new(wrapped: W) -> WriteStats<W> {
        WriteStats(wrapped,0,0)
    }

    pub fn get_ref(&self) -> &W {
        &self.0
    }

    pub fn bytes_through(&self) -> usize {
        self.2
    }

    pub fn writes(&self) -> usize {
        self.1
    }
}

impl<W: Write> Write for WriteStats<W> {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        let r = self.0.write(buf);
        match r {
             Ok(r)   =>  { self.1 += 1; self.2 += r; return Ok(r);},
             _       => r,
         }
    }
    fn flush(&mut self) -> Result<()> {
        self.0.flush()
    }
}
