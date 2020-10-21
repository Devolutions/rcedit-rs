use cc::Build;

const SOURCE_FILES: [&str; 2] = ["src/rescle.cc", "src/librcedit.cpp"];
const HEADER_FILES: [&str; 1] = ["src/rescle.h"];

fn track_file_changes(file: &str) {
    println!("cargo:rerun-if-changed={}", file);
}

fn main() {
    SOURCE_FILES.iter().copied().for_each(track_file_changes);
    HEADER_FILES.iter().copied().for_each(track_file_changes);

    let current_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();

    Build::new()
        .cpp(true)
        .static_crt(true)
        .flag("-std=c++11")
        .files(SOURCE_FILES.iter().map(|name| format!("{}/{}", current_dir, name)))
        .compile("rcedit");
}
