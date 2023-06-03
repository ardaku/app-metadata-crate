use nucleide::{
    name::{Name, Read as _},
    parse::Reader,
    producers::Read as _,
    Module,
};

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
                // FIXME: Name section must appear before producers if exists
                println!("Name:");
                for name in Reader::new(&section.data[..])
                    .names()
                    .expect("Failed to parse")
                {
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
