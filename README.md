# Nucleide
A crate to manipulate [custom sections] of a WebAssembly module to view/edit
application metadata.

# Specification
Nucleide specifies WASM metadata that is read by the Nucleic Desktop
Environment.  It includes the WebAssembly 2.0 and the Daku 1.0.0-beta.0
specifications.

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
 - `size: u32`: Number of bytes

#### 0 => Module Name
 - `size: u32`: Number of UTF-8 bytes in `data`
 - `data: [u8]`: UTF-8 bytes describing name of module

#### 1 => Function Names
 - `size: u32`: Number of mappings
 - `data: [_]`: List of mappings (listed below)

---

 - `index: u32`: Function index
 - `size: u32`: Number of UTF-8 bytes in `data`
 - `data: [u8]`: UTF-8 bytes describing name of function

#### 2 => Local Names
 - `size: u32`: Number of mappings
 - `data: [_]`: List of mappings (listed below)

---

 - `index: u32`: Function index
 - `size: u32`: Number of mappings
 - `data: [_]`: List of mappings (listed below)

---

 - `index: u32`: Local index
 - `size: u32`: Number of UTF-8 bytes in `data`
 - `data: [u8]`: UTF-8 bytes describing name of function

### Daku (`daku`)
Daku programs are a WebAssembly module that must have the `daku` custom section,
are compressed with ZStd, and should use the `.daku` file extension.

 - `size: u32`: Number of portals
 - `data: [u32]`: List of portal IDs

### Nucleide (`app`)
 - `subsection: u8`: Each subsection is optional, and must be placed in this
   order:
   - 0 => App Name (UTF-8)
   - 1 => Localized App Name (Localization map, see below)
   - 2 => App Icon Full Color (Concatenated list of [RVG] or [QOI] at different
     resolutions)
   - 3 => App Icon Reduced Colors (Concatenated list of [RVG] or [QOI] at
     different resolutions)
 - `size: u32`: Number of bytes

#### Map
 - `size: u32`: Number of mappings
 - `data: [_]`: List of mappings (listed below)

---

 - `u16`: 2-letter language code (Example: "en")
 - `u16`: 2-letter region code (Example: "US")
 - `size: u32`: Number of UTF-8 bytes in `data`
 - `data: [u8]`: UTF-8 text

[custom sections]: https://webassembly.github.io/spec/core/binary/modules.html#index-2
[the wasm spec]: https://webassembly.github.io/spec/core/appendix/custom.html#name-section
[QOI]: https://qoiformat.org/qoi-specification.pdf
[RVG]: https://github.com/ardaku/rvg/blob/master/RVG.md
