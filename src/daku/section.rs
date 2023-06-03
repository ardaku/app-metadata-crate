// Copyright © 2022-2023 The Nucleide Contributors.
//
// Licensed under any of:
// - Apache License, Version 2.0 (https://www.apache.org/licenses/LICENSE-2.0)
// - Boost Software License, Version 1.0 (https://www.boost.org/LICENSE_1_0.txt)
// - MIT License (https://mit-license.org/)
// At your choosing (See accompanying files LICENSE_APACHE_2_0.txt,
// LICENSE_MIT.txt and LICENSE_BOOST_1_0.txt).

use alloc::vec::Vec;

use crate::daku::{Nucleide, Portal};

/// Daku section
#[derive(Debug)]
pub struct Daku<'a> {
    /// List of portals required by the application.
    pub portals: Vec<Portal>,
    /// List of nucleide metadata extension subsections for Nucleic desktop.
    pub nucleide: Vec<Nucleide<'a>>,
}
