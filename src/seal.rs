// Copyright © 2022-2023 The Nucleide Contributors.
//
// Licensed under any of:
// - Apache License, Version 2.0 (https://www.apache.org/licenses/LICENSE-2.0)
// - Boost Software License, Version 1.0 (https://www.boost.org/LICENSE_1_0.txt)
// - MIT License (https://mit-license.org/)
// At your choosing (See accompanying files LICENSE_APACHE_2_0.txt,
// LICENSE_MIT.txt and LICENSE_BOOST_1_0.txt).

use crate::parse::{Reader, Writer};

// Sealed trait to prevent third-party implementations of some public traits,
// not exported publicly.
//
// This is not a doc comment so that `missing_docs` causes a compilation failure
// if exported on accident.
pub trait Seal {}

impl Seal for Reader<'_> {}
impl Seal for Writer<'_> {}
