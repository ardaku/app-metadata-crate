// Copyright © 2022-2023 The Nucleide Contributors.
//
// Licensed under any of:
// - Apache License, Version 2.0 (https://www.apache.org/licenses/LICENSE-2.0)
// - Boost Software License, Version 1.0 (https://www.boost.org/LICENSE_1_0.txt)
// - MIT License (https://mit-license.org/)
// At your choosing (See accompanying files LICENSE_APACHE_2_0.txt,
// LICENSE_MIT.txt and LICENSE_BOOST_1_0.txt).

use core::fmt;

use parity_wasm::elements;

/// Result type alias
pub type Result<T = (), E = Error> = core::result::Result<T, E>;

/// Deserialization/serialization error
#[derive(Debug)]
pub struct Error(pub(crate) elements::Error);

impl Error {
    pub(crate) const fn with_msg(message: &'static str) -> Self {
        Self(elements::Error::Other(message))
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        <elements::Error as fmt::Display>::fmt(&self.0, f)
    }
}
