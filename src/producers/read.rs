// Copyright © 2022-2023 The Nucleide Contributors.
//
// Licensed under any of:
// - Apache License, Version 2.0 (https://www.apache.org/licenses/LICENSE-2.0)
// - Boost Software License, Version 1.0 (https://www.boost.org/LICENSE_1_0.txt)
// - MIT License (https://mit-license.org/)
// At your choosing (See accompanying files LICENSE_APACHE_2_0.txt,
// LICENSE_MIT.txt and LICENSE_BOOST_1_0.txt).

use alloc::vec::Vec;

use crate::{
    parse::Reader,
    producers::{Producer, ProducerKind, VersionedSoftware},
    seal::Seal,
    wasm::Read as _,
};

/// Producers section reader.
pub trait Read<'a>: Seal {
    /// Parse conventional WebAssembly producers custom section.
    fn producers(&mut self) -> Option<Vec<Producer<'a>>>;
}

impl<'a> Read<'a> for Reader<'a> {
    fn producers(&mut self) -> Option<Vec<Producer<'a>>> {
        (0..self.integer()?)
            .map(|_| {
                let kind = match self.name()? {
                    "language" => ProducerKind::Language,
                    "processed-by" => ProducerKind::ProcessedBy,
                    "sdk" => ProducerKind::Sdk,
                    _ => return None,
                };
                let software = (0..self.integer()?)
                    .map(|_| {
                        Some(VersionedSoftware {
                            name: self.name()?,
                            version: self.name()?,
                        })
                    })
                    .collect::<Option<_>>()?;

                Some(Producer {
                    kind,
                    list: software,
                })
            })
            .collect()
    }
}
