use nucleide::parse::{Reader, UInt};

/// Writes to a buffer.
pub struct Writer(Vec<u8>);

impl Writer {
    pub fn new(buffer: Vec<u8>) -> Self {
        Self(buffer)
    }

    pub fn uleb128<T: UInt>(&mut self, value: T) {
        let mut remaining = value;

        while {
            let byte = remaining.little();

            remaining >>= 7;

            let more = remaining != T::ZERO;

            self.0.push(if more { byte | 0x80 } else { byte & !0x80 });

            more
        } {}
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

fn producers<'a>(reader: &mut Reader<'a>) -> Option<Vec<Producer<'a>>> {
    let mut producers = Vec::new();

    for _ in 0..reader.uleb128()? {
        let mut software = Vec::new();
        let len = reader.uleb128()?.try_into().ok()?;
        let kind = match reader.str(len)? {
            "language" => ProducerKind::Language,
            "processed-by" => ProducerKind::ProcessedBy,
            "sdk" => ProducerKind::Sdk,
            _ => return None,
        };

        for _ in 0..reader.uleb128()? {
            let len = reader.uleb128()?.try_into().ok()?;
            let name = reader.str(len)?;
            let len = reader.uleb128()?.try_into().ok()?;
            let version = reader.str(len)?;

            software.push(VersionedSoftware { name, version });
        }

        producers.push(Producer {
            kind,
            list: software,
        })
    }

    Some(producers)
}

fn parse_name(reader: &mut Reader<'_>) -> Option<String> {
    let len = reader.uleb128()?.try_into().ok()?;

    Some(reader.str(len)?.to_string())
}

fn parse_name_map(reader: &mut Reader<'_>) -> Option<HashMap<u32, String>> {
    let mut name_map = HashMap::new();

    for _ in 0..reader.uleb128()? {
        let idx = reader.uleb128()?;
        let name = parse_name(reader)?;

        name_map.insert(idx, name);
    }

    Some(name_map)
}

fn parse_usize(reader: &mut Reader<'_>) -> Option<usize> {
    Some(reader.uleb128()?.try_into().ok()?)
}

fn names(reader: &mut Reader<'_>) -> Option<Vec<Name>> {
    let mut names = Vec::new();

    // Get first byte, subsection kind
    let Some(mut subsection) = reader.u8() else {
        return Some(names);
    };

    // Get integer, length of subsection
    let mut len;

    if subsection == 0 {
        len = parse_usize(reader)?;
        let name = parse_name(reader)?;
        names.push(Name::Module(name));

        let Some(new_subsection) = reader.u8() else {
            return Some(names);
        };

        subsection = new_subsection;
    }

    if subsection == 1 {
        len = parse_usize(reader)?;
        let name_map = parse_name_map(reader)?;
        names.push(Name::Function(name_map));

        let Some(new_subsection) = reader.u8() else {
            return Some(names);
        };

        subsection = new_subsection;
    }

    if subsection == 2 {
        len = parse_usize(reader)?;
        // *data = data.get(len..)?; // FIXME
        names.push(Name::Local(HashMap::new()));

        let Some(new_subsection) = reader.u8() else {
            return Some(names);
        };

        subsection = new_subsection;
    }

    if subsection == 3 {
        len = parse_usize(reader)?;
        // *data = data.get(len..)?; // FIXME
        names.push(Name::Label(HashMap::new()));

        let Some(new_subsection) = reader.u8() else {
            return Some(names);
        };

        subsection = new_subsection;
    }

    if subsection == 4 {
        len = parse_usize(reader)?;
        let name_map = parse_name_map(reader)?;
        names.push(Name::Type(name_map));

        let Some(new_subsection) = reader.u8() else {
            return Some(names);
        };

        subsection = new_subsection;
    }

    if subsection == 5 {
        len = parse_usize(reader)?;
        let name_map = parse_name_map(reader)?;
        names.push(Name::Table(name_map));

        let Some(new_subsection) = reader.u8() else {
            return Some(names);
        };

        subsection = new_subsection;
    }

    if subsection == 6 {
        len = parse_usize(reader)?;
        let name_map = parse_name_map(reader)?;
        names.push(Name::Memory(name_map));

        let Some(new_subsection) = reader.u8() else {
            return Some(names);
        };

        subsection = new_subsection;
    }

    if subsection == 7 {
        len = parse_usize(reader)?;
        let name_map = parse_name_map(reader)?;
        names.push(Name::Global(name_map));

        let Some(new_subsection) = reader.u8() else {
            return Some(names);
        };

        subsection = new_subsection;
    }

    if subsection == 8 {
        len = parse_usize(reader)?;
        let name_map = parse_name_map(reader)?;
        names.push(Name::Element(name_map));

        let Some(new_subsection) = reader.u8() else {
            return Some(names);
        };

        subsection = new_subsection;
    }

    if subsection == 9 {
        len = parse_usize(reader)?;
        let name_map = parse_name_map(reader)?;
        names.push(Name::Data(name_map));

        let Some(_new_subsection) = reader.u8() else {
            return Some(names);
        };

        // subsection = new_subsection;
    }

    None
}

fn main() {
    // Test encode/decode
    let mut writer = Writer::new(Vec::new());
    for i in (0..=u32::from(u16::MAX))
        .chain((u32::MAX - u32::from(u16::MAX))..=u32::MAX)
    {
        writer.uleb128(i);
        assert!(writer.0.len() < 7);
        let mut reader = Reader::new(&writer.0[..]);
        let j = reader.uleb128().unwrap();
        assert_eq!(i, j);
        assert!(reader.end().is_some());
        writer.0.clear();
    }
    for i in
        (u64::from(u32::MAX) + 1)..(u64::from(u32::MAX) + u64::from(u16::MAX))
    {
        writer.uleb128(i);
        let mut reader = Reader::new(&writer.0[..]);
        let decoded = reader.uleb128();
        assert!(decoded.is_none(), "{i} decoded is {decoded:?}");
        writer.0.clear();
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
                let mut data = Reader::new(&section.data[..]);
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
                let mut data = Reader::new(&section.data[..]);
                for field in producers(&mut data).expect("Failed to parse") {
                    println!(" - {field:?}");
                }
            }
            _ => println!("Skipping Unknown Custom Section: {}", section.name),
        }
    }
}
