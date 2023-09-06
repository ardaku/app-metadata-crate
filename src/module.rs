// Copyright © 2022-2023 The Nucleide Contributors.
//
// Licensed under any of:
// - Apache License, Version 2.0 (https://www.apache.org/licenses/LICENSE-2.0)
// - Boost Software License, Version 1.0 (https://www.boost.org/LICENSE_1_0.txt)
// - MIT License (https://mit-license.org/)
// At your choosing (See accompanying files LICENSE_APACHE_2_0.txt,
// LICENSE_MIT.txt and LICENSE_BOOST_1_0.txt).

use alloc::{string::String, vec::Vec};
use core::mem;

use parity_wasm::elements::{self, Serialize};
use num_enum::{IntoPrimitive as Into, TryFromPrimitive as TryFrom};

use crate::{section::SectionKind, Error, Result, Section, parse::Reader, wasm::Read as _};

/// A WebAssembly module (or part of one).
///
/// Use [`Module::new`] to build from a buffer.
#[derive(Debug)]
pub struct Module<'a> {
    module: elements::Module,
    buf: &'a [u8],
}

impl<'a> Module<'a> {
    /// Creates a Module from buffer.
    pub fn new(buf: &'a [u8]) -> Result<Self> {
        const MAGIC: &[u8; 4] = b"\0asm";
        const VERSION: u32 = 1;

        const EOF: Error = Error::with_msg("End of file");
        const MAGIC_ERR: Error = Error::with_msg("Unexpected magic bytes");
        const VERSION_ERR: Error = Error::with_msg("Unknown WASM version");

        let mut reader = Reader::new(buf);
        let magic = reader.bytes(4).ok_or(EOF)?;
        let version = reader.u32().ok_or(EOF)?;

        (magic == MAGIC).then_some(()).ok_or(MAGIC_ERR)?;
        (version == VERSION).then_some(()).ok_or(VERSION_ERR)?;

        // FIXME: Testing
        #[derive(Into, TryFrom, Debug)]
        #[repr(u8)]
        enum SectionId {
            Custom = 0,
            Type = 1,
            Import = 2,
            Function = 3,
            Table = 4,
            Memory = 5,
            Global = 6,
            Export = 7,
            Start = 8,
            Element = 9,
            Code = 10,
            Data = 11,
            DataCount = 12,
        }
        let mut dbg = String::new();
        while reader.end().is_none() {
            let id = reader.u8().ok_or(EOF)?;
            let id = SectionId::try_from(id).map_err(|_| Error::with_msg("Invalid section ID: {id}"))?;
            let size = reader.integer().ok_or(EOF)?;
            let size = size.try_into().map_err(|_| Error::with_msg("Not enough memory"))?;
            let _reader = reader.reader(size).ok_or(EOF)?;

            dbg.push_str(&alloc::format!("\n{id:?}; {size} bytes"));
        }
        panic!("{dbg}");

        Ok(Self {
            module: elements::Module::from_bytes(buf).map_err(Error)?,
            buf,
        })
    }

    /// Returns an iterator over the module’s custom sections.
    ///
    /// [`Section`]s are always yielded as the `Any` variant (borrowed).  They
    /// can be parsed with [`Section::to()`].
    pub fn sections(&self) -> Result<impl Iterator<Item = Section<'_>>> {
        const ERROR_MESSAGE: Error = Error::with_msg("Incorrect Section Order");

        let mut kind = SectionKind::Name;
        let iter = self.module.custom_sections();

        for section in self.module.custom_sections() {
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

        self.module.set_custom_section(name, data.to_vec());

        Some(())
    }

    /// Removes the given custom section, if it exists. Returns the removed
    /// section if it existed, or None otherwise.
    pub fn clear_section(
        &mut self,
        name: impl AsRef<str>,
    ) -> Option<Section<'static>> {
        let mut section = self.module.clear_custom_section(&name)?;
        let (mut name, mut data) = (String::new(), Vec::new());

        mem::swap(&mut name, section.name_mut());
        mem::swap(&mut data, section.payload_mut());

        Some(Section::Any {
            name: name.into(),
            data: data.into(),
        })
    }

    /// Write out module to a `Vec` of bytes.
    pub fn into_buffer(self) -> Result<Vec<u8>> {
        let mut v = Vec::new();

        self.module.serialize(&mut v).map_err(Error)?;

        Ok(v)
    }
}
