use nucleide::{
    daku::Nucleide, name::Name, producers::ProducerKind, Module, Section,
};

fn main() {
    const BYTES: &[u8] = include_bytes!(
        "../hello_world/target/wasm32-unknown-unknown/debug/hello_world.wasm",
    );

    for section in Module::new(BYTES).expect("Bad WASM file").custom_sections()
    {
        let name = section.name();

        // Try to downcast section from bytes to type-safe representation.
        let Some(section) = section.to() else {
            if name.starts_with(".debug_") {
                println!("Skipping DWARF Debug Data Section: {name}");
                continue;
            }

            // Either malformed or unknown
            println!("Didn't know how to parse section {name}");
            continue;
        };

        match section {
            // FIXME: Name section must appear before producers if exists
            Section::Name(names) => {
                println!("§ `name`");
                println!("========");
                for name in names {
                    match name {
                        Name::Module(name) => {
                            println!(" • Module {name:?}");
                        }
                        Name::Function(names) => {
                            println!(" • Function");
                            for (id, name) in names {
                                println!("   {id}. {name:?}");
                            }
                        }
                        Name::Global(names) => {
                            println!(" • Global");
                            for (id, name) in names {
                                println!("   {id}. {name:?}");
                            }
                        }
                        Name::Data(names) => {
                            println!(" • Data");
                            for (id, name) in names {
                                println!("   {id}. {name:?}");
                            }
                        }
                        Name::Local(names) => {
                            println!(" • Local");
                            for (id, name) in names {
                                println!("   {id}. {name:?}");
                            }
                        }
                        Name::Label(names) => {
                            println!(" • Label");
                            for (id, name) in names {
                                println!("   {id}. {name:?}");
                            }
                        }
                        Name::Type(names) => {
                            println!(" • Types");
                            for (id, name) in names {
                                println!("   {id}. {name:?}");
                            }
                        }
                        Name::Table(names) => {
                            println!(" • Table");
                            for (id, name) in names {
                                println!("   {id}. {name:?}");
                            }
                        }
                        Name::Memory(names) => {
                            println!(" • Memory");
                            for (id, name) in names {
                                println!("   {id}. {name:?}");
                            }
                        }
                        Name::Element(names) => {
                            println!(" • Element");
                            for (id, name) in names {
                                println!("   {id}. {name:?}");
                            }
                        }
                    }
                }
                println!();
            }
            Section::Producers(producers) => {
                println!("§ `producers`");
                println!("=============");

                for producer_group in producers {
                    let kind = match producer_group.kind {
                        ProducerKind::Language => "Language",
                        ProducerKind::ProcessedBy => "Processed By",
                        ProducerKind::Sdk => "SDK",
                    };

                    println!(" • {kind}:");

                    for producer in producer_group.list {
                        let name = producer.name;
                        let version = producer.version;

                        println!("   • Name: {name:?}, Version: {version:?}")
                    }
                }

                println!();
            }
            Section::Daku(daku) => {
                println!("§ `daku`");
                println!("========");
                println!(" • Portals:");

                for portal in daku.portals {
                    println!("   • {portal:?}");
                }

                let Some(nucleide) = daku.nucleide else {
                    println!();
                    continue;
                };

                println!(" • Nucleide Extension:");

                for subsection in nucleide {
                    match subsection {
                        Nucleide::LocalizedNames(locale_name_map) => {
                            println!(
                                "   • Localized Names: {locale_name_map:?}",
                            );
                        }
                        Nucleide::LocalizedDescriptions(locale_name_map) => {
                            println!(
                                "   • Localized Descriptions: \
                                 {locale_name_map:?}",
                            );
                        }
                        Nucleide::ThemedIcons(file_list) => {
                            println!("   • Themed Icons: {file_list:?}");
                        }
                        Nucleide::LocalizedAssets(locale_file_map) => {
                            println!(
                                "   • Localized Assets: {locale_file_map:?}",
                            );
                        }
                        Nucleide::Tags(tags) => {
                            println!("   • Tags: {tags:?}");
                        }
                        Nucleide::Categories(categories) => {
                            println!("   • Categories: {categories:?}");
                        }
                        Nucleide::Developer(developer) => {
                            println!("   • Developer: {developer:?}");
                        }
                    }
                }

                println!();
            }
            Section::Any { .. } => unreachable!(),
        }
    }
}
