// Copyright © 2022-2023 The Nucleide Contributors.
//
// Licensed under any of:
// - Apache License, Version 2.0 (https://www.apache.org/licenses/LICENSE-2.0)
// - Boost Software License, Version 1.0 (https://www.boost.org/LICENSE_1_0.txt)
// - MIT License (https://mit-license.org/)
// At your choosing (See accompanying files LICENSE_APACHE_2_0.txt,
// LICENSE_MIT.txt and LICENSE_BOOST_1_0.txt).

use alloc::{borrow::Cow, vec::Vec};

/// Versioned software name
#[derive(Clone, Eq, PartialEq, Debug)]
pub struct VersionedSoftware<'a> {
    /// Name of the program/application/tool
    pub name: Cow<'a, str>,
    /// Version of the program/application/tool
    pub version: Cow<'a, str>,
}

/// Kind of producer
#[derive(Clone, Copy, Eq, PartialEq, Debug)]
pub enum ProducerKind {
    /// Source language list
    Language,
    /// Individual tool list
    ProcessedBy,
    /// SDK list
    Sdk,
}

/// Producer Field
#[derive(Clone, Eq, PartialEq, Debug)]
pub struct Producer<'a> {
    /// Kind of the list
    pub kind: ProducerKind,
    /// List of versioned names
    pub list: Vec<VersionedSoftware<'a>>,
}
