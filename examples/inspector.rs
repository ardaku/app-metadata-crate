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

fn encode_uleb128_u64(buffer: &mut Vec<u8>, value: u64) {
    let mut remaining = value;

    while {
        let [byte, _, _, _, _, _, _, _] = remaining.to_le_bytes();

        remaining >>= 7;

        let more = remaining != 0;

        buffer.push(if more { byte | 0x80 } else { byte & !0x80 });

        more
    } {}
}

fn encode_uleb128_u32(buffer: &mut Vec<u8>, value: u32) {
    let mut remaining = value;

    while {
        let [byte, _, _, _] = remaining.to_le_bytes();

        remaining >>= 7;

        let more = remaining != 0;

        buffer.push(if more { byte | 0x80 } else { byte & !0x80 });

        more
    } {}
}

fn decode_uleb128_u32(data: &[u8]) -> Option<(&[u8], u32)> {
    let mut data = data;
    let mut value = 0;
    let mut shift = 0;

    while {
        let byte = data.first().cloned()?;

        data = data.get(1..)?;
        value |= u32::from(byte & 0x7f) << shift;
        shift += 7;

        let more = shift < 32;
        let fits_u32 = more || byte < 16;

        if byte & 0x80 == 0 && fits_u32 {
            return Some((data, value));
        }

        more
    } {}

    None
}

fn producers(data: &[u8]) -> Option<Vec<Producer>> {
    let mut producers = Vec::new();

    let (mut data, len) = decode_uleb128_u32(data)?;

    for _ in 0..len {
        let len;
        (data, len) = decode_uleb128_u32(data)?;
        let len = usize::try_from(len).ok()?;
        let field = std::str::from_utf8(data.get(..len)?).ok()?;
        data = data.get(len..)?;

        //
        let kind = match field {
            "language" => ProducerKind::Language,
            "processed-by" => ProducerKind::ProcessedBy,
            "sdk" => ProducerKind::Sdk,
            _ => return None,
        };

        let len;
        (data, len) = decode_uleb128_u32(data)?;
        let len = usize::try_from(len).ok()?;

        let mut software = Vec::new();

        for _ in 0..len {
            let len;
            (data, len) = decode_uleb128_u32(data)?;
            let len = usize::try_from(len).ok()?;
            let name = std::str::from_utf8(data.get(..len)?).ok()?;
            data = data.get(len..)?;

            let len;
            (data, len) = decode_uleb128_u32(data)?;
            let len = usize::try_from(len).ok()?;
            let version = std::str::from_utf8(data.get(..len)?).ok()?;
            data = data.get(len..)?;

            software.push(VersionedSoftware { name, version });
        }

        producers.push(Producer {
            kind,
            list: software,
        })
    }

    Some(producers)
}

fn parse_name(data: &[u8]) -> Option<(&[u8], String)> {
    let (data, len) = decode_uleb128_u32(data)?;
    let len = usize::try_from(len).ok()?;
    let name = std::str::from_utf8(data.get(..len)?).ok()?.to_string();

    Some((&data[len..], name))
}

fn parse_name_map(data: &[u8]) -> Option<(&[u8], HashMap<u32, String>)> {
    let mut name_map = HashMap::new();
    let (mut data, len) = decode_uleb128_u32(data)?;

    for _ in 0..len {
        let (idx, name);
        (data, idx) = decode_uleb128_u32(data)?;
        (data, name) = parse_name(data)?;
        name_map.insert(idx, name);
    }

    Some((data, name_map))
}

fn parse_usize(data: &[u8]) -> Option<(&[u8], usize)> {
    let (data, len) = decode_uleb128_u32(data)?;
    
    Some((data, usize::try_from(len).ok()?))
}

fn names(data: &[u8]) -> Option<Vec<Name>> {
    let mut names = Vec::new();

    // Get first byte, subsection kind
    let Some(mut subsection) = data.first().cloned() else {
        return Some(names);
    };
    let mut data = data.get(1..)?;

    // Get integer, length of subsection
    let mut len;

    if subsection == 0 {
        (data, len) = parse_usize(data)?;
        let name;
        (data, name) = parse_name(data)?;
        names.push(Name::Module(name));

        let Some(new_subsection) = data.first().cloned() else {
            return Some(names);
        };

        subsection = new_subsection;
        data = data.get(1..)?;
    }

    if subsection == 1 {
        (data, len) = parse_usize(data)?;
        let name_map;
        (data, name_map) = parse_name_map(data)?;
        names.push(Name::Function(name_map));

        let Some(new_subsection) = data.first().cloned() else {
            return Some(names);
        };

        subsection = new_subsection;
        data = data.get(1..)?;
    }
        
    if subsection == 2 {
        (data, len) = parse_usize(data)?;
        data = data.get(len..)?;
        names.push(Name::Local(HashMap::new()));

        let Some(new_subsection) = data.first().cloned() else {
            return Some(names);
        };

        subsection = new_subsection;
        data = data.get(1..)?;
    }

    if subsection == 3 {
        (data, len) = parse_usize(data)?;
        data = data.get(len..)?;
        names.push(Name::Label(HashMap::new()));

        let Some(new_subsection) = data.first().cloned() else {
            return Some(names);
        };

        subsection = new_subsection;
        data = data.get(1..)?;
    }

    if subsection == 4 {
        (data, len) = parse_usize(data)?;
        let name_map;
        (data, name_map) = parse_name_map(data)?;
        names.push(Name::Type(name_map));

        let Some(new_subsection) = data.first().cloned() else {
            return Some(names);
        };

        subsection = new_subsection;
        data = data.get(1..)?;
    }

    if subsection == 5 {
        (data, len) = parse_usize(data)?;
        let name_map;
        (data, name_map) = parse_name_map(data)?;
        names.push(Name::Table(name_map));

        let Some(new_subsection) = data.first().cloned() else {
            return Some(names);
        };

        subsection = new_subsection;
        data = data.get(1..)?;
    }

    if subsection == 6 {
        (data, len) = parse_usize(data)?;
        let name_map;
        (data, name_map) = parse_name_map(data)?;
        names.push(Name::Memory(name_map));

        let Some(new_subsection) = data.first().cloned() else {
            return Some(names);
        };

        subsection = new_subsection;
        data = data.get(1..)?;
    }

    if subsection == 7 {
        (data, len) = parse_usize(data)?;
        let name_map;
        (data, name_map) = parse_name_map(data)?;
        names.push(Name::Global(name_map));

        let Some(new_subsection) = data.first().cloned() else {
            return Some(names);
        };

        subsection = new_subsection;
        data = data.get(1..)?;
    }

    if subsection == 8 {
        (data, len) = parse_usize(data)?;
        let name_map;
        (data, name_map) = parse_name_map(data)?;
        names.push(Name::Element(name_map));

        let Some(new_subsection) = data.first().cloned() else {
            return Some(names);
        };

        subsection = new_subsection;
        data = data.get(1..)?;
    }

    if subsection == 9 {
        (data, len) = parse_usize(data)?;
        let name_map;
        (data, name_map) = parse_name_map(data)?;
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
        encode_uleb128_u32(&mut buf, i);
        let (empty, j) = decode_uleb128_u32(&buf).unwrap();
        assert_eq!(i, j);
        assert!(empty.is_empty());
        assert!(buf.len() < 7);
        buf.clear();
    }
    for i in
        (u64::from(u32::MAX) + 1)..(u64::from(u32::MAX) + u64::from(u16::MAX))
    {
        encode_uleb128_u64(&mut buf, i);
        let decoded = decode_uleb128_u32(&buf);
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
                for name in names(&section.data).expect("Failed to parse") {
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
                for field in producers(&section.data).expect("Failed to parse")
                {
                    println!(" - {field:?}");
                }
            }
            _ => println!("Skipping Unknown Custom Section: {}", section.name),
        }
    }
}
