pub(crate) trait IterJoinExt {
    fn join(&mut self, &str) -> String;
}

impl<T: AsRef<str>, I: Iterator<Item = T>> IterJoinExt for I {
    fn join(&mut self, glue: &str) -> String {
        let first = match self.next() {
            Some(first) => first,
            None        => return String::new()
        };

        let mut buffer = String::with_capacity(128);

        buffer.push_str(first.as_ref());

        for item in self {
            buffer.push_str(glue);
            buffer.push_str(item.as_ref());
        }

        buffer
    }
}

pub(crate) trait Bits {
    const BITS: usize;
}

pub(crate) struct Bits11;

impl Bits for Bits11 {
    const BITS: usize = 11;
}

pub(crate) struct BitWriter<B: Bits> {
    _bits: B,
    offset: usize,
    remainder: u32,
    inner: Vec<u8>,
}

impl<B: Bits> BitWriter<B> {
    pub fn with_capacity(capacity: usize, bits: B) -> Self {
        let mut bytes = capacity / 8;

        if capacity % 8 != 0 {
            bytes += 1;
        }

        Self {
            _bits: bits,
            offset: 0,
            remainder: 0,
            inner: Vec::with_capacity(bytes)
        }
    }

    pub fn push(&mut self, source: u16) {
        let shift = 32 - B::BITS;

        self.remainder |= ((source as u32) << shift) >> self.offset;
        self.offset += B::BITS;

        while self.offset >= 8 {
            self.inner.push((self.remainder >> 24) as u8);
            self.remainder <<= 8;
            self.offset -= 8;
        }
    }

    pub fn len(&self) -> usize {
        self.inner.len() * 8 + self.offset
    }

    pub fn into_bytes(mut self) -> Vec<u8> {
        if self.offset != 0 {
            self.inner.push((self.remainder >> 24) as u8);
        }

        self.inner
    }
}


pub(crate) trait IterBitsExt: Iterator<Item = u8> + Sized {
    fn bits<B: Bits>(self, bits: B) -> BitIter<Self, B>;
}

impl<I: Iterator<Item = u8> + Sized> IterBitsExt for I {
    fn bits<B: Bits>(self, bits: B) -> BitIter<Self, B> {
        BitIter::new(self, bits)
    }
}

pub(crate) struct BitIter<I: Iterator<Item = u8> + Sized, B: Bits> {
    _bits: B,
    source: I,
    read: usize,
    buffer: u32,
}

impl<I: Iterator<Item = u8> + Sized, B: Bits> BitIter<I, B> {
    pub fn new(source: I, bits: B) -> Self {
        let source = source.into_iter();

        BitIter {
            _bits: bits,
            source,
            read: 0,
            buffer: 0,
        }
    }
}

impl<I: Iterator<Item = u8>, B: Bits> Iterator for BitIter<I, B> {
    type Item = u16;

    fn next(&mut self) -> Option<u16> {
        while self.read < B::BITS {
            let byte = self.source.next()?;

            self.read += 8;
            self.buffer |= (byte as u32) << (32 - self.read);
        }

        let result = (self.buffer >> (32 - B::BITS)) as u16;

        self.buffer <<= B::BITS;
        self.read -= B::BITS;

        Some(result)
    }
}

/// Truncate an owned `Vec`
pub(crate) fn truncate<T>(mut source: Vec<T>, size: usize) -> Vec<T> {
    source.truncate(size);
    source
}

/// Extract the first `bits` from the `source` byte
pub(crate) fn checksum(source: u8, bits: u8) -> u8 {
    debug_assert!(bits <= 8, "Can operate on 8-bit integers only");

    source >> (8 - bits)
}
