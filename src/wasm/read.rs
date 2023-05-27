// Copyright © 2022-2023 The Nucleide Contributors.
//
// Licensed under any of:
// - Apache License, Version 2.0 (https://www.apache.org/licenses/LICENSE-2.0)
// - Boost Software License, Version 1.0 (https://www.boost.org/LICENSE_1_0.txt)
// - MIT License (https://mit-license.org/)
// At your choosing (See accompanying files LICENSE_APACHE_2_0.txt,
// LICENSE_MIT.txt and LICENSE_BOOST_1_0.txt).

mod seal {
    pub trait Seal {}
}

use alloc::collections::BTreeMap;

use crate::parse::Reader;

/// WebAssembly primitive reader methods
pub trait Read<'a>: self::seal::Seal {
    /// Parse the next ULEB128-encoded 32-bit unsigned integer.
    fn integer(&mut self) -> Option<u32>;

    /// Parse a WebAssembly "Name".
    fn name(&mut self) -> Option<&'a str>;

    /// Parse a WebAssembly "Name Map".
    fn name_map(&mut self) -> Option<BTreeMap<u32, &'a str>>;

    /// Parse a WebAssembly "Indirect Name Map".
    fn indirect_name_map(
        &mut self,
    ) -> Option<BTreeMap<u32, BTreeMap<u32, &'a str>>>;
}

impl self::seal::Seal for Reader<'_> {}

impl<'a> Read<'a> for Reader<'a> {
    fn integer(&mut self) -> Option<u32> {
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

    fn name(&mut self) -> Option<&'a str> {
        let len = self.integer()?.try_into().ok()?;

        self.str(len)
    }

    fn name_map(&mut self) -> Option<BTreeMap<u32, &'a str>> {
        let mut name_map = BTreeMap::new();

        for _ in 0..self.integer()? {
            name_map.insert(self.integer()?, self.name()?);
        }

        Some(name_map)
    }

    fn indirect_name_map(
        &mut self,
    ) -> Option<BTreeMap<u32, BTreeMap<u32, &'a str>>> {
        let mut indirect_name_map = BTreeMap::new();

        for _ in 0..self.integer()? {
            indirect_name_map.insert(self.integer()?, self.name_map()?);
        }

        Some(indirect_name_map)
    }
}
