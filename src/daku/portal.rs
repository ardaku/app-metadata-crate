// Copyright © 2022-2023 The Nucleide Contributors.
//
// Licensed under any of:
// - Apache License, Version 2.0 (https://www.apache.org/licenses/LICENSE-2.0)
// - Boost Software License, Version 1.0 (https://www.boost.org/LICENSE_1_0.txt)
// - MIT License (https://mit-license.org/)
// At your choosing (See accompanying files LICENSE_APACHE_2_0.txt,
// LICENSE_MIT.txt and LICENSE_BOOST_1_0.txt).

use num_enum::{IntoPrimitive as Into, TryFromPrimitive as TryFrom};

/// A portal
#[repr(u32)]
#[derive(Debug, Copy, Clone, Into, TryFrom)]
pub enum Portal {
    /// Logging API (stdout/printf)
    Log = 0x00,
    /// Developer command API (stdin/scanf)
    Prompt = 0x01,
    ///
    Fetch = 0x02,
    ///
    Serve = 0x03,
    ///
    Speakers = 0x04,
    ///
    Microphone = 0x05,
    ///
    Screen = 0x06,
    ///
    Camera = 0x07,
    ///
    Window = 0x08,
    ///
    Spawn = 0x09,
    /// Set user information API (username, display name, localization)
    User = 0x0A,
    /// Get user information API (username, display name, localization)
    Preferences = 0x0B,
    /// Create new users, settings for all users
    System = 0x0C,
    /// Get system information and settings
    About = 0x0D,
    ///
    File = 0x0E,
    ///
    Hid = 0x0F,
    ///
    Timer = 0x10,
    ///
    Clock = 0x11,
    ///
    Gpu = 0x12,
    ///
    Location = 0x13,
}
