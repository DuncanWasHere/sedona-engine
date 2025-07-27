use std::io::Write;
use std::path::Path;
use std::{env, fs};
use glob::glob;
use quote::quote;
use crate::generate::{generate_copy_traits, generate_default_queries, generate_queries, generate_systems, generate_world_rs};
use crate::parse::{collect_ecs_defs, CollectedData, EntityDefField};

/// Build the ECS from entity, component, and system defs matching the source glob pattern.
pub fn build_ecs(source_glob: &str) {
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR not set");
    let out_dir = env::var("OUT_DIR").expect("OUT_DIR not set");
    let pattern = format!("{manifest_dir}/{source_glob}");

    let mut include_files = Vec::new();
    let mut collected_data = CollectedData::default();

    for file in match glob(&pattern) {
        Ok(files) => files,
        Err(e) => panic!("Failed to glob {pattern}: {e}"),
    } {
        match file {
            Ok(path) => {
                if path.is_file() {
                    let path_str = path.display().to_string();
                    let collected = collect_ecs_defs(&path_str);
                    collected_data.entities.extend(collected.entities);
                    collected_data.queries.extend(collected.queries);
                    collected_data.systems.extend(collected.systems);
                }
            }
            Err(e) => eprintln!("Glob error: {e}"),
        }
    }

    collected_data.entities.iter_mut().for_each(|entity| {
        entity.fields.push(EntityDefField {
            name: "entity".into(),
            data_type: "Entity".into(),
            serialize: true,
        })
    });

    collected_data.retain_unique_queries();

    include_files.push(generate_default_queries(&out_dir));
    generate_world_rs(&out_dir, &mut include_files, &collected_data);
    generate_queries(&out_dir, &mut include_files, &collected_data);
    generate_systems(&out_dir, &mut include_files, &collected_data);
    generate_copy_traits(&out_dir, &mut include_files, &collected_data);

    let main_file = Path::new(&out_dir).join("sedona_ecs.rs");
    let mut output = match fs::File::create(main_file) {
        Ok(file) => file,
        Err(e) => panic!("Error writing ECS data to file: {e}"),
    };

    let mut include_rs = vec![];
    for file in include_files {
        include_rs.push(quote! {
            include!(concat!(env!("OUT_DIR"), #file));
        });
    }

    let sedona_ecs_rs = quote! {
        #(#include_rs)*
    };

    match write!(output, "{sedona_ecs_rs}") {
        Ok(_) => {}
        Err(e) => panic!("Error writing ECS data to file: {e}"),
    }
}
