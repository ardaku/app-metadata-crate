// Copyright © 2022-2023 The Nucleide Contributors.
//
// Licensed under any of:
// - Apache License, Version 2.0 (https://www.apache.org/licenses/LICENSE-2.0)
// - Boost Software License, Version 1.0 (https://www.boost.org/LICENSE_1_0.txt)
// - MIT License (https://mit-license.org/)
// At your choosing (See accompanying files LICENSE_APACHE_2_0.txt,
// LICENSE_MIT.txt and LICENSE_BOOST_1_0.txt).

use alloc::{
    borrow::{Cow, ToOwned},
    vec::Vec,
};

use crate::Section;

/// Daku custom section; an iterator over a list of portals.
pub struct Daku<'a>(pub(super) Cow<'a, [u8]>);

impl Daku<'static> {
    /// Create a new Daku section from a list of portals.
    pub fn new(portals: &[u32]) -> Self {
        let mut payload = Vec::new();

        for portal in portals {
            payload.extend(portal.to_le_bytes());
        }

        Self(payload.into())
    }
}

impl Iterator for Daku<'_> {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        let item = self.0.get(..4)?;
        let item = u32::from_le_bytes([item[0], item[1], item[2], item[3]]);

        match self.0 {
            Cow::Borrowed(ref mut slice) => *slice = &slice[4..],
            Cow::Owned(ref mut vec) => {
                let _ = vec.drain(..4);
            }
        }

        Some(item)
    }
}

impl<'a> From<Daku<'a>> for Section<'static> {
    fn from(daku: Daku<'a>) -> Self {
        Self {
            name: "daku".to_owned().into(),
            data: daku.0.into_owned().into(),
        }
    }
}
