use nucleide::{
    name::Name, parse::Reader, producers::Read as _, wasm::Read as _, Module,
};

fn names<'a>(reader: &mut Reader<'a>) -> Option<Vec<Name<'a>>> {
    let mut names = Vec::new();
    let mut subsection_min = 0;

    while reader.end().is_none() {
        let (subsection, mut reader) = reader.subsection()?;

        // Must be ordered correctly
        (subsection >= subsection_min).then_some(())?;
        names.push(match subsection {
            0 => Name::Module(reader.name()?),
            1 => Name::Function(reader.name_map()?),
            2 => Name::Local(reader.indirect_name_map()?),
            3 => Name::Label(reader.indirect_name_map()?),
            4 => Name::Type(reader.name_map()?),
            5 => Name::Table(reader.name_map()?),
            6 => Name::Memory(reader.name_map()?),
            7 => Name::Global(reader.name_map()?),
            8 => Name::Element(reader.name_map()?),
            9 => Name::Data(reader.name_map()?),
            _ => return None,
        });
        reader.end()?;
        subsection_min = subsection + 1;
    }

    Some(names)
}

fn main() {
    const BYTES: &[u8] = include_bytes!(
        "../hello_world/target/wasm32-unknown-unknown/debug/hello_world.wasm",
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
                let mut data = Reader::new(section.data.get(..).unwrap());
                for field in data.producers().expect("Failed to parse") {
                    println!(" - {field:?}");
                }
            }
            _ => println!("Skipping Unknown Custom Section: {}", section.name),
        }
    }
}
