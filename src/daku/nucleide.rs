// Copyright © 2022-2023 The Nucleide Contributors.
//
// Licensed under any of:
// - Apache License, Version 2.0 (https://www.apache.org/licenses/LICENSE-2.0)
// - Boost Software License, Version 1.0 (https://www.boost.org/LICENSE_1_0.txt)
// - MIT License (https://mit-license.org/)
// At your choosing (See accompanying files LICENSE_APACHE_2_0.txt,
// LICENSE_MIT.txt and LICENSE_BOOST_1_0.txt).

use alloc::{borrow::Cow, collections::BTreeMap, vec::Vec};

use num_enum::{IntoPrimitive as Into, TryFromPrimitive as TryFrom};

/// App category (Nucleide extension)
#[repr(u32)]
#[derive(Debug, Copy, Clone, Into, TryFrom)]
pub enum Category {
    /// Applications for playing / recording / editing audio, video, drawing,
    /// photos, fonts, 3D-modeling
    Media = 0x00,
    /// Applications for viewing / editing / translating documents and
    /// spreadsheets
    Office = 0x01,
    /// Applications for inspecting the operating system, tweaking, installing,
    /// and virtualization
    System = 0x02,
    /// Applications for software development, math, related tools
    Coding = 0x03,
    /// Applications for browsing the web, peer-to-peer file sharing, email,
    /// social media, etc.
    Internet = 0x04,
    /// Applications for playing video games
    Gaming = 0x05,
    /// Applications for simulations, electrical/mechanical engineering, A/I
    /// for inspecting data, robots
    Science = 0x06,
    /// Applications for education, learning
    Education = 0x07,
    /// Applications to-do lists, calendar, wellbeing, fitness, directions,
    /// mapping, weather, smart home, etc.
    Life = 0x08,
    /// Applications for coupons, buying/selling, trading, currency
    Finance = 0x09,
}

/// Metadata file (Nucleide extension)
#[derive(Debug)]
pub struct File<'a> {
    /// The path of the file
    pub path: Cow<'a, str>,
    /// Data in the file
    pub data: Cow<'a, [u8]>,
}

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
    LocalizedAssets(BTreeMap<u32, File<'a>>),
    /// English lowercase words separated by spaces (no punctuation allowed)
    Tags(Vec<Cow<'a, str>>),
    /// App category for the Emporium (limit 2)
    Categories(Vec<Category>),
    /// Name of organization/company/developer of application
    Developer(Cow<'a, str>),
}
