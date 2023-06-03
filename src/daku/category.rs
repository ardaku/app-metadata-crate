// Copyright © 2022-2023 The Nucleide Contributors.
//
// Licensed under any of:
// - Apache License, Version 2.0 (https://www.apache.org/licenses/LICENSE-2.0)
// - Boost Software License, Version 1.0 (https://www.boost.org/LICENSE_1_0.txt)
// - MIT License (https://mit-license.org/)
// At your choosing (See accompanying files LICENSE_APACHE_2_0.txt,
// LICENSE_MIT.txt and LICENSE_BOOST_1_0.txt).

use num_enum::{IntoPrimitive as Into, TryFromPrimitive as TryFrom};

/// App category (Nucleide extension)
#[repr(u8)]
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
