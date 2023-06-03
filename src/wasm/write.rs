// Copyright © 2022-2023 The Nucleide Contributors.
//
// Licensed under any of:
// - Apache License, Version 2.0 (https://www.apache.org/licenses/LICENSE-2.0)
// - Boost Software License, Version 1.0 (https://www.boost.org/LICENSE_1_0.txt)
// - MIT License (https://mit-license.org/)
// At your choosing (See accompanying files LICENSE_APACHE_2_0.txt,
// LICENSE_MIT.txt and LICENSE_BOOST_1_0.txt).

use alloc::{borrow::Cow, collections::BTreeMap};

use crate::{parse::Writer, seal::Seal};

/// WebAssembly primitive reader methods
pub trait Write<'a>: Seal {
    /// Encode the next ULEB128-encoded 32-bit unsigned integer.
    fn integer(&mut self, int: u32);

    /// Encode a WebAssembly "Name".
    fn name(&mut self, name: impl AsRef<str>);

    /// Encode a WebAssembly "Name Map".
    fn name_map(&mut self, name_map: BTreeMap<u32, Cow<'_, str>>);

    /// Encode a WebAssembly "Indirect Name Map".
    fn indirect_name_map(
        &mut self,
        indirect_name_map: BTreeMap<u32, BTreeMap<u32, Cow<'_, str>>>,
    );

    /// Encode a WebAssembly "Subsection"
    fn subsection(&mut self, subsection: u8, data: &[u8]);
}

impl<'a> Write<'a> for Writer<'a> {
    fn integer(&mut self, int: u32) {
        self.uleb128(int)
    }

    fn name(&mut self, name: impl AsRef<str>) {
        let name = name.as_ref();

        self.integer(name.len().try_into().unwrap_or(u32::MAX));
        self.str(name);
    }

    fn name_map(&mut self, name_map: BTreeMap<u32, Cow<'_, str>>) {
        self.integer(name_map.len().try_into().unwrap_or(u32::MAX));

        for (key, value) in name_map {
            self.integer(key);
            self.name(&value);
        }
    }

    fn indirect_name_map(
        &mut self,
        indirect_name_map: BTreeMap<u32, BTreeMap<u32, Cow<'_, str>>>,
    ) {
        self.integer(indirect_name_map.len().try_into().unwrap_or(u32::MAX));

        for (key, value) in indirect_name_map {
            self.integer(key);
            self.name_map(value);
        }
    }

    fn subsection(&mut self, subsection: u8, data: &[u8]) {
        self.u8(subsection);
        self.bytes(data);
    }
}
