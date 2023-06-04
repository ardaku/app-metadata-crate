// Copyright © 2022-2023 The Nucleide Contributors.
//
// Licensed under any of:
// - Apache License, Version 2.0 (https://www.apache.org/licenses/LICENSE-2.0)
// - Boost Software License, Version 1.0 (https://www.boost.org/LICENSE_1_0.txt)
// - MIT License (https://mit-license.org/)
// At your choosing (See accompanying files LICENSE_APACHE_2_0.txt,
// LICENSE_MIT.txt and LICENSE_BOOST_1_0.txt).

use alloc::{borrow::Cow, collections::BTreeMap};

/// Name subsection
#[derive(Debug)]
pub enum Name<'a> {
    /// Module Name
    Module(Cow<'a, str>),
    /// Function Names
    Function(BTreeMap<u32, Cow<'a, str>>),
    /// Local Names Per Function
    Local(BTreeMap<u32, BTreeMap<u32, Cow<'a, str>>>),
    /// Ext: Goto/Loop Label Names Per Function
    Label(BTreeMap<u32, BTreeMap<u32, Cow<'a, str>>>),
    /// Ext: Type Names
    Type(BTreeMap<u32, Cow<'a, str>>),
    /// Ext: Table Names
    Table(BTreeMap<u32, Cow<'a, str>>),
    /// Ext: Memory Names
    Memory(BTreeMap<u32, Cow<'a, str>>),
    /// Ext: Global Names
    Global(BTreeMap<u32, Cow<'a, str>>),
    /// Ext: Element Names
    Element(BTreeMap<u32, Cow<'a, str>>),
    /// Ext: Data Names
    Data(BTreeMap<u32, Cow<'a, str>>),
}
