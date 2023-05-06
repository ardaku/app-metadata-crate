# Nucleide
A crate to manipulate [custom sections] of a WebAssembly module to view/edit
application metadata.

# Specification
Nucleide specifies WASM metadata that is read by the Nucleic Desktop
Environment.  It includes the WebAssembly 2.0 and the Daku 1.0.0-beta.0
specifications.

Daku programs are a WebAssembly module that must have the `daku` custom section,
are compressed with ZStd, and should use the `.daku` file extension; thus the
Nucleide specification, as an extension of Daku, shall follow.

App data that can be displayed by a software manager, and where it comes from:
 - Non-Localized App Name: `name` section => Module Name Subsection
 - Programming Language: `producers` section => Language Field
 - Processed With: `producers` section => Processed-By Field
 - Generated With: `producers` section => SDK Field
 - Required Permissions: `daku` section => Portals Header
 - Localized App Names: `daku` section => Translations Subsection
 - App Description: `daku` section => Description Translations Subsection
 - App Icon Themes: `daku` section => App Icon Themes Subsection
 - App Screenshots: `daku` section => Description Assets Subsection
 - Searchable Tags: `daku` section => Tags Subsection
 - Categories: `daku` section => Categories Subsection
 - Organization: `daku` section => Organization Name Subsection

## Types
Nucleide custom sections reuse WebAssembly types:

#### `Byte`
Simply an 8-bit integer.

#### `Integer`
A [Unsigned LEB128] variable-length encoded litte-endian integer, with a maximum
value of 2³²-1 (can be anywhere from 1-5 bytes).

#### `Vector[T]`
A sequence of the following:

 - `size: Integer`
 - `data: [T; size]`

#### `Name`
Containing valid UTF-8 (no null termination); wrapper around:

 - `Vector[Byte]`

#### `NameMap`
A `Vector`, with each element containing a sequence of the following:

 - `index: Integer` - Must be sorted in sequence
 - `name: Name` 

#### `IndirectNameMap`
A `Vector`, with each element containing a sequence of the following:

 - `index: Integer` - Must be sorted in sequence
 - `name_map: NameMap`

## Custom Sections

### Name (`name`)
From [the wasm spec], debug info.  It is expected that apps are built with this
module generated for easier debugging, but stripped away and put into a separate
`.name` file for distribution.

 - `subsection: u8`: Each subsection is optional, and must be placed in this
   order:
   - 0 => Module Name
   - 1 => Function Names
   - 2 => Local Names
   - 3 => Ext: Label Names
   - 4 => Ext: Type Names
   - 5 => Ext: Table Names
   - 6 => Ext: Memory Names
   - 7 => Ext: Global Names
   - 8 => Ext: Element Names
   - 9 => Ext: Data Names
 - `size: u32`: Number of bytes

#### 0 => Module Name
 - `name: Name`: Name of the app

#### 1 => Function Names
 - `name_map: NameMap`: Names of each function

#### 2 => Local Names
 - `indirect_name_map: IndirectNameMap`: Names of each variable in each function

#### 3 => Ext: Label Names
 - `indirect_name_map: IndirectNameMap`: Names of each label in each function

#### 4 => Ext: Type Names
 - `name_map: NameMap`: Names of each type

#### 5 => Ext: Table Names
 - `name_map: NameMap`: Names of each table

#### 6 => Ext: Memory Names
 - `name_map: NameMap`: Names of each memory

#### 7 => Ext: Global Names
 - `name_map: NameMap`: Names of each global

#### 8 => Ext: Element Names
 - `name_map: NameMap`: Names of each element

#### 9 => Ext: Data Names
 - `name_map: NameMap`: Names of each data

### Producers (`producers`)
From WebAssembly's [tool conventions], information on how the `.daku`
WebAssembly file was generated.

A `Vector`, with each element containing a sequence of the following:

 - `name: Name` - One of:
   - `"language"`
   - `"processed-by"`
   - `"sdk"`
 - `tool_version_pairs: Vector<(String, String)>`

### Daku (`daku`)

 - `portals: Vector<Integer>`: List of Portal IDs

Following the Daku portals list, is the nucleide extension:

 - `subsection: u8`: Each subsection is optional, and must be placed in this
   order:
   - 0 => Reserved for potential breaking 2.0 version of Daku
   - 1 => App Name Translations
   - 2 => App Description Translations
   - 3 => App Icon Themes
   - 4 => App Description Assets
   - 5 => Searchable Tags
   - 6 => Searchable Categories
   - 7 => Organization Name
 - `size: u32`: Number of bytes

#### 1 => App Name Translations
 - `localized_names: NameMap`

Integer representation of a 4-letter (2-letter lowercase language, 2-letter
uppercase region) locale ASCII description:

 - `locale: b"enUS"`
   ```C
   locale[0] | locale[1] << 7 | locale[2] << 14 | locale[3] << 21
   ```

#### 2 => App Description Translations

 - `localized_mdfiles: NameMap`: Markdown file for each description

Integer representation of a 4-letter (2-letter lowercase language, 2-letter
uppercase region) locale ASCII description:

 - `locale: b"enUS"`
   ```C
   locale[0] | locale[1] << 7 | locale[2] << 14 | locale[3] << 21
   ```

#### 3 => App Icon Themes
A `Vector`, with each element containing a sequence of the following:

 - `name: Name`: Theme name, `"default"` or `"reduced"`; reduced theme should be
   binary (on/off) RGBA.  default is full 0-255 range for each.
 - `data: Vector<u8>`: Concatenated list of [QOI]  (future: or [RVG]) files.
   Best resolution out of the files will be chosen.  None can have the same
   resolution.

#### 4 => App Description Assets
A `Vector`, with each element containing a sequence of the following:

 - `locale: Integer`: Set to 0 for non-localized assets.
 - `path: Name`: Markdown path
 - `data: Vector<u8>`: [QOI]  (future: or [RVG]) file.

#### 5 => Searchable Tags
A `Vector` (limit 8), with each element containing:

 - `tag: Name`: Name of the tag (all lowercase ASCII english words separated by
   spaces; no `-` or `_`, other punctuation)

#### 6 => Searchable Categories
A `Vector` (limit 2), with each element containing:

 - `tag: Name`: Name of the category, one of:
   - `media` - Applications for playing / recording / editing audio, video,
     drawing, photos, fonts, 3D-modeling
   - `office` - Applications for viewing / editing / translating documents and
     spreadsheets
   - `system` - Applications for inspecting the operating system, tweaking,
     installing, and virtualization
   - `coding` - Applications for software development, math, related tools
   - `internet` - Applications for browsing the web, peer-to-peer file sharing,
     email, social media, etc.
   - `gaming` - Applications for playing video games
   - `science` - Applications for simulations, electrical/mechanical
     engineering, A/I for inspecting data, robots
   - `education` - Applications for education, learning
   - `life` - Applications to-do lists, calendar, wellbeing, fitness,
     directions, mapping, weather, smart home, etc.
   - `finance` - Applications for coupons, buying/selling, trading, currency

#### 7 => Organization Name

 - `organization: Name`: Name of organization that developed the software

[custom sections]: https://webassembly.github.io/spec/core/binary/modules.html#index-2
[the wasm spec]: https://webassembly.github.io/spec/core/appendix/custom.html#name-section
[QOI]: https://qoiformat.org/qoi-specification.pdf
[RVG]: https://github.com/ardaku/rvg/blob/master/RVG.md
[Unsigned LEB128]: https://en.wikipedia.org/wiki/LEB128#Unsigned_LEB128
[tool conventions]: https://github.com/WebAssembly/tool-conventions/blob/main/ProducersSection.md
