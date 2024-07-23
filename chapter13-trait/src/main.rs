use std::fmt;
use std::fmt::{Formatter, write};
use std::io::Write;

struct BufferBuilder {
    buf: Vec<u8>,
}


impl BufferBuilder {
    pub fn new() -> Self {
        Self {
            buf: Vec::with_capacity(1024),
        }
    }
}

impl fmt::Debug for BufferBuilder {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", String::from_utf8_lossy(&self.buf))
    }
}

impl Write for BufferBuilder {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.buf.extend_from_slice(buf);
        Ok(buf.len())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}


fn main() {
    let mut buf = BufferBuilder::new();
    buf.write_all("Hello World!".as_ref()).unwrap();
    println!("{:?}", buf);
}
