use std::fs::File;
use tempfile::tempdir;
use assembler::is_valid_file;

#[test]
fn test_invalid_extension() {
    let dir = tempdir().unwrap();
    let file_path = dir.path().join("test.txt");
    File::create(&file_path).unwrap();

    assert!(!is_valid_file(file_path.to_str().unwrap()));
}

#[test]
fn test_valid_file_exists() {
    let dir = tempdir().unwrap();
    let file_path = dir.path().join("test_file.s");
    File::create(&file_path).unwrap();
    assert!(is_valid_file(file_path.to_str().unwrap()));
}

#[test]
fn test_file_does_not_exist() {
    let dir = tempdir().unwrap();
    let file_path = dir.path().join("nonexistent.s");
    assert!(!is_valid_file(file_path.to_str().unwrap()));
}
