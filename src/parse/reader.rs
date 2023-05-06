// Copyright © 2022-2023 The Nucleide Contributors.
//
// Licensed under any of:
// - Apache License, Version 2.0 (https://www.apache.org/licenses/LICENSE-2.0)
// - Boost Software License, Version 1.0 (https://www.boost.org/LICENSE_1_0.txt)
// - MIT License (https://mit-license.org/)
// At your choosing (See accompanying files LICENSE_APACHE_2_0.txt,
// LICENSE_MIT.txt and LICENSE_BOOST_1_0.txt).

use core::str;

/// Reads from a buffer.
#[derive(Debug)]
pub struct Reader<'a>(&'a [u8]);

impl<'a> Reader<'a> {
    /// Create a new `Reader` on the provided `buffer`.
    pub fn new(buffer: &'a [u8]) -> Self {
        Self(buffer)
    }

    /// Parse the next byte
    pub fn u8(&mut self) -> Option<u8> {
        const SIZE: usize = core::mem::size_of::<u8>();

        let value = self.subslice(SIZE)?;

        value.get(0).copied()
    }

    /// Parse the next little-endian `u16`
    pub fn u16(&mut self) -> Option<u16> {
        const SIZE: usize = core::mem::size_of::<u16>();

        let value = self.subslice(SIZE)?;

        Some(u16::from_le_bytes(value.get(..SIZE)?.try_into().ok()?))
    }

    /// Parse the next little-endian `u32`
    pub fn u32(&mut self) -> Option<u32> {
        const SIZE: usize = core::mem::size_of::<u32>();

        let value = self.subslice(SIZE)?;

        Some(u32::from_le_bytes(value.get(..SIZE)?.try_into().ok()?))
    }

    /// Parse the next little-endian `u64`
    pub fn u64(&mut self) -> Option<u64> {
        const SIZE: usize = core::mem::size_of::<u64>();

        let value = self.subslice(SIZE)?;

        Some(u64::from_le_bytes(value.get(..SIZE)?.try_into().ok()?))
    }

    /// Parse the next little-endian `u128`
    pub fn u128(&mut self) -> Option<u128> {
        const SIZE: usize = core::mem::size_of::<u128>();

        let value = self.subslice(SIZE)?;

        Some(u128::from_le_bytes(value.get(..SIZE)?.try_into().ok()?))
    }

    /// Parse the next ULEB128 value
    pub fn uleb128(&mut self) -> Option<u32> {
        let mut value = 0;
        let mut shift = 0;

        while {
            let byte = self.u8()?;

            value |= u32::from(byte & 0x7f) << shift;
            shift += 7;

            let more = shift < u32::BITS;
            let fits_u32 = more || byte < 16;

            if byte & 0x80 == 0 && fits_u32 {
                return Some(value);
            }

            more
        } {}

        None
    }

    /// Parse a UTF-8 `String` of specified length
    pub fn str(&mut self, len: usize) -> Option<&'a str> {
        let bytes = self.subslice(len)?;

        str::from_utf8(bytes.get(..len)?).ok()
    }

    /// Return `Some(())` if end of buffer.
    pub fn end(&self) -> Option<()> {
        self.0.is_empty().then_some(())
    }

    /// Parse a WebAssembly "Name".
    pub fn name(&mut self) -> Option<&'a str> {
        let len = self.uleb128()?.try_into().ok()?;

        self.str(len)
    }

    fn subslice(&mut self, size: usize) -> Option<&'a [u8]> {
        if size > self.0.len() {
            return None;
        }

        let (slice, data) = self.0.split_at(size);

        self.0 = data;

        Some(slice)
    }
}
