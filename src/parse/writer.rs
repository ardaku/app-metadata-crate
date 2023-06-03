// Copyright © 2022-2023 The Nucleide Contributors.
//
// Licensed under any of:
// - Apache License, Version 2.0 (https://www.apache.org/licenses/LICENSE-2.0)
// - Boost Software License, Version 1.0 (https://www.boost.org/LICENSE_1_0.txt)
// - MIT License (https://mit-license.org/)
// At your choosing (See accompanying files LICENSE_APACHE_2_0.txt,
// LICENSE_MIT.txt and LICENSE_BOOST_1_0.txt).

use alloc::vec::Vec;

use crate::parse::UInt;

/// Writes to a buffer.
#[derive(Debug)]
pub struct Writer<'a>(&'a mut Vec<u8>);

impl<'a> Writer<'a> {
    /// Create a new `Writer` to the provided `buffer`.
    pub fn new(buffer: &'a mut Vec<u8>) -> Self {
        Self(buffer)
    }

    /// Write out `value` in ULEB128 encoding.
    pub fn uleb128<T: UInt>(&mut self, value: T) {
        let mut remaining = value;

        while {
            let byte = remaining.little();

            remaining >>= 7;

            let more = remaining != T::ZERO;

            self.u8(if more { byte | 0x80 } else { byte & !0x80 });

            more
        } {}
    }

    /// Write out a byte
    pub fn u8(&mut self, byte: u8) {
        self.0.push(byte);
    }

    /// Write out a little-endian encoded 2-byte integer.
    pub fn u16(&mut self, int: u16) {
        self.bytes(int.to_le_bytes());
    }

    /// Write out a little-endian encoded 4-byte integer.
    pub fn u32(&mut self, int: u32) {
        self.bytes(int.to_le_bytes());
    }

    /// Write out a little-endian encoded 8-byte integer.
    pub fn u64(&mut self, int: u64) {
        self.bytes(int.to_le_bytes());
    }

    /// Write out a little-endian encoded 16-byte integer.
    pub fn u128(&mut self, int: u128) {
        self.bytes(int.to_le_bytes());
    }

    /// Write out a UTF-8 string slice (does not include length).
    pub fn str(&mut self, string: impl AsRef<str>) {
        self.bytes(string.as_ref().as_bytes())
    }

    /// Write out raw bytes.
    pub fn bytes(&mut self, bytes: impl AsRef<[u8]>) {
        self.0.extend(bytes.as_ref())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{parse::Reader, wasm::Read as _};

    #[test]
    fn roundtrip() {
        let mut buffer = Vec::new();
        let mut writer = Writer::new(&mut buffer);
        for i in (0..=u32::from(u16::MAX))
            .chain((u32::MAX - u32::from(u16::MAX))..=u32::MAX)
        {
            writer.uleb128(i);
            assert!(writer.0.len() < 7);
            let mut reader = Reader::new(&writer.0[..]);
            let j = reader.integer().unwrap();
            assert_eq!(i, j);
            assert!(reader.end().is_some());
            writer.0.clear();
        }
        for i in (u64::from(u32::MAX) + 1)
            ..(u64::from(u32::MAX) + u64::from(u16::MAX))
        {
            writer.uleb128(i);
            let mut reader = Reader::new(&writer.0[..]);
            let decoded = reader.integer();
            assert!(decoded.is_none(), "{i} decoded is {decoded:?}");
            writer.0.clear();
        }
    }
}
