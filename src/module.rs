// Copyright © 2022-2023 The Nucleide Contributors.
//
// Licensed under any of:
// - Apache License, Version 2.0 (https://www.apache.org/licenses/LICENSE-2.0)
// - Boost Software License, Version 1.0 (https://www.boost.org/LICENSE_1_0.txt)
// - MIT License (https://mit-license.org/)
// At your choosing (See accompanying files LICENSE_APACHE_2_0.txt,
// LICENSE_MIT.txt and LICENSE_BOOST_1_0.txt).

use alloc::vec::Vec;

use parity_wasm::elements::{self, CustomSection, Serialize};

use crate::{section::SectionKind, Error, Result, Section};

/// Represents WebAssembly module. Use new to build from buffer.
#[derive(Debug)]
pub struct Module(elements::Module);

impl Module {
    /// Creates a Module from buffer.
    pub fn new(buf: &[u8]) -> Result<Self> {
        Ok(Module(elements::Module::from_bytes(buf).map_err(Error)?))
    }

    /// Returns an iterator over the module’s custom sections.
    ///
    /// [`Section`]s are always yielded as the `Any` variant (borrowed).  They
    /// can be parsed with [`Section::to()`].
    pub fn sections(&self) -> Result<impl Iterator<Item = Section<'_>>> {
        const ERROR_MESSAGE: Error = Error::with_msg("Incorrect Section Order");

        let mut kind = SectionKind::Name;
        let iter = self.0.custom_sections();

        for section in self.0.custom_sections() {
            match section.name() {
                "name" if kind <= SectionKind::Name => {
                    kind = SectionKind::Producers
                }
                "producers" if kind <= SectionKind::Producers => {
                    kind = SectionKind::Daku
                }
                "daku" if kind <= SectionKind::Daku => {
                    kind = SectionKind::Unknown
                }
                "name" | "producers" | "daku" => return Err(ERROR_MESSAGE),
                _ => {}
            }
        }

        Ok(iter.map(|section| Section::Any {
            name: section.name().into(),
            data: section.payload().into(),
        }))
    }

    /// Sets the payload associated with the given custom section, or adds a new
    /// custom section, as appropriate.
    pub fn set_section(&mut self, mut section: Section<'_>) -> Option<()> {
        let (name, data) = section.to_any()?;

        self.0.set_custom_section(name, data.to_vec());

        Some(())
    }

    /// Removes the given custom section, if it exists. Returns the removed
    /// section if it existed, or None otherwise.
    pub fn clear_section(
        &mut self,
        name: impl AsRef<str>,
    ) -> Option<CustomSection> {
        self.0.clear_custom_section(name)
    }

    /// Write out module to a `Vec` of bytes.
    pub fn into_buffer(self) -> Result<Vec<u8>> {
        let mut v = Vec::new();
        self.0.serialize(&mut v).map_err(Error)?;
        Ok(v)
    }
}
