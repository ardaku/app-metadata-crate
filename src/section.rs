// Copyright © 2022-2023 The Nucleide Contributors.
//
// Licensed under any of:
// - Apache License, Version 2.0 (https://www.apache.org/licenses/LICENSE-2.0)
// - Boost Software License, Version 1.0 (https://www.boost.org/LICENSE_1_0.txt)
// - MIT License (https://mit-license.org/)
// At your choosing (See accompanying files LICENSE_APACHE_2_0.txt,
// LICENSE_MIT.txt and LICENSE_BOOST_1_0.txt).

use alloc::{borrow::Cow, string::String, vec::Vec};

use crate::{
    daku::{Daku, Read as _, Write as _},
    name::{Name, Read as _, Write as _},
    parse::{Reader, Writer},
    producers::{Producer, Read as _, Write as _},
};

/// Custom section
#[derive(Debug)]
pub enum Section<'a> {
    /// The `name` section
    Name(Vec<Name<'a>>),
    /// The `producers` section
    Producers(Vec<Producer<'a>>),
    /// The `daku` section
    Daku(Daku<'a>),
    /// Any section
    Any {
        /// The name of the custom section
        name: Cow<'a, str>,
        /// Data in the custom section
        data: Cow<'a, [u8]>,
    },
}

impl Section<'_> {
    /// Get the name of the section.
    pub fn name(&self) -> &str {
        use Section::*;

        match self {
            Name(_) => "name",
            Producers(_) => "producers",
            Daku(_) => "daku",
            Any { name, .. } => name,
        }
    }

    /// Convert section to `Any` variant, and return the `name` and `data`.
    pub fn to_any(&mut self) -> Option<(&str, &[u8])> {
        let (name, mut data) = (String::new(), Vec::new());
        let writer = &mut Writer::new(&mut data);

        match self {
            Self::Name(names) => {
                writer.names(names)?;

                *self = Self::Any {
                    name: name.into(),
                    data: data.into(),
                };
                self.to_any()
            }
            Self::Producers(producers) => {
                writer.producers(producers);

                *self = Self::Any {
                    name: name.into(),
                    data: data.into(),
                };
                self.to_any()
            }
            Self::Daku(daku) => {
                writer.daku(daku)?;

                *self = Self::Any {
                    name: name.into(),
                    data: data.into(),
                };
                self.to_any()
            }
            Self::Any { name, data } => Some((&name[..], &data[..])),
        }
    }

    /// Convert to non-Any variant if known, return `None` if can't.
    ///
    /// # Notes
    /// Returns none if owned rather than borrowed, or if not the `Any` variant.
    pub fn to(&self) -> Option<Self> {
        let Self::Any { name, data } = self else {
            return None;
        };
        let Cow::Borrowed(data) = data else {
            return None;
        };
        let mut reader = Reader::new(data);

        Some(match &name[..] {
            "name" => {
                let names = reader.names()?;

                reader.end()?;

                Self::Name(names)
            }
            "producers" => {
                let producers = reader.producers()?;

                reader.end()?;

                Self::Producers(producers)
            }
            "daku" => {
                let daku = reader.daku()?;

                reader.end()?;

                Self::Daku(daku)
            }
            _ => return None,
        })
    }
}
