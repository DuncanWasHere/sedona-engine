use std::{fs, path::Path, process::Command};

pub fn write_token_stream_to_file(out_dir: &str, file_name: &str, buffer: &str) -> String {
    let dest_path = Path::new(&out_dir).join(file_name);
    fs::write(&dest_path, buffer)
        .unwrap_or_else(|_| panic!("Failed to write to file: {file_name}"));
    format_file(dest_path.to_str().unwrap());
    format!("/{file_name}")
}

pub fn format_file(file_name: &str) {
    Command::new("rustfmt")
        .arg(file_name)
        .output()
        .expect("Failed to execute rustfmt");
}
