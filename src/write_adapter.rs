use std::{fmt, io};

// thanks, stephaneyfx: https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=fc473d7fffb1cb07e8e2c6b1dad65ced
pub struct WriteAdapter<W>(W);

impl<W> WriteAdapter<W> {
    pub fn adapt(from: W) -> WriteAdapter<W> {
        WriteAdapter(from)
    }
}

impl<W: io::Write> fmt::Write for WriteAdapter<W> {
    fn write_str(&mut self, s: &str) -> Result<(), fmt::Error> {
        self.0.write_all(s.as_bytes()).map_err(|_| fmt::Error)
    }
}

impl<W: fmt::Write> io::Write for WriteAdapter<W> {
    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }

    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        let s = std::str::from_utf8(buf)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
        self.0
            .write_str(s)
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
        Ok(s.len())
    }
}
