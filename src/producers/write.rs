// Copyright © 2022-2023 The Nucleide Contributors.
//
// Licensed under any of:
// - Apache License, Version 2.0 (https://www.apache.org/licenses/LICENSE-2.0)
// - Boost Software License, Version 1.0 (https://www.boost.org/LICENSE_1_0.txt)
// - MIT License (https://mit-license.org/)
// At your choosing (See accompanying files LICENSE_APACHE_2_0.txt,
// LICENSE_MIT.txt and LICENSE_BOOST_1_0.txt).

use crate::{
    parse::Writer,
    producers::{Producer, ProducerKind},
    seal::Seal,
    wasm::Write as _,
};

/// Producers section writer
pub trait Write<'a>: Seal {
    /// Write out conventional WebAssembly producers custom section.
    fn producers(&mut self, producers: &[Producer<'_>]);
}

impl<'a> Write<'a> for Writer<'a> {
    fn producers(&mut self, producers: &[Producer<'_>]) {
        self.integer(producers.len().try_into().unwrap_or(u32::MAX));

        for producer in producers {
            self.name(match producer.kind {
                ProducerKind::Language => "language",
                ProducerKind::ProcessedBy => "processed-by",
                ProducerKind::Sdk => "sdk",
            });

            self.integer(producer.list.len().try_into().unwrap_or(u32::MAX));

            for versioned_software in &producer.list {
                self.name(&versioned_software.name);
                self.name(&versioned_software.version);
            }
        }
    }
}
