// Copyright © 2022-2023 The Nucleide Contributors.
//
// Licensed under any of:
// - Apache License, Version 2.0 (https://www.apache.org/licenses/LICENSE-2.0)
// - Boost Software License, Version 1.0 (https://www.boost.org/LICENSE_1_0.txt)
// - MIT License (https://mit-license.org/)
// At your choosing (See accompanying files LICENSE_APACHE_2_0.txt,
// LICENSE_MIT.txt and LICENSE_BOOST_1_0.txt).
//
//! Custom section for [daku].
//!
//! [daku]: https://ardaku.org/daku/

mod category;
mod file;
mod nucleide;
mod portal;
mod read;
mod section;
mod write;

pub use self::{
    category::Category, file::File, nucleide::Nucleide, portal::Portal,
    read::Read, section::Daku, write::Write,
};
