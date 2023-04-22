// Copyright © 2022-2023 The Nucleide Contributors.
//
// Licensed under any of:
// - Apache License, Version 2.0 (https://www.apache.org/licenses/LICENSE-2.0)
// - Boost Software License, Version 1.0 (https://www.boost.org/LICENSE_1_0.txt)
// - MIT License (https://mit-license.org/)
// At your choosing (See accompanying files LICENSE_APACHE_2_0.txt,
// LICENSE_MIT.txt and LICENSE_BOOST_1_0.txt).

#![doc = include_str!("../README.md")]
#![no_std]

extern crate alloc;

mod daku;
mod error;

use alloc::{borrow::Cow, string::String, vec::Vec};

pub use parity_wasm::elements::CustomSection;
use parity_wasm::{elements::Serialize, *};

pub use self::{daku::Daku, error::Error};

/// Represents WebAssembly module. Use new to build from buffer.
pub struct Module(elements::Module);

impl Module {
    /// Creates a Module from buffer.
    pub fn new(buf: &[u8]) -> Result<Self, Error> {
        Ok(Module(elements::Module::from_bytes(buf)?))
    }

    /// Returns an iterator over the module’s custom sections.
    pub fn custom_sections(
        &self,
    ) -> impl Iterator<Item = &elements::CustomSection> {
        self.0.custom_sections()
    }

    /// Returns the Daku custom section.
    pub fn daku(&self) -> Option<Daku<'_>> {
        let section = self
            .custom_sections()
            .find(|section| section.name() == "daku")?;

        let payload = section.payload();
        let count = payload.get(..4)?;
        let bytes = 4 * usize::try_from(u32::from_le_bytes([
            count[0], count[1], count[2], count[3],
        ]))
        .ok()?;

        Some(Daku(Cow::Borrowed(payload.split_at(4).1.get(..bytes)?)))
    }

    /// Sets the payload associated with the given custom section, or adds a new
    /// custom section, as appropriate.
    pub fn add_custom_section(
        &mut self,
        name: impl Into<String>,
        payload: Vec<u8>,
    ) {
        self.0.set_custom_section(name, payload)
    }

    /// Removes the given custom section, if it exists. Returns the removed
    /// section if it existed, or None otherwise.
    pub fn clear_custom_section(
        &mut self,
        name: impl AsRef<str>,
    ) -> Option<CustomSection> {
        self.0.clear_custom_section(name)
    }

    pub fn into_buffer(self) -> Result<Vec<u8>, Error> {
        let mut v = Vec::new();
        self.0.serialize(&mut v)?;
        Ok(v)
    }
}

/// Custom section
pub struct Section<'a> {
    /// The name of the custom section
    pub name: Cow<'a, str>,
    /// Data in the custom section
    pub data: Cow<'a, [u8]>,
}
