// Copyright © 2022-2023 The Nucleide Contributors.
//
// Licensed under any of:
// - Apache License, Version 2.0 (https://www.apache.org/licenses/LICENSE-2.0)
// - Boost Software License, Version 1.0 (https://www.boost.org/LICENSE_1_0.txt)
// - MIT License (https://mit-license.org/)
// At your choosing (See accompanying files LICENSE_APACHE_2_0.txt,
// LICENSE_MIT.txt and LICENSE_BOOST_1_0.txt).

use alloc::{string::String, vec::Vec};

pub use parity_wasm::elements::CustomSection;
use parity_wasm::elements::{self, Serialize};

pub use crate::{Daku, Error, Section};

/// Represents WebAssembly module. Use new to build from buffer.
pub struct Module(elements::Module);

impl Module {
    /// Creates a Module from buffer.
    pub fn new(buf: &[u8]) -> Result<Self, Error> {
        Ok(Module(elements::Module::from_bytes(buf)?))
    }

    /// Returns an iterator over the module’s custom sections.
    pub fn custom_sections(&self) -> impl Iterator<Item = Section<'_>> {
        self.0.custom_sections().map(|section| Section {
            name: section.name().into(),
            data: section.payload().into(),
        })
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

    #[allow(unused)] // FIXME
    fn daku_section(&self) -> Option<Section<'_>> {
        self.custom_sections()
            .find(|section| section.name == "daku")
    }
}
