// Copyright © 2022-2023 The Nucleide Contributors.
//
// Licensed under any of:
// - Apache License, Version 2.0 (https://www.apache.org/licenses/LICENSE-2.0)
// - Boost Software License, Version 1.0 (https://www.boost.org/LICENSE_1_0.txt)
// - MIT License (https://mit-license.org/)
// At your choosing (See accompanying files LICENSE_APACHE_2_0.txt,
// LICENSE_MIT.txt and LICENSE_BOOST_1_0.txt).

use alloc::{borrow::Cow, vec::Vec};

use crate::{daku::Portal, parse::Writer, Error};

/// Ordered set of requested portals.
#[derive(Debug, Default)]
pub struct Portals<'a>(pub(super) Cow<'a, [u8]>, u32);

impl Portals<'_> {
    /// Create an empty set of portals.
    pub fn new() -> Self {
        Self(Vec::new().into(), 0)
    }

    /// Append a portal to the ordered set
    ///
    /// Returns an error if the portals are not in numerical (ascending) order.
    pub fn append(mut self, portal: Portal) -> Result<Self, Error> {
        let mut data = self.0.into_owned();
        let mut writer = Writer::new(&mut data);
        let portal = portal.into();

        (portal > self.1)
            .then_some(())
            .ok_or(Error::with_msg("Wrong portal order"))?;

        writer.uleb128(portal);
        self.0 = data.into();
        self.1 = portal;

        Ok(self)
    }
}
