use std::{fs::File, io, path::Path};

use rcedit::ResourceUpdater;
use tempfile::{Builder as TempFileBuilder, TempPath};

fn copy_test_binary() -> TempPath {
    let mut temp_file = TempFileBuilder::new().suffix(".exe").tempfile().unwrap();
    let mut original = File::open("tests/data/fake_resources_binary.exe").unwrap();
    io::copy(&mut original, &mut temp_file).unwrap();
    temp_file.into_temp_path()
}

fn patch(binary_path: &Path) {
    let mut updater = ResourceUpdater::new();
    updater.load(&binary_path).unwrap();
    updater.set_icon(Path::new("tests/data/new_icon.ico")).unwrap();
    updater.set_rcdata(102, Path::new("tests/data/new_rcdata.txt")).unwrap();
    updater.set_string(103, "Lorem ipsum").unwrap();
    updater.commit().unwrap();
}

#[test]
fn patching_test() {
    let binary_path = copy_test_binary();
    patch(&binary_path);
    let patched_file = File::open(&binary_path).unwrap();
    assert_eq!(patched_file.metadata().unwrap().len(), 12288);
}
