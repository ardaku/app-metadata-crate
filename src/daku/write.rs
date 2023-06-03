// Copyright © 2022-2023 The Nucleide Contributors.
//
// Licensed under any of:
// - Apache License, Version 2.0 (https://www.apache.org/licenses/LICENSE-2.0)
// - Boost Software License, Version 1.0 (https://www.boost.org/LICENSE_1_0.txt)
// - MIT License (https://mit-license.org/)
// At your choosing (See accompanying files LICENSE_APACHE_2_0.txt,
// LICENSE_MIT.txt and LICENSE_BOOST_1_0.txt).

use crate::{daku::Portal, parse::Writer, seal::Seal, wasm::Write as _};

/// Daku section  writer.
pub trait Write<'a>: Seal {
    /// Write out portals list from Daku section.
    fn portals(&mut self, portals: &[Portal]);
}

impl<'a> Write<'a> for Writer<'a> {
    fn portals(&mut self, portals: &[Portal]) {
        // Write vector length
        self.integer(portals.len().try_into().unwrap_or(u32::MAX));

        for portal in portals.iter().copied() {
            self.integer(portal.into());
        }
    }
}
