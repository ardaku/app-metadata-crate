// Copyright © 2022-2023 The Nucleide Contributors.
//
// Licensed under any of:
// - Apache License, Version 2.0 (https://www.apache.org/licenses/LICENSE-2.0)
// - Boost Software License, Version 1.0 (https://www.boost.org/LICENSE_1_0.txt)
// - MIT License (https://mit-license.org/)
// At your choosing (See accompanying files LICENSE_APACHE_2_0.txt,
// LICENSE_MIT.txt and LICENSE_BOOST_1_0.txt).

use alloc::collections::BTreeMap;

use crate::{
    daku::{Category, Daku, File, Nucleide, Portal},
    parse::Writer,
    seal::Seal,
    wasm::Write as _,
};

/// Daku section  writer.
pub trait Write<'a>: Seal {
    /// Write out daku section.
    fn daku(&mut self, daku: &Daku<'_>) -> Option<()>;

    /// Write out portals list from Daku section.
    fn portals(&mut self, portals: &[Portal]);

    /// Write out nucleide extension subsections.
    fn nucleide(&mut self, subsections: &[Nucleide<'_>]) -> Option<()>;

    /// Write out file (Nucleide extension).
    fn file(&mut self, file: &File<'_>);

    /// Write out file vector (Nucleide extension).
    fn file_vector(&mut self, files: &[File<'_>]);

    /// Write out map of files (Nucleide extension)
    fn file_map(&mut self, files: &BTreeMap<u32, File<'_>>);

    /// Write out category (Nucleide extension).
    fn category(&mut self, category: Category);

    /// Write out category vector (Nucleide extension).
    fn category_vector(&mut self, categories: &[Category]);
}

impl<'a> Write<'a> for Writer<'a> {
    fn daku(&mut self, daku: &Daku<'_>) -> Option<()> {
        self.portals(daku.portals.as_slice());

        if let Some(ref nucleide) = daku.nucleide {
            self.nucleide(nucleide.as_slice())?;
        }

        Some(())
    }

    fn portals(&mut self, portals: &[Portal]) {
        // Write vector length
        self.integer(portals.len().try_into().unwrap_or(u32::MAX));

        for portal in portals.iter().copied() {
            self.integer(portal.into());
        }
    }

    fn nucleide(&mut self, subsections: &[Nucleide<'_>]) -> Option<()> {
        let mut subsection_min = 0;

        for subsection in subsections {
            let id = match subsection {
                Nucleide::LocalizedNames(_) => 0,
                Nucleide::LocalizedDescriptions(_) => 1,
                Nucleide::ThemedIcons(_) => 2,
                Nucleide::LocalizedAssets(_) => 3,
                Nucleide::Tags(_) => 4,
                Nucleide::Categories(_) => 5,
                Nucleide::Developer(_) => 6,
            };

            // Must be ordered correctly
            (id >= subsection_min).then_some(())?;
            subsection_min = id + 1;
            self.u8(id);

            match subsection {
                Nucleide::LocalizedNames(data) => self.name_map(data),
                Nucleide::LocalizedDescriptions(data) => self.name_map(data),
                Nucleide::ThemedIcons(data) => self.file_vector(data),
                Nucleide::LocalizedAssets(data) => self.file_map(data),
                Nucleide::Tags(data) => self.name_vector(data),
                Nucleide::Categories(data) => self.category_vector(data),
                Nucleide::Developer(data) => self.name(data),
            };
        }

        Some(())
    }

    fn file(&mut self, file: &File<'_>) {
        self.name(&file.path);
        self.integer(file.data.len().try_into().unwrap_or(u32::MAX));
        self.bytes(&file.data);
    }

    fn file_vector(&mut self, files: &[File<'_>]) {
        // Write vector length
        self.integer(files.len().try_into().unwrap_or(u32::MAX));

        files.iter().for_each(|file| self.file(file));
    }

    fn file_map(&mut self, files: &BTreeMap<u32, File<'_>>) {
        // Write map length
        self.integer(files.len().try_into().unwrap_or(u32::MAX));

        files.iter().for_each(|(k, v)| {
            self.integer(*k);
            self.file(v);
        });
    }

    fn category(&mut self, category: Category) {
        self.integer(category.into());
    }

    fn category_vector(&mut self, categories: &[Category]) {
        // Write vector length
        self.integer(categories.len().try_into().unwrap_or(u32::MAX));

        categories
            .iter()
            .for_each(|category| self.category(*category));
    }
}
