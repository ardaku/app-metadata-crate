use nucleide::parse::UInt;

/// Reads from a buffer.
pub struct Reader<'a>(&'a [u8]);

impl<'a> Reader<'a> {
    pub fn new(buffer: &'a [u8]) -> Self {
        Self(buffer)
    }

    pub fn uleb128_u8(&mut self) -> Option<u8> {
        decode_uleb128_u32(&mut self.0)?.try_into().ok()
    }

    pub fn uleb128_u16(&mut self) -> Option<u16> {
        decode_uleb128_u32(&mut self.0)?.try_into().ok()
    }

    pub fn uleb128_u32(&mut self) -> Option<u32> {
        decode_uleb128_u32(&mut self.0)?.try_into().ok()
    }

    pub fn uleb128_u64(&mut self) -> Option<u64> {
        decode_uleb128_u32(&mut self.0)?.try_into().ok()
    }

    pub fn uleb128_u128(&mut self) -> Option<u128> {
        decode_uleb128_u32(&mut self.0)?.try_into().ok()
    }
}

/// Writes to a buffer.
pub struct Writer(Vec<u8>);

impl Writer {
    pub fn new(buffer: Vec<u8>) -> Self {
        Self(buffer)
    }

    pub fn uleb128_u8(&mut self, value: u8) {
        encode_uleb128(&mut self.0, value);
    }

    pub fn uleb128_u16(&mut self, value: u16) {
        encode_uleb128(&mut self.0, value);
    }

    pub fn uleb128_u32(&mut self, value: u32) {
        encode_uleb128(&mut self.0, value);
    }

    pub fn uleb128_u64(&mut self, value: u64) {
        encode_uleb128(&mut self.0, value);
    }

    pub fn uleb128_u128(&mut self, value: u128) {
        encode_uleb128(&mut self.0, value);
    }
}

//

use std::collections::HashMap;

use nucleide::Module;

/// Versioned software name
#[derive(Debug)]
pub struct VersionedSoftware<'a> {
    /// Name of the program/application/tool
    pub name: &'a str,
    /// Version of the program/application/tool
    pub version: &'a str,
}

/// List from
/// <https://github.com/WebAssembly/tool-conventions/blob/main/ProducersSection.md>
#[derive(Debug)]
pub enum ProducerKind {
    /// Source language list
    Language,
    /// Individual tool list
    ProcessedBy,
    /// SDK list
    Sdk,
}

/// Producer Field
#[derive(Debug)]
pub struct Producer<'a> {
    /// Kind of the list
    pub kind: ProducerKind,
    /// List of versioned names
    pub list: Vec<VersionedSoftware<'a>>,
}

/// Name subsection
#[derive(Debug)]
pub enum Name {
    /// Module Name
    Module(String),
    /// Function Names
    Function(HashMap<u32, String>),
    /// Local Names Per Function
    Local(HashMap<u32, HashMap<u32, String>>),
    /// Ext: Goto/Loop Label Names Per Function
    Label(HashMap<u32, HashMap<u32, String>>),
    /// Ext: Type Names
    Type(HashMap<u32, String>),
    /// Ext: Table Names
    Table(HashMap<u32, String>),
    /// Ext: Memory Names
    Memory(HashMap<u32, String>),
    /// Ext: Global Names
    Global(HashMap<u32, String>),
    /// Ext: Element Names
    Element(HashMap<u32, String>),
    /// Ext: Data Names
    Data(HashMap<u32, String>),
}

fn encode_uleb128<T: UInt>(buffer: &mut Vec<u8>, value: T) {
    let mut remaining = value;

    while {
        let byte = remaining.little();

        remaining >>= 7;

        let more = remaining != T::ZERO;

        buffer.push(if more { byte | 0x80 } else { byte & !0x80 });

        more
    } {}
}

fn decode_uleb128_u32(data: &mut &[u8]) -> Option<u32> {
    let mut data = data;
    let mut value = 0;
    let mut shift = 0;

    while {
        let byte = data.first().cloned()?;

        *data = data.get(1..)?;
        value |= u32::from(byte & 0x7f) << shift;
        shift += 7;

        let more = shift < u32::BITS;
        let fits_u32 = more || byte < 16;

        if byte & 0x80 == 0 && fits_u32 {
            return Some(value);
        }

        more
    } {}

    None
}

fn producers<'a>(data: &mut &'a [u8]) -> Option<Vec<Producer<'a>>> {
    let mut producers = Vec::new();
    let len = decode_uleb128_u32(data)?;

    for _ in 0..len {
        let len = decode_uleb128_u32(data)?;
        let len = usize::try_from(len).ok()?;
        let field = std::str::from_utf8(data.get(..len)?).ok()?;
        *data = data.get(len..)?;

        //
        let kind = match field {
            "language" => ProducerKind::Language,
            "processed-by" => ProducerKind::ProcessedBy,
            "sdk" => ProducerKind::Sdk,
            _ => return None,
        };

        let len = decode_uleb128_u32(data)?;
        let len = usize::try_from(len).ok()?;

        let mut software = Vec::new();

        for _ in 0..len {
            let len = decode_uleb128_u32(data)?;
            let len = usize::try_from(len).ok()?;
            let name = std::str::from_utf8(data.get(..len)?).ok()?;
            *data = data.get(len..)?;

            let len = decode_uleb128_u32(data)?;
            let len = usize::try_from(len).ok()?;
            let version = std::str::from_utf8(data.get(..len)?).ok()?;
            *data = data.get(len..)?;

            software.push(VersionedSoftware { name, version });
        }

        producers.push(Producer {
            kind,
            list: software,
        })
    }

    Some(producers)
}

fn parse_name(data: &mut &[u8]) -> Option<String> {
    let len = decode_uleb128_u32(data)?;
    let len = usize::try_from(len).ok()?;
    let name = std::str::from_utf8(data.get(..len)?).ok()?.to_string();

    *data = data.get(len..)?;

    Some(name)
}

fn parse_name_map(data: &mut &[u8]) -> Option<HashMap<u32, String>> {
    let mut name_map = HashMap::new();
    let len = decode_uleb128_u32(data)?;

    for _ in 0..len {
        let idx = decode_uleb128_u32(data)?;
        let name = parse_name(data)?;
        name_map.insert(idx, name);
    }

    Some(name_map)
}

fn parse_usize(data: &mut &[u8]) -> Option<usize> {
    let len = decode_uleb128_u32(data)?;

    Some(usize::try_from(len).ok()?)
}

fn names(data: &mut &[u8]) -> Option<Vec<Name>> {
    let mut names = Vec::new();

    // Get first byte, subsection kind
    let Some(mut subsection) = data.first().cloned() else {
        return Some(names);
    };
    *data = data.get(1..)?;

    // Get integer, length of subsection
    let mut len;

    if subsection == 0 {
        len = parse_usize(data)?;
        let name = parse_name(data)?;
        names.push(Name::Module(name));

        let Some(new_subsection) = data.first().cloned() else {
            return Some(names);
        };

        subsection = new_subsection;
        *data = data.get(1..)?;
    }

    if subsection == 1 {
        len = parse_usize(data)?;
        let old_data = *data;
        let name_map = parse_name_map(data)?;
        names.push(Name::Function(name_map));

        let Some(new_subsection) = data.first().cloned() else {
            return Some(names);
        };

        subsection = new_subsection;
        *data = data.get(1..)?;
    }

    if subsection == 2 {
        len = parse_usize(data)?;
        *data = data.get(len..)?;
        names.push(Name::Local(HashMap::new()));

        let Some(new_subsection) = data.first().cloned() else {
            return Some(names);
        };

        subsection = new_subsection;
        *data = data.get(1..)?;
    }

    if subsection == 3 {
        len = parse_usize(data)?;
        *data = data.get(len..)?;
        names.push(Name::Label(HashMap::new()));

        let Some(new_subsection) = data.first().cloned() else {
            return Some(names);
        };

        subsection = new_subsection;
        *data = data.get(1..)?;
    }

    if subsection == 4 {
        len = parse_usize(data)?;
        let name_map = parse_name_map(data)?;
        names.push(Name::Type(name_map));

        let Some(new_subsection) = data.first().cloned() else {
            return Some(names);
        };

        subsection = new_subsection;
        *data = data.get(1..)?;
    }

    if subsection == 5 {
        len = parse_usize(data)?;
        let name_map = parse_name_map(data)?;
        names.push(Name::Table(name_map));

        let Some(new_subsection) = data.first().cloned() else {
            return Some(names);
        };

        subsection = new_subsection;
        *data = data.get(1..)?;
    }

    if subsection == 6 {
        len = parse_usize(data)?;
        let name_map = parse_name_map(data)?;
        names.push(Name::Memory(name_map));

        let Some(new_subsection) = data.first().cloned() else {
            return Some(names);
        };

        subsection = new_subsection;
        *data = data.get(1..)?;
    }

    if subsection == 7 {
        len = parse_usize(data)?;
        let name_map = parse_name_map(data)?;
        names.push(Name::Global(name_map));

        let Some(new_subsection) = data.first().cloned() else {
            return Some(names);
        };

        subsection = new_subsection;
        *data = data.get(1..)?;
    }

    if subsection == 8 {
        len = parse_usize(data)?;
        let name_map = parse_name_map(data)?;
        names.push(Name::Element(name_map));

        let Some(new_subsection) = data.first().cloned() else {
            return Some(names);
        };

        subsection = new_subsection;
        *data = data.get(1..)?;
    }

    if subsection == 9 {
        len = parse_usize(data)?;
        let name_map = parse_name_map(data)?;
        names.push(Name::Data(name_map));

        let Some(_new_subsection) = data.first().cloned() else {
            return Some(names);
        };

        // subsection = new_subsection;
        // data = data.get(1..)?;
    }

    None
}

fn main() {
    // Test encode/decode
    let mut buf = Vec::new();
    for i in (0..=u32::from(u16::MAX))
        .chain((u32::MAX - u32::from(u16::MAX))..=u32::MAX)
    {
        encode_uleb128(&mut buf, i);
        assert!(buf.len() < 7);
        let mut empty = &buf[..];
        let j = decode_uleb128_u32(&mut empty).unwrap();
        assert_eq!(i, j);
        assert!(empty.is_empty());
        buf.clear();
    }
    for i in
        (u64::from(u32::MAX) + 1)..(u64::from(u32::MAX) + u64::from(u16::MAX))
    {
        encode_uleb128(&mut buf, i);
        let decoded = decode_uleb128_u32(&mut &buf[..]);
        assert!(decoded.is_none(), "{i} decoded is {decoded:?}");
        buf.clear();
    }

    // Example:

    const BYTES: &[u8] = include_bytes!(
        "../hello_world/target/wasm32-unknown-unknown/debug/hello_world.wasm"
    );

    for section in Module::new(BYTES).expect("Bad WASM file").custom_sections()
    {
        match &*section.name {
            name if name.starts_with(".debug_") => {
                println!("Skipping DWARF Debug Data Section: {name}")
            }
            "name" => {
                println!("Name:");
                let mut data = &section.data[..];
                for name in names(&mut data).expect("Failed to parse") {
                    match name {
                        Name::Module(name) => {
                            println!(" - Module {name:?}");
                        }
                        Name::Function(names) => {
                            let mut names = names.iter().collect::<Vec<_>>();
                            names.sort();
                            println!(" - Function");
                            for (id, name) in names {
                                println!("   - {id}; {name}");
                            }
                        }
                        Name::Global(names) => {
                            let mut names = names.iter().collect::<Vec<_>>();
                            names.sort();
                            println!(" - Global");
                            for (id, name) in names {
                                println!("   - {id}; {name}");
                            }
                        }
                        Name::Data(names) => {
                            let mut names = names.iter().collect::<Vec<_>>();
                            names.sort();
                            println!(" - Data");
                            for (id, name) in names {
                                println!("   - {id}; {name}");
                            }
                        }
                        _ => println!(" - {name:?}"),
                    }
                }
            }
            "producers" => {
                // FIXME: Must appear after name section
                println!("Producers");
                let mut data = &section.data[..];
                for field in producers(&mut data).expect("Failed to parse")
                {
                    println!(" - {field:?}");
                }
            }
            _ => println!("Skipping Unknown Custom Section: {}", section.name),
        }
    }
}
