// Copyright © 2022-2023 The Nucleide Contributors.
//
// Licensed under any of:
// - Apache License, Version 2.0 (https://www.apache.org/licenses/LICENSE-2.0)
// - Boost Software License, Version 1.0 (https://www.boost.org/LICENSE_1_0.txt)
// - MIT License (https://mit-license.org/)
// At your choosing (See accompanying files LICENSE_APACHE_2_0.txt,
// LICENSE_MIT.txt and LICENSE_BOOST_1_0.txt).

use crate::{name::Name, parse::Writer, seal::Seal, wasm::Write as _};

/// Name section writer
pub trait Write<'a>: Seal {
    /// Write out standard WebAssembly name custom section.
    fn names(&mut self, names: &[Name<'_>]) -> Option<()>;
}

impl<'a> Write<'a> for Writer<'a> {
    fn names(&mut self, names: &[Name<'_>]) -> Option<()> {
        let mut subsection_min = 0;

        for name in names {
            let subsection = match name {
                Name::Module(_) => 0,
                Name::Function(_) => 1,
                Name::Local(_) => 2,
                Name::Label(_) => 3,
                Name::Type(_) => 4,
                Name::Table(_) => 5,
                Name::Memory(_) => 6,
                Name::Global(_) => 7,
                Name::Element(_) => 8,
                Name::Data(_) => 9,
            };

            // Must be ordered correctly
            (subsection >= subsection_min).then_some(())?;
            subsection_min = subsection + 1;
            self.u8(subsection);

            match name {
                Name::Module(data) => self.name(data),
                Name::Function(data) => self.name_map(data),
                Name::Local(data) => self.indirect_name_map(data),
                Name::Label(data) => self.indirect_name_map(data),
                Name::Type(data) => self.name_map(data),
                Name::Table(data) => self.name_map(data),
                Name::Memory(data) => self.name_map(data),
                Name::Global(data) => self.name_map(data),
                Name::Element(data) => self.name_map(data),
                Name::Data(data) => self.name_map(data),
            };
        }

        Some(())
    }
}
