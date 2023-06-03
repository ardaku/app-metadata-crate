// Copyright © 2022-2023 The Nucleide Contributors.
//
// Licensed under any of:
// - Apache License, Version 2.0 (https://www.apache.org/licenses/LICENSE-2.0)
// - Boost Software License, Version 1.0 (https://www.boost.org/LICENSE_1_0.txt)
// - MIT License (https://mit-license.org/)
// At your choosing (See accompanying files LICENSE_APACHE_2_0.txt,
// LICENSE_MIT.txt and LICENSE_BOOST_1_0.txt).

use alloc::{borrow::Cow, collections::BTreeMap, vec::Vec};

use crate::daku::{Category, File};

/// Nucleide subsection extension for Daku (for use with Nucleic desktop)
#[derive(Debug)]
pub enum Nucleide<'a> {
    /// Localized names for the Nucleic desktop application
    LocalizedNames(BTreeMap<u32, Cow<'a, str>>),
    /// Localized descriptions for the app Emporium
    LocalizedDescriptions(BTreeMap<u32, Cow<'a, str>>),
    /// Icons for each theme (standard ones are "default" and "reduced")
    ThemedIcons(Vec<File<'a>>),
    /// Localized assets for the app Emporium
    LocalizedAssets(BTreeMap<u32, Vec<File<'a>>>),
    /// English lowercase words separated by spaces (no punctuation allowed)
    Tags(Vec<Cow<'a, str>>),
    /// App category for the Emporium (limit 2)
    Categories(Vec<Category>),
    /// Name of organization/company/developer of application
    Developer(Cow<'a, str>),
}
