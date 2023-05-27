use nucleide::{
    name::Name,
    parse::Reader,
    producers::{Producer, ProducerKind, VersionedSoftware},
    wasm::Read as _,
    Module,
};

fn producers<'a>(reader: &mut Reader<'a>) -> Option<Vec<Producer<'a>>> {
    (0..reader.integer()?)
        .map(|_| {
            let kind = match reader.name()? {
                "language" => ProducerKind::Language,
                "processed-by" => ProducerKind::ProcessedBy,
                "sdk" => ProducerKind::Sdk,
                _ => return None,
            };
            let software = (0..reader.integer()?)
                .map(|_| {
                    Some(VersionedSoftware {
                        name: reader.name()?,
                        version: reader.name()?,
                    })
                })
                .collect::<Option<_>>()?;

            Some(Producer {
                kind,
                list: software,
            })
        })
        .collect()
}

fn parse_usize(reader: &mut Reader<'_>) -> Option<usize> {
    Some(reader.integer()?.try_into().ok()?)
}

fn names<'a>(reader: &mut Reader<'a>) -> Option<Vec<Name<'a>>> {
    let mut names = Vec::new();

    // Get first byte, subsection kind
    let Some(mut subsection) = reader.u8() else {
        return Some(names);
    };

    // Get integer, length of subsection
    let mut len;

    if subsection == 0 {
        len = parse_usize(reader)?;
        names.push(Name::Module(reader.name()?));

        let Some(new_subsection) = reader.u8() else {
            return Some(names);
        };

        subsection = new_subsection;
    }

    if subsection == 1 {
        len = parse_usize(reader)?;
        names.push(Name::Function(reader.name_map()?));

        let Some(new_subsection) = reader.u8() else {
            return Some(names);
        };

        subsection = new_subsection;
    }

    if subsection == 2 {
        len = parse_usize(reader)?;
        names.push(Name::Local(reader.indirect_name_map()?));

        let Some(new_subsection) = reader.u8() else {
            return Some(names);
        };

        subsection = new_subsection;
    }

    if subsection == 3 {
        len = parse_usize(reader)?;
        names.push(Name::Label(reader.indirect_name_map()?));

        let Some(new_subsection) = reader.u8() else {
            return Some(names);
        };

        subsection = new_subsection;
    }

    if subsection == 4 {
        len = parse_usize(reader)?;
        names.push(Name::Type(reader.name_map()?));

        let Some(new_subsection) = reader.u8() else {
            return Some(names);
        };

        subsection = new_subsection;
    }

    if subsection == 5 {
        len = parse_usize(reader)?;
        names.push(Name::Table(reader.name_map()?));

        let Some(new_subsection) = reader.u8() else {
            return Some(names);
        };

        subsection = new_subsection;
    }

    if subsection == 6 {
        len = parse_usize(reader)?;
        names.push(Name::Memory(reader.name_map()?));

        let Some(new_subsection) = reader.u8() else {
            return Some(names);
        };

        subsection = new_subsection;
    }

    if subsection == 7 {
        len = parse_usize(reader)?;
        names.push(Name::Global(reader.name_map()?));

        let Some(new_subsection) = reader.u8() else {
            return Some(names);
        };

        subsection = new_subsection;
    }

    if subsection == 8 {
        len = parse_usize(reader)?;
        names.push(Name::Element(reader.name_map()?));

        let Some(new_subsection) = reader.u8() else {
            return Some(names);
        };

        subsection = new_subsection;
    }

    if subsection == 9 {
        len = parse_usize(reader)?;
        names.push(Name::Data(reader.name_map()?));

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
                            println!(" - Function");
                            for (id, name) in names {
                                println!("   - {id}; {name}");
                            }
                        }
                        Name::Global(names) => {
                            println!(" - Global");
                            for (id, name) in names {
                                println!("   - {id}; {name}");
                            }
                        }
                        Name::Data(names) => {
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
