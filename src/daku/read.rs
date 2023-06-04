// Copyright © 2022-2023 The Nucleide Contributors.
//
// Licensed under any of:
// - Apache License, Version 2.0 (https://www.apache.org/licenses/LICENSE-2.0)
// - Boost Software License, Version 1.0 (https://www.boost.org/LICENSE_1_0.txt)
// - MIT License (https://mit-license.org/)
// At your choosing (See accompanying files LICENSE_APACHE_2_0.txt,
// LICENSE_MIT.txt and LICENSE_BOOST_1_0.txt).

use alloc::{collections::BTreeMap, vec::Vec};

use crate::{
    daku::{Category, File, Nucleide, Portal},
    parse::Reader,
    seal::Seal,
    wasm::Read as _,
};

/// Daku section reader.
pub trait Read<'a>: Seal {
    /// Parse portals list from Daku section.
    fn portals(&mut self) -> Option<Vec<Portal>>;

    /// Parse nucleide extensions subsection.
    fn nucleide(&mut self) -> Option<Vec<Nucleide<'a>>>;

    /// Parse file (Nucleide extension).
    fn file(&mut self) -> Option<File<'a>>;

    /// Parse vector of files (Nucleide extension)
    fn file_vector(&mut self) -> Option<Vec<File<'a>>>;

    /// Parse map of files (Nucleide extension)
    fn file_map(&mut self) -> Option<BTreeMap<u32, File<'a>>>;

    /// Parse category (Nucleide extension).
    fn category(&mut self) -> Option<Category>;

    /// Parse category vector (Nucleide extension).
    fn category_vector(&mut self) -> Option<Vec<Category>>;
}

impl<'a> Read<'a> for Reader<'a> {
    fn portals(&mut self) -> Option<Vec<Portal>> {
        let size = self.integer()?.try_into().ok()?;

        (0..size).map(|_| self.integer()?.try_into().ok()).collect()
    }

    fn nucleide(&mut self) -> Option<Vec<Nucleide<'a>>> {
        let mut subsections = Vec::new();
        let mut subsection_min = 0;

        while self.end().is_none() {
            let (subsection, mut reader) = self.subsection()?;

            // Must be ordered correctly
            (subsection >= subsection_min).then_some(())?;
            subsections.push(match subsection {
                0 => Nucleide::LocalizedNames(reader.name_map()?),
                1 => Nucleide::LocalizedDescriptions(reader.name_map()?),
                2 => Nucleide::ThemedIcons(reader.file_vector()?),
                3 => Nucleide::LocalizedAssets(reader.file_map()?),
                4 => Nucleide::Tags(reader.name_vector()?),
                5 => Nucleide::Categories(reader.category_vector()?),
                6 => Nucleide::Developer(reader.name()?),
                _ => return None,
            });
            reader.end()?;
            subsection_min = subsection + 1;
        }

        Some(subsections)
    }

    fn file(&mut self) -> Option<File<'a>> {
        Some(File {
            path: self.name()?,
            data: {
                let len = self.integer()?.try_into().ok()?;

                self.bytes(len)?.into()
            },
        })
    }

    fn file_vector(&mut self) -> Option<Vec<File<'a>>> {
        (0..self.integer()?).map(|_| self.file()).collect()
    }

    fn file_map(&mut self) -> Option<BTreeMap<u32, File<'a>>> {
        let mut file_map = BTreeMap::new();

        for _ in 0..self.integer()? {
            file_map.insert(self.integer()?, self.file()?);
        }

        Some(file_map)
    }

    fn category(&mut self) -> Option<Category> {
        self.integer()?.try_into().ok()
    }

    fn category_vector(&mut self) -> Option<Vec<Category>> {
        (0..self.integer()?).map(|_| self.category()).collect()
    }
}
