use std::{env, path::PathBuf};

fn main() {
    let root_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let output_file = PathBuf::from(&root_dir)
        .join("include")
        .join("audio_engine")
        .join("audio_engine.h")
        .display()
        .to_string();

    let config = cbindgen::Config {
        pragma_once: true,
        namespace: Some("audio_engine".to_owned()),
        braces: cbindgen::Braces::NextLine,
        line_length: 80,
        tab_width: 4,
        ..Default::default()
    };

    cbindgen::generate_with_config(&root_dir, config)
        .unwrap()
        .write_to_file(&output_file);
}
