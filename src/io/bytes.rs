//! Implementations of bytes::BufMut and bytes::Buf for Ring

use std::io::BufRead;

use bytes::{buf::UninitSlice, Buf, BufMut};

use super::{Ring, SeqRead, SeqWrite};

impl Buf for Ring {
    fn remaining(&self) -> usize {
        self.read_len()
    }

    fn chunk(&self) -> &[u8] {
        self.as_read_slice(usize::MAX)
    }

    fn advance(&mut self, cnt: usize) {
        self.consume(cnt)
    }
}

unsafe impl BufMut for Ring {
    fn remaining_mut(&self) -> usize {
        self.write_len()
    }

    unsafe fn advance_mut(&mut self, cnt: usize) {
        self.feed(cnt)
    }

    fn chunk_mut(&mut self) -> &mut UninitSlice {
        UninitSlice::new(self.as_write_slice(usize::MAX))
    }
}

#[cfg(test)]
mod tests {
    use crate::page_size;

    use super::super::Ring;
    use bytes::{Buf, BufMut};

    #[test]
    fn test_bytes_sanity() {
        let size = page_size();
        let mut buf = Ring::new(size).expect("failed to create buffer");

        // empty case
        assert_eq!(buf.remaining(), 0);
        assert_eq!(buf.chunk().len(), 0);
        assert_eq!(buf.remaining_mut(), size);
        assert_eq!(buf.chunk_mut().len(), size);

        // write something
        buf.put_slice(b"hello world");
        assert_eq!(buf.remaining(), 11);
        assert_eq!(buf.chunk(), b"hello world");
        assert_eq!(buf.remaining_mut(), size - 11);
        assert_eq!(buf.chunk_mut().len(), size - 11);

        // consume some
        buf.advance(6);
        assert_eq!(buf.remaining(), 5);
        assert_eq!(buf.chunk(), b"world");
        assert_eq!(buf.remaining_mut(), size - 5);
        assert_eq!(buf.chunk_mut().len(), size - 5);
    }
}
