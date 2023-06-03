// Copyright © 2022-2023 The Nucleide Contributors.
//
// Licensed under any of:
// - Apache License, Version 2.0 (https://www.apache.org/licenses/LICENSE-2.0)
// - Boost Software License, Version 1.0 (https://www.boost.org/LICENSE_1_0.txt)
// - MIT License (https://mit-license.org/)
// At your choosing (See accompanying files LICENSE_APACHE_2_0.txt,
// LICENSE_MIT.txt and LICENSE_BOOST_1_0.txt).

use alloc::vec::Vec;

use crate::{name::Name, parse::Reader, seal::Seal, wasm::Read as _};

/// Name section reader.
pub trait Read<'a>: Seal {
    /// Parse standard WebAssembly name custom section.
    fn names(&mut self) -> Option<Vec<Name<'a>>>;
}

impl<'a> Read<'a> for Reader<'a> {
    fn names(&mut self) -> Option<Vec<Name<'a>>> {
        let mut names = Vec::new();
        let mut subsection_min = 0;

        while self.end().is_none() {
            let (subsection, mut reader) = self.subsection()?;

            // Must be ordered correctly
            (subsection >= subsection_min).then_some(())?;
            names.push(match subsection {
                0 => Name::Module(reader.name()?),
                1 => Name::Function(reader.name_map()?),
                2 => Name::Local(reader.indirect_name_map()?),
                3 => Name::Label(reader.indirect_name_map()?),
                4 => Name::Type(reader.name_map()?),
                5 => Name::Table(reader.name_map()?),
                6 => Name::Memory(reader.name_map()?),
                7 => Name::Global(reader.name_map()?),
                8 => Name::Element(reader.name_map()?),
                9 => Name::Data(reader.name_map()?),
                _ => return None,
            });
            reader.end()?;
            subsection_min = subsection + 1;
        }

        Some(names)
    }
}
