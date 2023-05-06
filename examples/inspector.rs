use std::collections::HashMap;

use nucleide::{
    parse::{Reader, UInt, Writer},
    Module,
};

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
        let kind = match reader.name()? {
            "language" => ProducerKind::Language,
            "processed-by" => ProducerKind::ProcessedBy,
            "sdk" => ProducerKind::Sdk,
            _ => return None,
        };

        for _ in 0..reader.uleb128()? {
            let name = reader.name()?;
            let version = reader.name()?;

            software.push(VersionedSoftware { name, version });
        }

        producers.push(Producer {
            kind,
            list: software,
        })
    }

    Some(producers)
}

fn parse_name_map(reader: &mut Reader<'_>) -> Option<HashMap<u32, String>> {
    let mut name_map = HashMap::new();

    for _ in 0..reader.uleb128()? {
        name_map.insert(reader.uleb128()?, reader.name()?.to_string());
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
        names.push(Name::Module(reader.name()?.to_string()));

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
