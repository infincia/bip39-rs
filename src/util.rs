
pub(crate) struct BitWriter {
    offset: usize,
    remainder: u32,
    inner: Vec<u8>,
}

impl BitWriter {
    pub fn with_capacity(capacity: usize) -> Self {
        let mut bytes = capacity / 8;

        if capacity % 8 != 0 {
            bytes += 1;
        }

        Self {
            offset: 0,
            remainder: 0,
            inner: Vec::with_capacity(bytes)
        }
    }

    pub fn push(&mut self, source: u16, bits: usize) {
        debug_assert!(bits > 0 && bits <= 16, "bits out of range");

        let shift = 32 - bits;

        self.remainder |= ((source as u32) << shift) >> self.offset;
        self.offset += bits;

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

pub(crate) fn truncate(mut source: Vec<u8>, size: usize) -> Vec<u8> {
    source.truncate(size);
    source
}

pub(crate) fn checksum(source: u8, bits: usize) -> u8 {
    debug_assert!(bits <= 8, "Can operate on 8-bit integers only");

    source >> (8 - bits)
}
